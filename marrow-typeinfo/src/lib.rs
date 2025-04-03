use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::Infallible,
    num::TryFromIntError,
    rc::Rc,
    sync::Arc,
};

use marrow::{
    datatypes::{DataType, Field},
    types::f16,
};

/// Derive [TypeInfo] for a given type
///
/// Currently structs and enums with any type of lifetime parameters are supported.
pub use marrow_typeinfo_derive::TypeInfo;

// TODO: include the path in context to allow overwrites
#[derive(Debug, Default, Clone)]
pub struct Context {
    data: HashMap<TypeId, Rc<dyn Any>>,
}

struct DefaultStringType(DataType);

struct LargeList(bool);

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: Any>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();
        self.data.insert(type_id, Rc::new(value));
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        let key = TypeId::of::<T>();
        let value = self.data.get(&key)?;
        let Some(value) = value.downcast_ref() else {
            unreachable!();
        };
        Some(value)
    }

    pub fn with_default_string_type(mut self, ty: DataType) -> Self {
        // TODO: check that ty is compatible with strings
        self.set(DefaultStringType(ty));
        self
    }

    pub fn with_large_list(mut self, large_list: bool) -> Self {
        self.set(LargeList(large_list));
        self
    }

    pub fn get_field<T: TypeInfo>(&self, name: &str) -> Result<Field, Error> {
        // TODO: allow to overwrite child fields
        T::get_field(name, ContextRef(self))
    }

    pub fn get_data_type<T: TypeInfo>(&self) -> Result<DataType, Error> {
        Ok(self.get_field::<T>("item")?.data_type)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ContextRef<'a>(&'a Context);

impl<'a> ContextRef<'a> {
    pub fn get_context(self) -> &'a Context {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Error(String);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error({:?})", self.0)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self(value.to_string())
    }
}

/// Get the Arrow type information for a given Rust type
///
/// The functions cannot be called directly. First construct a [Context], then call the
/// corresponding methods.
pub trait TypeInfo {
    /// See [Context::get_field]
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error>;

    /// See [Context::get_data_type]
    fn get_data_type(context: ContextRef<'_>) -> Result<DataType, Error> {
        Ok(Self::get_field("item", context)?.data_type)
    }
}

macro_rules! define_primitive {
    ($(($ty:ty, $dt:expr),)*) => {
        $(
            impl TypeInfo for $ty {
                fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
                    let _ = context;
                    Ok(Field {
                        name: name.to_owned(),
                        data_type: $dt,
                        nullable: false,
                        metadata: Default::default(),
                    })
                }
            }
        )*
    };
}

define_primitive!(
    (bool, DataType::Boolean),
    (u8, DataType::UInt8),
    (u16, DataType::UInt16),
    (u32, DataType::UInt32),
    (u64, DataType::UInt64),
    (i8, DataType::Int8),
    (i16, DataType::Int16),
    (i32, DataType::Int32),
    (i64, DataType::Int64),
    (f16, DataType::Float16),
    (f32, DataType::Float32),
    (f64, DataType::Float64),
);

impl TypeInfo for () {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        let _ = context;
        Ok(Field {
            name: name.to_owned(),
            data_type: DataType::Null,
            nullable: true,
            metadata: Default::default(),
        })
    }
}

fn get_default_string_type(context: &Context) -> DataType {
    if let Some(DefaultStringType(ty)) = context.get() {
        ty.clone()
    } else {
        DataType::LargeUtf8
    }
}

fn new_field(name: &str, data_type: DataType) -> Field {
    Field {
        name: name.to_owned(),
        data_type,
        nullable: false,
        metadata: Default::default(),
    }
}

impl TypeInfo for &str {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        Ok(new_field(
            name,
            get_default_string_type(context.get_context()),
        ))
    }
}

impl TypeInfo for String {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        Ok(new_field(
            name,
            get_default_string_type(context.get_context()),
        ))
    }
}

impl TypeInfo for Box<str> {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        Ok(new_field(
            name,
            get_default_string_type(context.get_context()),
        ))
    }
}

impl TypeInfo for Arc<str> {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        Ok(new_field(
            name,
            get_default_string_type(context.get_context()),
        ))
    }
}

impl<const N: usize, T: TypeInfo> TypeInfo for [T; N] {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        let base_field = context.get_context().get_field::<T>("element")?;
        let n = i32::try_from(N)?;

        // TODO: allow to customize
        let data_type = if matches!(base_field.data_type, DataType::UInt8) {
            DataType::FixedSizeBinary(n)
        } else {
            DataType::FixedSizeList(Box::new(base_field), n)
        };

        Ok(Field {
            name: name.to_owned(),
            data_type,
            nullable: false,
            metadata: Default::default(),
        })
    }
}

fn get_list_field<T: TypeInfo>(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
    let larget_list = if let Some(LargeList(large_list)) = context.get_context().get() {
        *large_list
    } else {
        false
    };

    let base_field = context.get_context().get_field::<T>("element")?;

    Ok(Field {
        name: name.to_owned(),
        data_type: if larget_list {
            DataType::LargeList(Box::new(base_field))
        } else {
            DataType::List(Box::new(base_field))
        },
        nullable: false,
        metadata: Default::default(),
    })
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        get_list_field::<T>(name, context)
    }
}

impl<T: TypeInfo> TypeInfo for &[T] {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        get_list_field::<T>(name, context)
    }
}

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        let mut base_field = T::get_field(name, context)?;
        base_field.nullable = true;
        Ok(base_field)
    }
}

impl<K: TypeInfo, V: TypeInfo> TypeInfo for HashMap<K, V> {
    fn get_field(name: &str, context: ContextRef<'_>) -> Result<Field, Error> {
        let key_field = context.get_context().get_field::<K>("key")?;
        let value_field = context.get_context().get_field::<V>("value")?;
        let entry_field = new_field("entry", DataType::Struct(vec![key_field, value_field]));

        Ok(new_field(name, DataType::Map(Box::new(entry_field), false)))
    }
}

#[test]
fn examples() {
    assert_eq!(Context::new().get_data_type::<i64>(), Ok(DataType::Int64));
    assert_eq!(
        Context::new().get_data_type::<[u8; 8]>(),
        Ok(DataType::FixedSizeBinary(8))
    );
}
