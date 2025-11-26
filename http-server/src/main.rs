use std::net::{TcpListener, TcpStream};

use http_server_types::result::Result;

use crate::{
    configuration::Configuration,
    parse_request::parse_request,
    send_response::send_response,
    utils::parse_configuration::{
        ParseConfiguration, ParseConfigurationArgument, ParseConfigurationOption,
        ParseConfigurationType, parse_configuration,
    },
};

pub mod configuration;
pub mod parse_request;
pub mod send_response;
pub mod utils;

fn handle_client(stream: &mut TcpStream, configuration: &Configuration) -> Result<()> {
    let request = parse_request(stream)?;

    send_response(stream, &request, configuration)?;

    Ok(())
}

fn main() -> Result<()> {
    let configuration = parse_configuration(&ParseConfiguration {
        help: "help",
        arguments: &[ParseConfigurationArgument {
            help: "help",
            key: "path",
            r#type: &ParseConfigurationType::Path,
            default: ".",
        }],
        options: &[
            ParseConfigurationOption {
                short: "h",
                long: "help",
                help: "help",
                r#type: &ParseConfigurationType::None,
                default: None,
            },
            ParseConfigurationOption {
                short: "p",
                long: "port",
                help: "help",
                r#type: &ParseConfigurationType::PortType,
                default: Some("8080"),
            },
        ],
    })?;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.port))?;

    for stream in listener.incoming() {
        let result = handle_client(&mut stream?, &configuration);

        if let Err(e) = result {
            println!("error: {e:?}");
        }
    }
    Ok(())
}
