mod shape;
mod split;
mod terms;

pub use split::*;
pub use terms::{Sheaf, Term};

impl TryFrom<Vec<u8>> for File {
    type Error = <terms::File as TryFrom<Vec<u8>>>::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let value = terms::File::try_from(value)?;
        let value = shape::File::from(value);
        Ok(File::from(value))
    }
}
