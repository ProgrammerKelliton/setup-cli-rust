use std::fmt::{self, Display};

#[derive(Debug)]
pub struct IsEmptyFieldError;

impl Display for IsEmptyFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Você precisa informar o valor do campo")
    }
}

pub fn is_empty_field(value: String) -> Result<(), IsEmptyFieldError> {
    if value.trim().is_empty() {
        return Err(IsEmptyFieldError);
    }
    Ok(())
}
