use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataSourceName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataSource {
    pub name: DataSourceName,
    pub source_type: JsonValue,
    pub source_config: JsonValue,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OperatorName(pub String);
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OperatorArgumentName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OperatorArgumentDefinition {
    pub name: OperatorArgumentName,
    #[serde(rename = "type")]
    pub argument_type: ScalarType,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ComparisonOperatorDefinition {
    name: OperatorName,
    arguments: OperatorArguments,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum OperatorArguments {
    #[serde(rename = "argument")]
    SingleArgument(ScalarType),
    #[serde(rename = "arguments")]
    MultipleArguments(Vec<OperatorArgumentDefinition>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AggregateOperatorDefinition {
    name: OperatorName,
    #[serde(rename = "outputType")]
    output_type: ScalarType,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateOperatorDefinition {
    name: OperatorName,
    arguments: Option<OperatorArguments>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ScalarType {
    Container(ScalarTypeContainer),
    Named(ScalarTypeName),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ScalarTypeContainer {
    Nullable(Box<ScalarType>),
    Array(Box<ScalarType>),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ScalarTypeName(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScalarTypeDefinition {
    pub name: ScalarTypeName,
    #[serde(default)]
    pub comparison_operators: Vec<ComparisonOperatorDefinition>,
    #[serde(default)]
    pub aggregate_operators: Vec<AggregateOperatorDefinition>,
    #[serde(default)]
    pub update_operators: Vec<UpdateOperatorDefinition>,
}
