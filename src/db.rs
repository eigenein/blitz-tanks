use std::{collections::HashMap, path::PathBuf};

use prost::Message;
use scru128::Scru128Id;
use sled::Tree;
use tracing::{info, instrument};

use crate::{
    models::{User, VehicleDescription},
    prelude::*,
};

/// Convenience wrapper around the database.
#[derive(Clone)]
pub struct Db(sled::Db);

impl From<sled::Db> for Db {
    fn from(db: sled::Db) -> Self {
        Self(db)
    }
}

impl Db {
    #[instrument(skip_all, fields(?path))]
    pub fn open(path: &PathBuf) -> Result<Self> {
        sled::open(path)
            .with_context(|| format!("failed to open the database from `{path:?}`"))
            .map(Into::into)
    }

    /// Open a temporary database for unit testing.
    #[cfg(test)]
    pub fn open_temporary() -> Result<Self> {
        sled::Config::default()
            .temporary(true)
            .open()
            .context("failed to open a temporary database")
            .map(Into::into)
    }

    pub fn session_manager(&self) -> Result<SessionManager> {
        Ok(SessionManager::from(self.open_tree("sessions")?))
    }

    pub fn tankopedia_manager(&self) -> Result<TankopediaManager> {
        Ok(TankopediaManager::from(self.open_tree("tankopedia")?))
    }

    fn open_tree(&self, name: &str) -> Result<Tree> {
        self.0.open_tree(name).with_context(|| format!("failed to open tree `{name}`"))
    }
}

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct SessionManager(Tree);

impl From<Tree> for SessionManager {
    #[inline]
    fn from(tree: Tree) -> Self {
        Self(tree)
    }
}

impl SessionManager {
    /// Insert the user to the session tree.
    #[instrument(skip_all, fields(session_id = %session_id))]
    pub fn insert(&self, session_id: Scru128Id, user: &User) -> Result {
        self.0
            .insert(session_id.to_bytes(), user.encode_to_vec())
            .with_context(|| format!("failed to insert the session {session_id:?}"))?;
        Ok(())
    }

    #[cfg(test)]
    pub fn insert_test_session(&self) -> Result<Scru128Id> {
        use crate::models::new_session_id;

        let session_id = new_session_id();
        self.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() + 10,
                account_id: 0,
                nickname: "test".to_string(),
            },
        )?;
        Ok(session_id)
    }

    /// Retrieve a user from the session tree.
    #[instrument(skip_all, fields(session_id = %session_id))]
    pub fn get(&self, session_id: Scru128Id) -> Result<Option<User>> {
        let serialized_user = self
            .0
            .get(session_id.to_bytes())
            .with_context(|| format!("failed to retrieve session {session_id}"))?;
        let Some(serialized_user) = serialized_user else { return Ok(None) };
        let session = User::decode(serialized_user.as_ref())
            .with_context(|| format!("failed to deserialize session {session_id}"))?;
        Ok((session.expires_at > Utc::now().timestamp()).then_some(session))
    }
}

pub struct TankopediaManager(Tree);

impl From<Tree> for TankopediaManager {
    #[inline]
    fn from(tree: Tree) -> Self {
        Self(tree)
    }
}

impl TankopediaManager {
    /// Update the tankopedia database: insert new vehicles and update existing ones.
    pub fn update(&self, vehicles: Vec<VehicleDescription>) -> Result<&Self> {
        info!(n_vehicles = vehicles.len(), "📥 Updating the tankopedia…");
        for vehicle in vehicles {
            self.insert(&vehicle)?;
        }
        Ok(self)
    }

    /// Insert the vehicles, which Wargaming.net is too lazy to add to the tankopedia.
    pub fn prepopulate(&self) -> Result<&Self> {
        info!("🤬 Pre-populating the tankopedia…");
        self.insert(&VehicleDescription::new(9777, "WZ-114").premium())?;
        self.insert(&VehicleDescription::new(18241, "B-C Bourrasque").premium())?;
        self.insert(&VehicleDescription::new(12417, "Bisonte C45").premium())?;
        self.insert(&VehicleDescription::new(10545, "Wind").premium())?;
        self.insert(&VehicleDescription::new(24849, "Kryos").premium())?;
        self.insert(&VehicleDescription::new(20817, "Explorer").premium())?;
        self.insert(&VehicleDescription::new(1329, "Renault NC-31"))?;
        self.insert(&VehicleDescription::new(81, "Vickers Medium Mk. I").premium())?;
        self.insert(&VehicleDescription::new(3089, "Leichttraktor"))?;
        self.insert(&VehicleDescription::new(577, "Renault FT").premium())?;
        self.insert(&VehicleDescription::new(609, "R. Otsu"))?;
        self.insert(&VehicleDescription::new(545, "T1 Cunningham").premium())?;
        self.insert(&VehicleDescription::new(64081, "Mk I* Heavy Tank").premium())?;
        Ok(self)
    }

    /// Load the tankopedia into a hashmap.
    pub fn load(&self) -> Result<HashMap<u16, VehicleDescription>> {
        info!("📤 Loading the tankopedia…");
        let tankopedia = self
            .0
            .iter()
            .map(|result| {
                let (key, value) = result?;
                Ok((
                    u16::from_be_bytes(key.as_ref().try_into()?),
                    VehicleDescription::decode(value.as_ref())?,
                ))
            })
            .collect::<Result<HashMap<u16, VehicleDescription>>>()
            .context("failed to load the tankopedia")?;
        info!(n_vehicles = tankopedia.len(), "✅ Loaded the tankopedia");
        Ok(tankopedia)
    }

    fn insert(&self, vehicle: &VehicleDescription) -> Result {
        self.0.insert((vehicle.tank_id as u16).to_be_bytes(), vehicle.encode_to_vec())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::Db, models::new_session_id};

    #[test]
    fn unknown_session_ok() -> Result {
        let session = Db::open_temporary()?.session_manager()?.get(new_session_id())?;
        assert!(session.is_none());
        Ok(())
    }

    #[test]
    fn known_session_ok() -> Result {
        let manager = Db::open_temporary()?.session_manager()?;
        let session_id = new_session_id();
        manager.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() + 10,
                account_id: 0,
                nickname: "test".to_string(),
            },
        )?;
        let user = manager.get(session_id)?;
        assert!(user.is_some());
        Ok(())
    }

    #[test]
    fn expired_session_ok() -> Result {
        let manager = Db::open_temporary()?.session_manager()?;
        let session_id = new_session_id();
        manager.insert(
            session_id,
            &User {
                access_token: "test".to_string(),
                expires_at: Utc::now().timestamp() - 10,
                account_id: 0,
                nickname: "test".to_string(),
            },
        )?;
        let user = manager.get(session_id)?;
        assert!(user.is_none(), "actual user: {user:?}");
        Ok(())
    }
}
