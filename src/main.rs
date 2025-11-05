use std::net::{TcpListener, TcpStream};

use crate::{
    parse_request::parse_request,
    send_response::send_response,
    types::{error::Error, method::Method, result::Result},
};

pub mod parse_request;
pub mod send_response;
pub mod types;
pub mod utils;

fn handle_client(stream: &mut TcpStream) -> Result<()> {
    let request = parse_request(stream)?;

    send_response(stream, &request)?;

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4242")?;

    for stream in listener.incoming() {
        let result = handle_client(&mut stream?);

        if let Err(e) = result {
            println!("error: {e:?}");
        }
    }
    Ok(())
}
