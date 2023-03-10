use std::collections::{HashMap, HashSet};

use bytes::{BufMut, BytesMut, Bytes};
use common::{bail, VResult, WriteExtensions};

use common::Serialize;

use crate::network::packets::ConnectedPacket;
use crate::command::CommandEnum;

use crate::command::Command;

pub const COMMAND_PARAMETER_VALID: u32 = 0x100000;
pub const COMMAND_PARAMETER_ENUM: u32 = 0x200000;
pub const COMMAND_PARAMETER_SUFFIXED: u32 = 0x1000000;
pub const COMMAND_PARAMETER_SOFT_ENUM: u32 = 0x4000000;

#[derive(Debug, Clone)]
pub struct AvailableCommands<'a> {
    /// List of available commands
    pub commands: &'a [Command],
}

impl ConnectedPacket for AvailableCommands<'_> {
    const ID: u32 = 0x4c;

    fn serialized_size(&self) -> usize {
        0
    }
}

impl Serialize for AvailableCommands<'_> {
    fn serialize(&self, buffer: &mut BytesMut) {
        let mut value_indices = HashMap::new();
        let mut values = Vec::new();
        for command in self.commands {
            for alias in &command.aliases {
                if !value_indices.contains_key(alias) {
                    value_indices.insert(alias, values.len() as u32);
                    values.push(alias);
                }
            }

            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if let Some(ref command_enum) = parameter.command_enum {
                        for option in &command_enum.options {
                            if !value_indices.contains_key(option) {
                                value_indices.insert(option, values.len() as u32);
                                values.push(option);
                            }
                        }
                    }
                }
            }
        }

        let mut suffix_indices = HashMap::new();
        let mut suffixes = Vec::new();
        for command in self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if !parameter.suffix.is_empty() && !suffix_indices.contains_key(&parameter.suffix) {
                        suffix_indices.insert(
                            &parameter.suffix,
                            suffixes.len() as u32,
                        );
                        suffixes.push(&parameter.suffix);
                    }
                }
            }
        }

        let mut enum_indices = HashMap::new();
        let mut enums = Vec::new();
        for command in self.commands {
            if !command.aliases.is_empty() {
                let alias_enum = CommandEnum {
                    enum_id: command.name.clone() + "Aliases",
                    options: command.aliases.clone(),
                    dynamic: false,
                };
                enum_indices.insert(
                    command.name.clone() + "Aliases",
                    enums.len() as u32,
                );
                enums.push(alias_enum);
            }

            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if let Some(ref command_enum) = parameter.command_enum {
                        if !command_enum.dynamic
                            && !command_enum.options.is_empty()
                            && !enum_indices.contains_key(&command_enum.enum_id)
                        {
                            enum_indices.insert(
                                command_enum.enum_id.clone(),
                                enums.len() as u32,
                            );
                            enums.push(command_enum.clone());
                        }
                    }
                }
            }
        }

        let mut dynamic_indices = HashMap::new();
        let mut dynamic_enums = Vec::new();
        for command in self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if let Some(ref command_enum) = parameter.command_enum {
                        if command_enum.dynamic && !dynamic_indices.contains_key(&command_enum.enum_id) {
                            dynamic_indices.insert(
                                &command_enum.enum_id,
                                dynamic_enums.len() as u32,
                            );
                            dynamic_enums.push(&parameter.command_enum);
                        }
                    }
                }
            }
        }

        buffer.put_var_u32(values.len() as u32);
        for value in values {
            buffer.put_string(value);
        }

        buffer.put_var_u32(suffixes.len() as u32);
        for suffix in suffixes {
            buffer.put_string(suffix);
        }

        buffer.put_var_u32(enums.len() as u32);
        for command_enum in &enums {
            buffer.put_string(&command_enum.enum_id);
            buffer.put_var_u32(command_enum.options.len() as u32);

            let index_count = value_indices.len() as u32;
            for option in &command_enum.options {
                if index_count <= u8::MAX as u32 {
                    buffer.put_u8(value_indices[option] as u8);
                } else if index_count <= u16::MAX as u32 {
                    buffer.put_u16_le(value_indices[option] as u16);
                } else {
                    buffer.put_u32_le(value_indices[option]);
                }
            }
        }

        buffer.put_var_u32(self.commands.len() as u32);
        for command in self.commands {
            let mut alias = if !command.aliases.is_empty() {
                enum_indices[&(command.name.clone() + "Aliases")] as i32
            } else {
                -1
            };

            buffer.put_string(&command.name);
            buffer.put_string(&command.description);
            buffer.put_u16_le(0); // Command flags. Unknown.
            buffer.put_u8(command.permission_level as u8);
            buffer.put_i32_le(alias);

            buffer.put_var_u32(command.overloads.len() as u32);
            for overload in &command.overloads {
                buffer.put_var_u32(overload.parameters.len() as u32);
                for parameter in &overload.parameters {
                    let mut command_type = parameter.data_type as u32;
                    
                    if let Some(ref command_enum) = parameter.command_enum {                            
                        if command_enum.dynamic {
                            command_type = COMMAND_PARAMETER_SOFT_ENUM
                                | COMMAND_PARAMETER_VALID
                                | dynamic_indices[&command_enum.enum_id];
                        } else if !command_enum.options.is_empty() {
                            command_type = COMMAND_PARAMETER_ENUM
                                | COMMAND_PARAMETER_VALID
                                | enum_indices[&command_enum.enum_id];
                        } else if !parameter.suffix.is_empty() {
                            command_type = COMMAND_PARAMETER_SUFFIXED
                                | suffix_indices[&parameter.suffix];
                        }
                    } else {
                        command_type |= COMMAND_PARAMETER_VALID;
                    }

                    buffer.put_string(&parameter.name);
                    buffer.put_i32_le(command_type as i32);
                    buffer.put_bool(parameter.optional);
                    buffer.put_u8(parameter.options);
                }
            }
        }

        buffer.put_var_u32(dynamic_enums.len() as u32);
        for dynamic_enum in dynamic_enums.iter().copied().flatten() {
            buffer.put_string(&dynamic_enum.enum_id);
            buffer.put_var_u32(dynamic_enum.options.len() as u32);

            for option in &dynamic_enum.options {
                buffer.put_string(option);
            }
        }

        buffer.put_var_u32(0); // No constraints, they are useless.
    }
}
