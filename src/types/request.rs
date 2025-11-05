use std::path::PathBuf;

use crate::types::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: PathBuf,
    pub version: String,
}
