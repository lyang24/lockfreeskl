pub(crate) struct Key {
    pub user_key: Vec<u8>,
    pub trailer: u64,
}
pub enum InternalKeyType {
    Invalid,
}

impl From<InternalKeyType> for u64 {
    fn from(key_type: InternalKeyType) -> Self {
        match key_type {
            InternalKeyType::Invalid => 255,
        }
    }
}