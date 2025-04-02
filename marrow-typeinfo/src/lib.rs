use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::Infallible,
    num::TryFromIntError,
    rc::Rc,
    sync::Arc,
};

use marrow::datatypes::{DataType, Field};

pub use marrow_typeinfo_derive::TypeInfo;

// TODO: include the path in context to allow overwrites
#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<TypeId, Rc<dyn Any>>,
}

struct DefaultStringType(DataType);

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

pub trait TypeInfo {
    fn get_field(name: &str, context: &Context) -> Result<Field, Error>;

    fn get_data_type(context: &Context) -> Result<DataType, Error> {
        Ok(Self::get_field("item", context)?.data_type)
    }
}

macro_rules! define_primitive {
    ($(($ty:ty, $dt:expr)),*) => {
        $(
            impl TypeInfo for $ty {
                fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
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
    ((), DataType::Null),
    (bool, DataType::Boolean),
    (u8, DataType::UInt8),
    (u16, DataType::UInt16),
    (u32, DataType::UInt32),
    (u64, DataType::UInt64),
    (i8, DataType::Int8),
    (i16, DataType::Int16),
    (i32, DataType::Int32),
    (i64, DataType::Int64)
);

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
    fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
        Ok(new_field(name, get_default_string_type(context)))
    }
}

impl TypeInfo for String {
    fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
        Ok(new_field(name, get_default_string_type(context)))
    }
}

impl TypeInfo for Box<str> {
    fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
        Ok(new_field(name, get_default_string_type(context)))
    }
}

impl TypeInfo for Arc<str> {
    fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
        Ok(new_field(name, get_default_string_type(context)))
    }
}

impl<const N: usize, T: TypeInfo> TypeInfo for [T; N] {
    fn get_field(name: &str, context: &Context) -> Result<Field, Error> {
        let base_type = T::get_data_type(context)?;
        let n = i32::try_from(N)?;

        // TODO: allow to customize
        let data_type = match base_type {
            DataType::UInt8 => DataType::FixedSizeBinary(n),
            base_type => DataType::FixedSizeList(
                Box::new(Field {
                    name: String::from("element"),
                    data_type: base_type,
                    nullable: false,
                    metadata: Default::default(),
                }),
                n,
            ),
        };

        Ok(Field {
            name: name.to_owned(),
            data_type,
            nullable: false,
            metadata: Default::default(),
        })
    }
}

#[test]
fn examples() {
    assert_eq!(
        <i64 as TypeInfo>::get_data_type(&Default::default()),
        Ok(DataType::Int64)
    );
    assert_eq!(
        <[u8; 8] as TypeInfo>::get_data_type(&Default::default()),
        Ok(DataType::FixedSizeBinary(8))
    );
}
