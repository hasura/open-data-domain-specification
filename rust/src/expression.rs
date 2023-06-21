use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{command::CommandName, types::ArgumentName};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ValueExpression {
    Literal(JsonValue),
    Variable(String),
    Command {
        name: CommandName,
        arguments: HashMap<ArgumentName, ValueExpression>,
    },
    // TODO: Allow querying models
    // TODO: Allow transforming command/model output
}
