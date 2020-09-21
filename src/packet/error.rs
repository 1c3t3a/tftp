use std::convert::AsRef;
use std::fmt;
use std::io::{self, ErrorKind, Result};

use crate::bytes::{Bytes, FromBytes, IntoBytes};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Code {
    NotDefined = 0,
    FileNotFound = 1,
    AccessViolation = 2,
    DiskFull = 3,
    IllegalOperation = 4,
    UnknownTid = 5,
    FileAlreadyExists = 6,
    NoSuchUser = 7,
}

impl Code {
    pub fn from_u16(val: u16) -> Result<Self> {
        Ok(match val {
            v if v == 0 => Code::NotDefined,
            v if v == 1 => Code::FileNotFound,
            v if v == 2 => Code::AccessViolation,
            v if v == 3 => Code::DiskFull,
            v if v == 4 => Code::IllegalOperation,
            v if v == 5 => Code::UnknownTid,
            v if v == 6 => Code::FileAlreadyExists,
            v if v == 7 => Code::NoSuchUser,
            _ => return Err(ErrorKind::InvalidInput.into()),
        })
    }
}

impl IntoBytes for Code {
    fn into_bytes(self) -> Vec<u8> {
        let val = self as u16;
        let bytes = Bytes::new(val);
        bytes.into_bytes()
    }
}

impl FromBytes for Code {
    type Error = io::Error;

    fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        let bytes = Bytes::from_bytes(bytes)?;
        let code = Code::from_u16(bytes.into_inner())?;
        Ok(code)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Code::NotDefined => "Not defined, see error message (if any)",
            Code::FileNotFound => "File not found",
            Code::AccessViolation => "Access violation",
            Code::DiskFull => "Disk full or allocation exceeded",
            Code::IllegalOperation => "Illegal TFTP operation",
            Code::UnknownTid => "Unknown transfer ID",
            Code::FileAlreadyExists => "File already exists",
            Code::NoSuchUser => "No such user",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: Code,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_conversions() {
        assert_eq!(Code::from_u16(0).unwrap(), Code::NotDefined);
        assert_eq!(Code::from_u16(1).unwrap(), Code::FileNotFound);
        assert_eq!(Code::from_u16(2).unwrap(), Code::AccessViolation);
        assert_eq!(Code::from_u16(3).unwrap(), Code::DiskFull);
        assert_eq!(Code::from_u16(4).unwrap(), Code::IllegalOperation);
        assert_eq!(Code::from_u16(5).unwrap(), Code::UnknownTid);
        assert_eq!(Code::from_u16(6).unwrap(), Code::FileAlreadyExists);
        assert_eq!(Code::from_u16(7).unwrap(), Code::NoSuchUser);
        assert!(Code::from_u16(8).is_err());
    }
}