use std::collections::HashMap;

use mongodb::{Client, Collection, Database};
use prost::Message;
use scru128::Scru128Id;
use sled::Tree;
use tracing::{info, instrument};
use url::Url;

use crate::{
    models::{User, VehicleDescription, Vote},
    prelude::*,
};

/// Convenience wrapper around the database.
#[derive(Clone)]
pub struct Db {
    legacy_db: sled::Db,
    db: Database,
}

impl Db {
    pub const fn new(legacy_db: sled::Db, db: Database) -> Self {
        Self { legacy_db, db }
    }

    /// Open a temporary database for unit testing.
    #[cfg(test)]
    pub async fn open_temporary() -> Result<Self> {
        let legacy_db = sled::Config::default()
            .temporary(true)
            .open()
            .context("failed to open a temporary database")?;
        let db = Client::with_uri_str("mongodb://localhost")
            .await
            .context("failed to connect to MongoDB")?
            .database(&scru128::new_string());
        Ok(Self::new(legacy_db, db))
    }

    #[inline]
    pub fn session_manager(&self) -> Result<SessionManager> {
        self.open_manager("sessions")
    }

    #[inline]
    pub fn tankopedia_manager(&self) -> Result<TankopediaManager> {
        self.open_manager("tankopedia")
    }

    #[inline]
    pub fn vote_manager(&self) -> Result<VoteManager> {
        self.open_manager("ratings")
    }

    #[inline]
    pub fn open_manager<D, T: From<(Tree, Collection<D>)>>(&self, name: &str) -> Result<T> {
        let tree = self
            .legacy_db
            .open_tree(name)
            .with_context(|| format!("failed to open tree `{name}`"))?;
        let collection = self.db.collection(name);
        Ok(T::from((tree, collection)))
    }
}

/// Wrapper around the tree to manage client-side sessions.
#[derive(Clone)]
pub struct SessionManager(Tree);

