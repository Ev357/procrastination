use std::{collections::HashMap, env, path::PathBuf};

use http_server_types::{
    error::{ArgError, Error},
    result::Result,
};

use crate::{configuration::Configuration, utils::build_configuration::build_configuration};

#[derive(Debug)]
pub struct ParseConfiguration {
    pub help: &'static str,
    pub arguments: &'static [ParseConfigurationArgument],
    pub options: &'static [ParseConfigurationOption],
}

#[derive(Debug, Clone)]
pub enum ParseConfigurationType {
    Path,
    PortType,
    None,
}

impl ParseConfigurationType {
    pub fn is_valid(&self, argument: &str) -> Result<()> {
        match self {
            ParseConfigurationType::Path => {
                let path = PathBuf::from(argument);
                if !path.is_dir() {
                    return Err(Error::Arg(ArgError::InvalidPath(argument.to_string())));
                }

                Ok(())
            }
            ParseConfigurationType::PortType => {
                let port = argument.parse::<u16>();

                if port.is_err() {
                    return Err(Error::Arg(ArgError::InvalidPort(argument.to_string())));
                }

                Ok(())
            }
            ParseConfigurationType::None => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ParseConfigurationArgument {
    pub help: &'static str,
    pub key: &'static str,
    pub r#type: &'static ParseConfigurationType,
    pub default: &'static str,
}

#[derive(Debug)]
pub struct ParseConfigurationOption {
    pub short: &'static str,
    pub long: &'static str,
    pub help: &'static str,
    pub r#type: &'static ParseConfigurationType,
    pub default: Option<&'static str>,
}

pub fn parse_configuration(configuration: &ParseConfiguration) -> Result<Configuration> {
    let args = env::args().skip(1);

    struct FoldState {
        key: String,
        r#type: ParseConfigurationType,
    }

    let (arguments, options, _) = args.fold(
        (vec![], HashMap::new(), None::<FoldState>),
        |(mut arguments, mut options, fold_state), arg| match fold_state {
            None => {
                let option = configuration.options.iter().find(|option| {
                    format!("-{}", option.short) == arg || format!("--{}", option.long) == arg
                });

                if let Some(option) = option {
                    if let ParseConfigurationType::None = option.r#type {
                        options.insert(option.long.to_string(), None);

                        return (arguments, options, None);
                    }

                    return (
                        arguments,
                        options,
                        Some(FoldState {
                            key: option.long.to_string(),
                            r#type: option.r#type.clone(),
                        }),
                    );
                }

                arguments.push(arg.to_string());

                (arguments, options, None)
            }
            Some(state) => match state.r#type {
                ParseConfigurationType::None => (arguments, options, None),
                _ => {
                    options.insert(state.key.to_string(), Some(arg.to_string()));

                    (arguments, options, None)
                }
            },
        },
    );

    build_configuration(&arguments, &options, configuration)
}
