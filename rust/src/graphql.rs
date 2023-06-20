use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    model::ModelName,
    types::{FieldName, TypeName},
};

#[derive(Serialize, Deserialize, JsonSchema)]

pub struct GraphQlTypeName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum GraphQlTypeKind {
    Object,
    Union,
    Interface,
    Scalar,
    InputObject,
}

/// Definition of GraphQl output types.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlType {
    /// Metadata type to map to this GraphQl type. One metadata type can map to at most one output type.
    pub type_name: TypeName,
    /// The name to give this GraphQl output type. Must be unique across the API.
    pub graphql_type_name: GraphQlTypeName,
    /// The kind of the GraphQl Type
    pub graphql_type_kind: GraphQlTypeKind,
}

/// The definition of the GraphQL API component specific to a model.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelGraphQlApi {
    pub model_name: ModelName,
    pub select_uniques: Vec<SelectUniqueGraphQlDefinition>,
}

/// The name of a GraphQL object field.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GraphQlFieldName(pub String);

/// The definition of the GraphQL API for selecting a unique row/object from a model.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectUniqueGraphQlDefinition {
    /// The name of the query root field for this API.
    pub query_root_field: GraphQlFieldName,
    /// A set of fields which can uniquely identify a row/object in the model.
    pub unique_identifier: Vec<FieldName>,
}
