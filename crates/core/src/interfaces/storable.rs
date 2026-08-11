use crypto::Hash;

pub trait Storable {
    fn hash(&self) -> Hash;
    fn to_bytes(&self) -> Vec<u8>;
}
