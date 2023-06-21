use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod command;
pub mod data_source;
pub mod expression;
pub mod graphql;
pub mod model;
pub mod permissions;
pub mod relationship;
pub mod types;

pub use command::{Command, CommandSource};
pub use data_source::DataSource;
pub use graphql::{GraphQlType, ModelGraphQlApi};
pub use model::{Model, ModelSource};
pub use relationship::Relationship;
pub use types::{EnumType, NewType, ObjectType, ScalarTypeRepresentation, UnionType};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum OpenDdsObject {
    Collection { objects: Vec<OpenDdsObject> },
    DataSource(DataSource),
    ObjectType(ObjectType),
    UnionType(UnionType),
    EnumType(EnumType),
    NewType(NewType),
    ScalarTypeRepresentation(ScalarTypeRepresentation),
    Model(Model),
    ModelSource(ModelSource),
    Command(Command),
    CommandSource(CommandSource),
    Relationship(Relationship),
    // TypeOutputPermissions(TypeOutputPermissions),
    // ModelSelectPermissions(ModelSelectPermissions),
    // ModelInsertPermissions(ModelInsertPermissions),
    // ModelUpdatePermissions(ModelUpdatePermissions),
    // ModelDeletePermissions(ModelDeletePermissions),
    // CommandPermissions(CommandPermissions),
    GraphqlType(GraphQlType),
    ModelGraphqlApi(ModelGraphQlApi),
    // ScalarTypeGraphqlExpression(ScalarTypeGraphqlExpression),
    // CommandGraphqlApi(CommandGraphqlApi),
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OpenDds(pub Vec<OpenDdsObject>);
