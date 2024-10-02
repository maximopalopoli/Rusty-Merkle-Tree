use std::num::ParseIntError;

pub enum UserInterfaceErrors{
    NotEnoughArgumentsError(String),
    NotCorrectTypeError(ParseIntError)
}
