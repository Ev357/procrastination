use std::collections::HashMap;

use http_server_types::{
    error::{ArgError, Error},
    result::Result,
};

use crate::{
    configuration::Configuration,
    utils::parse_configuration::{ParseConfiguration, ParseConfigurationType},
};

pub fn build_configuration(
    arguments: &[String],
    options: &HashMap<String, Option<String>>,
    configuration: &ParseConfiguration,
) -> Result<Configuration> {
    let mut parsed_arguments = HashMap::<String, String>::new();

    let mut arguments_iterator = arguments.iter();
    configuration
        .arguments
        .iter()
        .try_for_each(|argument_configuration| {
            let argument = arguments_iterator.next();
            if let Some(argument) = argument {
                let result = argument_configuration.r#type.is_valid(argument);
                if result.is_ok() {
                    parsed_arguments
                        .insert(argument_configuration.key.to_string(), argument.to_string());
                };
                return result;
            }

            parsed_arguments.insert(
                argument_configuration.key.to_string(),
                argument_configuration.default.to_string(),
            );
            Ok(())
        })?;

    if let Some(argument) = arguments_iterator.next() {
        return Err(Error::Arg(ArgError::InvalidArgument(argument.to_string())));
    }

    configuration
        .options
        .iter()
        .try_for_each(|option_configuration| {
            let argument = options.get(option_configuration.long);
            if let Some(argument) = argument {
                return match option_configuration.r#type {
                    ParseConfigurationType::None => match argument {
                        Some(argument) => {
                            let result = option_configuration.r#type.is_valid(argument);
                            if result.is_ok() {
                                parsed_arguments.insert(
                                    option_configuration.long.to_string(),
                                    argument.to_owned(),
                                );
                            };
                            result
                        }
                        None => {
                            if let Some(default) = option_configuration.default {
                                parsed_arguments.insert(
                                    option_configuration.long.to_string(),
                                    default.to_owned(),
                                );
                            };
                            Ok(())
                        }
                    },
                    _ => match argument {
                        Some(argument) => {
                            let result = option_configuration.r#type.is_valid(argument);
                            if result.is_ok() {
                                parsed_arguments.insert(
                                    option_configuration.long.to_string(),
                                    argument.to_owned(),
                                );
                            };
                            result
                        }
                        None => Err(Error::Arg(ArgError::MissingArgument(
                            option_configuration.long.to_owned(),
                        ))),
                    },
                };
            }
            if let Some(default) = option_configuration.default {
                parsed_arguments.insert(option_configuration.long.to_string(), default.to_owned());
            }

            Ok(())
        })?;

    Configuration::from_hashmap(&parsed_arguments)
}
