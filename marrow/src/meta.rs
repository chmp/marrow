//! Metadata required to reconstruct arrays
use std::collections::HashMap;

use crate::datatypes::Field;

/// Metadata for a field
#[derive(Clone, Debug)]
pub struct FieldMeta {
    /// The name of the field
    pub name: String,
    /// Nullability flag of the field
    pub nullable: bool,
    /// Additional metadata of the field
    pub metadata: HashMap<String, String>,
}

#[allow(unused)]
pub(crate) fn meta_from_field(field: Field) -> FieldMeta {
    FieldMeta {
        name: field.name,
        nullable: field.nullable,
        metadata: field.metadata,
    }
}
