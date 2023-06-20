# Open Data Domain Specification (OpenDDS)

*This is the evolution of the previously described [Graphql Data Specification](https://github.com/hasura/graphql-data-specification).*

OpenDDS is a JSON specification for describing the data graph in your business domain, role based access control over this data, and any APIs over it.

OpenDDS is agnostic of any particular database and the core data domain definition is agnostic of the API format.

Any OpenDDS runtime (eg: Hasura) can extend it with additional configuration specific to that particular runtime.

**Status**: (Draft) This specification is a work in progress and can change rapidly. There are still some additions that need to be made to the spec for it to be fully functional.

## Concepts

OpenDDS defines the following concepts:

### OpenDDS object

The OpenDDS configuration is composed of a set of OpenDDS objects. Each object is defined by its `kind` and custom configuration depending on the kind.

There is a special object kind called `collection` which can group multiple objects.

### Data Source

The `dataSource` object defines a physical data source. Every data source will contain the following:
- Name of the data source
- Scalar types defined by the data source.
  - Every scalar type definition will contain:
    - Name of the scalar type.
    - Comparison operators on the scalar type, consisting of an operator name and the input arguments along with their types.
    - Aggregation operators on the scalar type, consisting of an operator name and the output scalar type.
    - Update operators on the scalar type, consistent of input arguments along with their types.
  - Argument and output types can either be scalar types or container types like "nullable" or "array".
- A data source type and a data source config, the structure and semantics of which are defined by a particular OpenDDS runtime.
  
Example OpenDDS object:

```json
{
    "kind": "dataSource",
    "name": "my-postgres-db",
    "scalarTypes": [{
        "name": "integer",
        "comparisonOperators": [{
            "name": "equals",
            "argument": "integer",
        }],
        "aggregationOperators": [{
            "name": "max",
            "outputType": "integer"
        }],
        "updateOperators": [{
            "name": "increment",
        }],
    }, {
        "name": "varchar",
        "comparisonOperators": [{
            "name": "like",
            "argument": "varchar",
        }],
        "updateOperators": [{
            "name": "set",
            "argument": "varchar",
        }],
    }],
    // The format of data inside these fields is defined
    // by the actual OpenDDS runtime (eg: Hasura)
    "sourceType": "postgres",
    "sourceConfig": {
        "connectionString": {
            "valueFromSecret": "MY-POSTGRES-DB-CONNECTION-STRING"
        }
    }
}
```

OpenDDS runtimes can additionally include any other fields in the data source definition (eg: connection strings for databases).

### Types

Types are a fundamental building block of OpenDDS with every bit of data in OpenDDS having a type.

- OpenDDS has a few primitive types
  - Integer: 32 bit signed integer
  - Float: Double-precision floating point
  - String: UTF-8 encoded string
  - Boolean
  - Nullable: Type container, representing an optional value
  - Array: Type container, representing repeated values
- User defined OpenDDS types can be one of
  - Object types: Complex types with fields
    - Each field has a name, a type, and optionally arguments that change their output behavior
  - Enum types: Enumerated set of values
  - Union types: Sum types with each variant being identified by a tag/name and a type
  - Opaque types: Types whose representation is opaque to OpenDDS
- OpenDDS also supports type inheritance, with a particular type taking 

Example:
```json
{
    "kind": "collection",
    "objects": [{
        "kind": "opaqueType",
        "name": "UUID"
    }, {
        "kind": "enumType",
        "name": "ProductCategory",
        "values": ["Health", "Technology", "Apparel"]
    }, {
        "kind": "objectType",
        "name": "Product",
        "fields": [{
            "name": "id",
            "type": "UUID"
        }, {
            "name": "name",
            "type": "String",
        }, {
            "name": "shipping_cost",
            "type": "float",
            "selectArguments": [{
                "name": "zipcode",
                "type": "String"
            }]
        }, {
            "name": "tags",
            "type": { "array": "String" }
        }, {
            "name": "category",
            "type": { "nullable": "ProductCategory" }
        }],
    }],
}
```

### Models

Models represent a collection of data objects of a certain type. They are the primary way of accessing data in OpenDDS. A model may in reality be backed by a database table, an ad-hoc SQL query, a pre-materialized view, a custom API server, etc.

Each model defines:
- The name of the model
- The data type of which this model is a collection.
- The capabilities of this model.
  - Selectable: Whether this model can be queried for data
    - Filterable: Whether this model supports filters when querying
    - Aggregatable: Whether this model can be aggregated
    - Paginatable: Whether this model supports pagination (offset)
    - Limitable: Whether this model supports retrieving limited objects
  - Insertable: Whether new objects can be inserted into this model
  - Updateable: Whether existing objects can be updated in this model
  - Deleteable: Whether existing objects can be deleted from this model
- Models can optionally take arguments to change the behavior of the model at runtime

Examples OpenDDS objects:
```json
{
    "kind": "model",
    "name": "Products",
    "dataType": "Product",
    "capabilities": {
        "select": {
            "selectable": true,
            "filterable": true,
            "aggregatable": true,
            "paginatable": true,
            "limitable": true,
        },
        "insert": { "insertable": true },
        "update": { "updateable": true },
        "delete": { "deletable": true }
    }
}
```
```json
{
    "kind": "model",
    "name": "BestSellingProducts",
    "dataType": "Product",
    "arguments": [{
        "name": "bestSellingThreshold",
        "type": "Float"
    }],
    "capabilities": {
        "select": {
            "selectable": true,
            "filterable": true,
            "aggregatable": true,
            "paginatable": false,
            "limitable": true,
        },
        "insert": { "insertable": false },
        "update": { "updateable": false },
        "delete": { "deletable": false }
    }
}
```

### Model Source

A model can be attached to a data source and the `modelSource` OpenDDS object can define how the model and its types / fields map to entities in the underlying data source.

The `modelSource` object contains:
- The model name for which the source is being defined
- The data source name corresponding to this model
- The underlying entity within the data source corresponding to this model
- For each OpenDDS type used within the model, including the data type of model elements, the mapping between the type specifics (like field names) to the underlying entities within the data source (like column names)

Example:
```json
{
    "kind": "modelSource",
    "modelName": "Products",
    "dataSource": "my-postgres-db",
    // Structure for modelSource is defined by the
    // particular OpenDDS runtime
    "modelSource": {
        "schema": "public",
        "table": "products",
    },
    "typeSources": {
        "Product": {
            // Structure of each field source is defined
            // by the particular OpenDDS runtime
            "fieldSources": {
                "id": { "columnName": "id" },
                "name": { "columnName": "product_name" }
            }
        }
    }
}
```
### Commands
Commands are the other way of accessing data within OpenDDS. Commands are functions / lambdas whose semantics are opaque to DDS except for their input arguments and output type.

Each `command` OpenDDS object defines:
- The name of the command
- The arguments to the command
- The output type of the command

Examples:

```json
{
    "kind": "command",
    "name": "Checkout",
    "arguments": [{
        "name": "cart_id",
        "type": "UUID"
    }, {
        "name": "card_number",
        "type": "String",
    }],
    "outputType": "CheckoutResult"
}
```

### Command Source
Similar to model source, the `commandSource` OpenDDS object defines how the command the types it uses map to an underlying data source. It contains:
- The command name for which the source is being defined
- The data source name corresponding to this model
- The underlying entity in the data source that corresponds to this command
- For each OpenDDS type used within the command, the mapping between the type specifics (like field names) to the underlying entities within the data source

Example:

```json
{
    "kind": "commandSource",
    "commandName": "Checkout",
    "dataSource": "my-checkout-api",
    // Structure of commandSource is defined
    // by the particular OpenDDS runtime
    "commandSource": {
        "post_path": "/checkout"
    },
    "typeSources": {
        "CheckoutResult": {
            // Structure of each field source is defined
            // by the particular OpenDDS runtime
            "fieldSources": {
                "code": { "responseField": "err_code" },
                "message": { "responseField": "err_msg" }
            }
        }
    }
}
```

### Relationships

Relationships define that given an object of a certain type, how to find related objects in a model or command. 

A `relationship` OpenDDS object defines:
- The name of the relationship
- The source type for the relationship
- The target of the relationship. It can either be
  - A model, or a field nested within the model's data type.
    - This can either be an one-to-one relationship or a one-to-many relationship
    - This also defines the source value (either preset or from the source type) for any model argument
    - This also defines mappings between source type fields to model fields that are filterable
  - A command, or a field nested within the command's output type
    - This also defines the source value (either preset or from the source type) for any command argument

```json
{
    "kind": "relationship",
    "source": "Category",
    "name": "BestSellers",
    "target": {
        "modelName": "BestSellingProducts",
        // Empty because we want relationship to the
        // top-level Product type in BestSellingProducts
        "fieldPath": [],
        // One category can have multiple best sellers
        "relationshipType": "ARRAY",
        "fieldMappings": [{
            "sourcePath": [{ "name": "id" }],
            "targetPath": [{ "name": "category_id" }]
        }]
    },
    "argumentMappings": {
        "argumentName": "bestSellingThreshold",
        "source": {
            // Can also be from a field of the source type
            "value": { "literal": 1000 }
        }
    }
}
```

### Permissions

OpenDDS allows you to define role based access control across types, models, and commands.

For any role, you can define the following kinds of permissions:

- Type output permissions: When selecting an object of a type:
  - What fields are allowed to be accessed
  - Any preset valuess or validation predicates for field arguments
- Model select permissions: When selecting objects from a model
  - Predicate defining which objects or rows within the model can be retrieved
  - Whether aggregations are allowed for this role
- Model insert permissions: When inserting objects into a model
  - Which fields are allowed to be inserted
  - Any preset values or validation predicates for the fields being inserted
- Model update permissions: When updating objects into a model
  - Which fields are allowed to be updated
  - predicate defining which objects or rows within the model can be updated
- Model delete permissions: When deleting objects from a model
  - predicate defining which objets or rows within the model can be deleted
- Command permissions: When executing a command
  - Any preset values or validation predicates for command arguments

Example:

```json
{
    "kind": "collection",
    "objects": [{
        "kind": "typeOutputPermissions",
        "typeName": "Product",
        "roles": {
            "user": {
                "fields": ["id", "name", "shippingCost"],
                "fieldArguments": {
                    "shippingCost": [{
                        "preset": {
                            "command": {
                                "name": "GetCurrentLocationZipcode"
                            }
                        }
                    }]
                }
            },
            "seller": {
                "fields": ["id", "name", "shippingCost", "category", "tags"],
            }
        }
    }, {
        "kind": "modelSelectPermissions",
        "typeName": "Product",
        "roles": {
            "user": {
                "filter": null
            },
            "seller": {
                "filter": {
                    "fieldComparison": {
                        "fieldName": "seller_id",
                        "operator": "equals",
                        "value": {
                            "variable": "X-HASURA-SELLER-ID"
                        }
                    }
                }
            }
        }
    }]
}
```

### Predicates
TODO: Describe the available OpenDDS predicates in detail

### GraphQl API Definition

After configure the core data graph, OpenDDS lets you configure a GraphQl API over that data. You can define:

- GraphQL types: Defines GraphQL types corresponding to OpenDDS types including
  - GraphQL type name
  - OpenDDS type name that backs this GraphQl type
  - GraphQL type kind (eg: object, input object, union, interface, etc.)
- GraphQL boolean expression type: For a particular scalar type, the GraphQl boolean expression type name for it
- GraphQL model API: This defines the API over a model, including
  - Query root field to retrieve a single object from the model using a unique identifier
  - Query root field to retrieve multiple objects from the model
    - Includes the name of the GraphQl type for the filter predicate
  - Query root field to retrieve aggregated data from the model
  - Mutation root fields for insert / update / delete
- GraphQL command API: This defines a query or mutation root field for a command

Example:
```json
{
    "kind": "collection",
    "objects": [{
        "kind": "graphqlType",
        "typeName": "Product",
        "graphqlTypeName": "Product",
        "graphqlTypeKind": "Object"
    }, {
        "kind": "modelGraphqlApi",
        "modelName": "Products",
        "selectUniques": [
            {
                "queryRootField": "ProductById",
                "uniqueIdentifier": ["id"]
            }
        ]
    }],
}
```

## Semantics

## References

Rust data structure: OpenDds in rust/lib.rs \
TypeScript structures: TODO \
JSONSchema: TODO