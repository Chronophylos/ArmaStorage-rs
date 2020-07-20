use std::os::raw::c_int;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCodes {
    Ok = 0,
    InvalidUtf8 = 1,
    UnknownFunction = 2,
    MissingArgument = 10,
    EmptyArgument = 11,
    StorageError = 20,
}

impl Into<c_int> for ErrorCodes {
    fn into(self) -> c_int {
        self as c_int
    }
}
