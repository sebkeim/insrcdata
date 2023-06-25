// insrcdata : embed tabular data in source code (https://github.com/sebkeim/insrcdata)
// Copyright (c)  2023 Sébastien Keim
// SPDX-License-Identifier: GPL-3.0-or-later
//
// applicative errors
//

use std::path::Path;
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(&err.to_string())
    }
}

pub fn io_error_result<T>(res: io::Result<T>, path: &Path) -> Result<T> {
    match res {
        Ok(res) => Ok(res),
        Err(err) => {
            let msg = format!("{} {}", path.to_str().unwrap_or("<unnamed>"), err);
            Err(Error::new(&msg))
        }
    }
}
