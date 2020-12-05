use std::error::Error;
use std::fmt;

// NoResultError is a generic error
// which shows that the answer could not
// be found.
#[derive(Debug)]
pub struct NoResultError {
    details: String,
}

impl NoResultError {
    // new creates an instance of the NoResultError,
    // a description of the error can be passed in
    // as a parameter.
    pub fn new(msg: &str) -> NoResultError {
        NoResultError {
            details: msg.to_string(),
        }
    }
}

// Implement fmt::Display, this needs to be done
// to allow for the error to be displayed.
impl fmt::Display for NoResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// Implement Error, this needs to be done
// to allow NoResultError to be used as
// a type Error.
impl Error for NoResultError {
    fn description(&self) -> &str {
        &self.details
    }
}

// SomethingIsWrongError is a generic error
// which shows if something has not worked.
#[derive(Debug)]
pub struct SomethingIsWrongError {
    details: String,
}

impl SomethingIsWrongError {
    // new creates an instance of the SomethingIsWrongError,
    // a description of the error can be passed in
    // as a parameter.
    pub fn new(msg: &str) -> SomethingIsWrongError {
        SomethingIsWrongError {
            details: msg.to_string(),
        }
    }
}

// Implement fmt::Display, this needs to be done
// to allow for the error to be displayed.
impl fmt::Display for SomethingIsWrongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// Implement Error, this needs to be done
// to allow SomethingIsWrongError to be used as
// a type Error.
impl Error for SomethingIsWrongError {
    fn description(&self) -> &str {
        &self.details
    }
}
