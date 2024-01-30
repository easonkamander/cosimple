mod count;
mod graph;
mod parse;
mod sheaf;

pub use graph::*;
pub use sheaf::*;

impl TryFrom<Vec<u8>> for File {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let value = parse::File::try_from(value.as_slice())
            .map_err(|errs| format!("Parsing Invalid Syntax - {}", errs.first().unwrap()))?;
        let value = count::File::try_from(&value)
            .map_err(|name| format!("Binding Unrecognized Variable - {}", name))?;
        Ok(File::from(&value))
    }
}