impl From<(Tree, Collection<User>)> for SessionManager {
    fn from((tree, _collection): (Tree, Collection<User>)) -> Self {
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

impl From<(Tree, Collection<VehicleDescription>)> for TankopediaManager {
    fn from((tree, _collection): (Tree, Collection<VehicleDescription>)) -> Self {
        Self(tree)
    }
}

impl TankopediaManager {
    /// Update the tankopedia database: insert new vehicles and update existing ones.
    pub fn update(&self, vehicles: Vec<VehicleDescription>) -> Result<&Self> {
        info!(n_vehicles = vehicles.len(), "📥 Updating the tankopedia…");
        for mut vehicle in vehicles {
            Self::fix_scheme(&mut vehicle)?;
            self.insert_vehicle(&vehicle)?;
        }
        Ok(self)
    }

    /// Insert the vehicles, which Wargaming.net is too lazy to add to the tankopedia.
    pub fn prepopulate(&self) -> Result<&Self> {
        info!("🤬 Pre-populating the tankopedia…");
        self.insert_unknown(9777, "WZ-114", true)?;
        self.insert_unknown(18241, "B-C Bourrasque", true)?;
        self.insert_unknown(12417, "Bisonte C45", true)?;
        self.insert_unknown(10545, "Wind", true)?;
        self.insert_unknown(24849, "Kryos", true)?;
        self.insert_unknown(20817, "Explorer", true)?;
        self.insert_unknown(1329, "Renault NC-31", false)?;
        self.insert_unknown(81, "Vickers Medium Mk. I", true)?;
        self.insert_unknown(3089, "Leichttraktor", true)?;
        self.insert_unknown(577, "Renault FT", true)?;
        self.insert_unknown(609, "R. Otsu", false)?;
        self.insert_unknown(545, "T1 Cunningham", true)?;
        self.insert_unknown(64081, "Mk I* Heavy Tank", true)?;
        self.insert_unknown(12673, "Bofors Tornvagn", true)?;
        self.insert_unknown(27425, "TL-7-120", true)?;
        self.insert_unknown(13441, "Aeonix", true)?;
        self.insert_unknown(25857, "Object 777 Version Ⅱ", true)?;
        self.insert_unknown(10609, "Magnate", true)?;
        self.insert_unknown(19777, "AltProto AMX 30", true)?;
        self.insert_unknown(26129, "Epsilon", true)?;
        self.insert_unknown(23297, "Object 244", true)?;
        self.insert_unknown(22353, "Churchill W", true)?;
        self.insert_unknown(20289, "Pirate", true)?;
        self.insert_unknown(10801, "Panlong", true)?;
        self.insert_unknown(10289, "WZ-132-1", false)?;
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

    fn insert_vehicle(&self, vehicle: &VehicleDescription) -> Result {
        self.0.insert((vehicle.tank_id as u16).to_be_bytes(), vehicle.encode_to_vec())?;
        Ok(())
    }

    fn insert_unknown(&self, tank_id: u16, name: &str, is_premium: bool) -> Result {
        self.insert_vehicle(&VehicleDescription {
            tank_id: tank_id as u32,
            name: name.to_string(),
            images: Default::default(),
            is_premium,
        })
    }

    /// Wargaming is too lazy to use HTTPS either.
    fn fix_scheme(vehicle: &mut VehicleDescription) -> Result {
        if let Some(url) = &vehicle.images.normal_url {
            let mut url = Url::parse(url)?;
            url.set_scheme("https")
                .map_err(|_| anyhow!("failed to update scheme for #{}", vehicle.tank_id))?;
            vehicle.images.normal_url = Some(url.to_string());
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct VoteManager(Tree);

impl From<(Tree, Collection<Vote>)> for VoteManager {
    fn from((tree, _collection): (Tree, Collection<Vote>)) -> Self {
        Self(tree)
    }
}

impl VoteManager {
    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub fn insert(&self, account_id: u32, tank_id: u16, vote: &Vote) -> Result {
        self.0
            .insert(Self::encode_key(account_id, tank_id), vote.encode_to_vec())
            .with_context(|| format!("failed to insert the #{account_id}'s vote for #{tank_id}"))?;
        Ok(())
    }

    /// Retrieve a single vote.
    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub fn get(&self, account_id: u32, tank_id: u16) -> Result<Option<Vote>> {
        self.0
            .get(Self::encode_key(account_id, tank_id))?
            .map(|value| Vote::decode(value.as_ref()))
            .transpose()
            .with_context(|| format!("failed to retrieve a #{account_id}'s vote for #{tank_id}"))
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub fn delete(&self, account_id: u32, tank_id: u16) -> Result {
        self.0.remove(Self::encode_key(account_id, tank_id))?;
        Ok(())
    }

    /// Retrieve all votes of the user.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub fn get_all_by_account_id(&self, account_id: u32) -> Result<Vec<(u16, Vote)>> {
        self.0
            .scan_prefix(account_id.to_be_bytes())
            .map(|result| {
                let (key, value) = result?;
                let tank_id = Self::decode_tank_id(key.as_ref())?;
                let event = Vote::decode(value.as_ref())?;
                Ok((tank_id, event))
            })
            .collect()
    }

    /// Iterate over **all** the votes.
    pub fn iter_all(&self) -> impl Iterator<Item = Result<(u32, u16, Vote)>> {
        self.0.iter().map(|result| {
            let (key, value) = result?;
            let (account_id, tank_id) = Self::decode_key(key.as_ref())?;
            Ok((account_id, tank_id, Vote::decode(value.as_ref())?))
        })
    }

    /// Encode the key corresponding to the user's vehicle.
    ///
    /// # Considerations
    ///
    /// 1. Key must be sortable, hence the big-endian encoding.
    /// 2. I should be able to retrieve all user's votes in one go, hence keys start with account ID.
    /// 3. I should be able to retrieve individual votes, hence the key contains tank ID as well.
    #[inline]
    fn encode_key(account_id: u32, tank_id: u16) -> Vec<u8> {
        [&account_id.to_be_bytes()[..], &tank_id.to_be_bytes()[..]].concat()
    }

    /// Decode tank ID from the Sled key.
    #[inline]
    fn decode_tank_id(key: &[u8]) -> Result<u16> {
        Ok(u16::from_be_bytes((&key[4..6]).try_into()?))
    }

    #[inline]
    fn decode_key(key: &[u8]) -> Result<(u32, u16)> {
        let account_id = u32::from_be_bytes((&key[0..4]).try_into()?);
        Ok((account_id, Self::decode_tank_id(key)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::Db,
        models::{new_session_id, Rating},
    };

    #[tokio::test]
    #[ignore]
    async fn unknown_session_ok() -> Result {
        let session = Db::open_temporary().await?.session_manager()?.get(new_session_id())?;
        assert!(session.is_none());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn known_session_ok() -> Result {
        let manager = Db::open_temporary().await?.session_manager()?;
        let session_id = manager.insert_test_session()?;
        let user = manager.get(session_id)?;
        assert!(user.is_some());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn expired_session_ok() -> Result {
        let manager = Db::open_temporary().await?.session_manager()?;
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

    #[tokio::test]
    #[ignore]
    async fn insert_get_vote_ok() -> Result {
        let manager = Db::open_temporary().await?.vote_manager()?;
        manager.insert(1, 42, &Vote::new_now(Rating::Like))?;
        assert!(manager.get(1, 42)?.is_some());
        assert_eq!(manager.get(2, 42)?, None);
        assert_eq!(manager.get(42, 1)?, None);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn get_all_by_account_id_ok() -> Result {
        let manager = Db::open_temporary().await?.vote_manager()?;
        let vote = Vote::new_now(Rating::Like);
        manager.insert(1, 42, &vote)?;
        assert_eq!(manager.get_all_by_account_id(0)?, []);
        assert_eq!(manager.get_all_by_account_id(1)?, [(42, vote)]);
        assert_eq!(manager.get_all_by_account_id(2)?, []);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn delete_vote_ok() -> Result {
        let manager = Db::open_temporary().await?.vote_manager()?;
        manager.insert(1, 42, &Vote::new_now(Rating::Like))?;
        manager.delete(1, 42)?;
        assert_eq!(manager.get(1, 42)?, None);
        Ok(())
    }
}
