use mongodb::bson::Bson;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct AccountId(u32);

impl From<AccountId> for Bson {
    #[inline]
    fn from(account_id: AccountId) -> Self {
        Bson::from(account_id.0)
    }
}
