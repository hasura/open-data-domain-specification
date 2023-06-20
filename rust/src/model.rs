use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{types::{TypeName, ArgumentDefinition, TypeSource}, data_source::DataSourceName};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ModelName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub name: ModelName,
    /// The type of the objects of which this model is a collection.
    pub data_type: TypeName,

    #[serde(default)]
    pub arguments: Vec<ArgumentDefinition>,

    pub capabilities: ModelCapabilities,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelCapabilities {
    pub select: SelectCapabilities,
    pub insert: InsertCapabilities,
    pub update: UpdateCapabilities,
    pub delete: DeleteCapabilities,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SelectCapabilities {
    pub selectable: bool,
    pub offsetable: bool,
    pub limitable: bool,
    pub aggregatable: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InsertCapabilities {
    pub insertable: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateCapabilities {
    pub updateable: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteCapabilities {
    pub deletable: bool,
}


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelSource {
    /// The name of the model for which this defines the source.
    pub model_name: ModelName,

    /// The name of the data source backing this model.
    #[serde(rename = "dataSource")]
    pub data_source_name: DataSourceName,

    /// The entity in the data source that backs this model.
    pub model_source: JsonValue,

    /// How the various types used in this model correspond to
    /// entities in the data source.
    pub type_sources: HashMap<TypeName, TypeSource>,
}