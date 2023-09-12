use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    command::CommandName,
    expression::ValueExpression,
    model::ModelName,
    types::{ArgumentName, FieldName, TypeName},
};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RelationshipName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Relationship {
    pub name: RelationshipName,
    pub source: TypeName,
    pub target: RelationshipTarget,
    pub mappings: Vec<RelationshipMapping>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum RelationshipType {
    Object,
    Array,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum RelationshipTarget {
    #[serde(rename_all = "camelCase")]
    Model {
        model: ModelName,
        relationship_type: RelationshipType,
        field_path: Vec<FieldName>,
    },
    #[serde(rename_all = "camelCase")]
    Command {
        command: CommandName,
        field_path: Vec<FieldName>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipMappingSource {
    Value(ValueExpression),
    FieldPath(Vec<FieldAccess>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FieldAccess {
    name: FieldName,
    #[serde(default)]
    arguments: HashMap<ArgumentName, ValueExpression>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipMappingTarget {
    Argument(ArgumentName),
    ModelField(Vec<FieldAccess>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RelationshipMapping {
    source: RelationshipMappingSource,
    target: RelationshipMappingTarget,
}
