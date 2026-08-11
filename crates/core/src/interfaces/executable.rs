use crate::interfaces::IAccount;

pub trait Executable {
    type Error;
    fn execute<A: IAccount>(
        &self,
        sender_acc: &mut A,
        recipient_acc: &mut A,
    ) -> Result<(), Self::Error>;
}
