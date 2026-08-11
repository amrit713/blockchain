pub trait Verifiable {
    type Error;
    fn verify(&self) -> Result<(), Self::Error>;
}
