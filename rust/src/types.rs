use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::data_source::{DataSourceName, ScalarTypeName};

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Clone)]
pub struct TypeName(pub String);

/// A reference to an OpenDDS type.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Type {
    Inbuilt(InbuiltType),
    Custom(TypeName),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum InbuiltType {
    Int,
    Float,
    Boolean,
    String,
    Nullable(Box<Type>),
    Array(Box<Type>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ObjectType {
    pub name: TypeName,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash, Clone)]
pub struct FieldName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FieldDefinition {
    pub name: FieldName,
    #[serde(rename = "type")]
    pub field_type: Type,
    #[serde(default)]
    pub arguments: Vec<ArgumentDefinition>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ArgumentName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ArgumentDefinition {
    pub name: ArgumentName,
    #[serde(rename = "type")]
    pub argument_type: Type,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EnumType {
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UnionType {
    pub variants: Vec<UnionTypeVariant>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UnionTypeVariant {
    pub tag: String,
    #[serde(rename = "type")]
    pub variant_type: Type,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct NewType {
    pub name: TypeName,
    pub representation: Option<Type>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScalarTypeRepresentation {
    pub data_source: DataSourceName,
    pub scalar_type: ScalarTypeName,
    pub representation: Type,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TypeSource {
    #[serde(rename_all = "camelCase")]
    Object {
        field_sources: HashMap<FieldName, FieldSource>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FieldSource {
    pub field_source: JsonValue,
    // TODO: Map field arguments
}