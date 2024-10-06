//! Supported data types
use std::collections::HashMap;

use crate::error::{fail, ErrorKind, MarrowError, Result};

/// The metadata of a field
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// The name of the field
    pub name: String,
    /// The data type of the field
    pub data_type: DataType,
    /// Whether the field supports missing values
    pub nullable: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Supported data types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum DataType {
    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Utf8,
    LargeUtf8,
    Binary,
    LargeBinary,
    FixedSizeBinary(i32),
    Date32,
    Date64,
    Timestamp(TimeUnit, Option<String>),
    Time32(TimeUnit),
    Time64(TimeUnit),
    Duration(TimeUnit),
    Decimal128(u8, i8),
    Struct(Vec<Field>),
    List(Box<Field>),
    LargeList(Box<Field>),
    FixedSizeList(Box<Field>, i32),
    Map(Box<Field>, bool),
    Dictionary(Box<DataType>, Box<DataType>, bool),
    Union(Vec<(i8, Field)>, UnionMode),
}

/// The unit of temporal quantities
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Second => write!(f, "Second"),
            TimeUnit::Millisecond => write!(f, "Millisecond"),
            TimeUnit::Microsecond => write!(f, "Microsecond"),
            TimeUnit::Nanosecond => write!(f, "Nanosecond"),
        }
    }
}

impl std::str::FromStr for TimeUnit {
    type Err = MarrowError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Second" => Ok(Self::Second),
            "Millisecond" => Ok(Self::Millisecond),
            "Microsecond" => Ok(Self::Microsecond),
            "Nanosecond" => Ok(Self::Nanosecond),
            s => fail!(ErrorKind::ParseError, "Invalid TimeUnit: {s}"),
        }
    }
}

#[test]
fn time_unit_as_str() {
    use std::str::FromStr;

    macro_rules! assert_variant {
        ($variant:ident) => {
            assert_eq!((TimeUnit::$variant).to_string(), stringify!($variant));
            assert_eq!(
                TimeUnit::from_str(stringify!($variant)).unwrap(),
                TimeUnit::$variant
            );
        };
    }

    assert_variant!(Second);
    assert_variant!(Millisecond);
    assert_variant!(Microsecond);
    assert_variant!(Nanosecond);
}

/// The storage mode of unions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnionMode {
    /// The underlying arrays also store unused values
    ///
    /// Each underlying array has the same length as the union array.
    Sparse,
    /// The underlying arrays only store used values.
    ///
    /// The sum of all underlying array lengths is the same as the length of the union array.    
    Dense,
}

impl std::fmt::Display for UnionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnionMode::Sparse => write!(f, "Sparse"),
            UnionMode::Dense => write!(f, "Dense"),
        }
    }
}

impl std::str::FromStr for UnionMode {
    type Err = MarrowError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Sparse" => Ok(UnionMode::Sparse),
            "Dense" => Ok(UnionMode::Dense),
            s => fail!(ErrorKind::ParseError, "Invalid UnionMode: {s}"),
        }
    }
}

#[test]
fn union_mode_as_str() {
    use std::str::FromStr;

    macro_rules! assert_variant {
        ($variant:ident) => {
            assert_eq!((UnionMode::$variant).to_string(), stringify!($variant));
            assert_eq!(
                UnionMode::from_str(stringify!($variant)).unwrap(),
                UnionMode::$variant
            );
        };
    }

    assert_variant!(Dense);
    assert_variant!(Sparse);
}
