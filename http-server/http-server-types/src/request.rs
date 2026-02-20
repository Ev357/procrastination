use std::path::PathBuf;

use crate::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: PathBuf,
}
