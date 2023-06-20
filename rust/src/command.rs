
use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::types::{ArgumentDefinition, Type, TypeName, TypeSource};

#[derive(Serialize, Deserialize, JsonSchema)]

pub struct CommandName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Command {
    pub name: CommandName,
    pub arguments: Vec<ArgumentDefinition>,
    pub output_type: Type,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandSource {
    pub command_source: JsonValue,
    pub type_sources: HashMap<TypeName, TypeSource>,
}