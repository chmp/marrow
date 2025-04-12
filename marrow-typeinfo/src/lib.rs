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

mod ext;

/// Derive [TypeInfo] for a given type
///
/// Currently structs and enums with any type of lifetime parameters are supported.
pub use marrow_typeinfo_derive::TypeInfo;

pub type Result<T, E = Error> = std::result::Result<T, E>;

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

#[derive(Debug, Default)]
pub struct Options {
    data: HashMap<TypeId, Rc<dyn Any>>,
    overwrites: HashMap<String, Field>,
}

impl Options {
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

    pub fn with_default_string_type(mut self, data_type: DataType) -> Self {
        // TOOD: check for valid string type
        self.set(DefaultStringType(data_type));
        self
    }

    pub fn with_default_list_index_type(mut self, list_type: ListIndexType) -> Self {
        self.set(LargeList(matches!(list_type, ListIndexType::Int64)));
        self
    }

    pub fn overwrite(mut self, path: &str, field: Field) -> Self {
        self.overwrites.insert(path.to_owned(), field);
        self
    }
}

pub enum ListIndexType {
    Int32,
    Int64,
}

impl TryFrom<DataType> for ListIndexType {
    type Error = Error;

    fn try_from(value: DataType) -> std::result::Result<Self, Self::Error> {
        match value {
            DataType::Int32 => Ok(Self::Int32),
            DataType::Int64 => Ok(Self::Int64),
            dt => Err(Error(format!(
                "Cannot interpretr {dt:?} as a ListIndexType"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context<'a> {
    path: &'a str,
    name: &'a str,
    options: &'a Options,
}

impl<'a> Context<'a> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_path(&self) -> &str {
        self.path
    }

    pub fn get_options(&self) -> &Options {
        self.options
    }

    pub fn get_field<T: TypeInfo>(&self, name: &str) -> Result<Field> {
        self.nest(name, T::get_field)
    }

    pub fn nest<F: FnOnce(Context<'_>) -> Result<Field>>(
        &self,
        name: &str,
        scope: F,
    ) -> Result<Field> {
        let path = format!("{}.{}", self.path, name);

        if let Some(overwrite) = self.options.overwrites.get(&path) {
            let mut overwrite = overwrite.clone();
            overwrite.name = String::from(name);
            return Ok(overwrite);
        }

        let child_context = Context {
            path: &path,
            name,
            options: self.options,
        };

        scope(child_context)
    }
}

pub fn get_field<T: TypeInfo>(name: &str, options: &Options) -> Result<Field> {
    let context = Context {
        path: "$",
        name,
        options,
    };
    T::get_field(context)
}

pub fn get_data_type<T: TypeInfo>(options: &Options) -> Result<DataType> {
    Ok(get_field::<T>("item", options)?.data_type)
}

struct DefaultStringType(DataType);

struct LargeList(bool);

/// Get the Arrow type information for a given Rust type
///
/// The functions cannot be called directly. First construct a [Context], then call the
/// corresponding methods.
pub trait TypeInfo {
    /// See [crate::get_field]
    fn get_field(context: Context<'_>) -> Result<Field>;
}

macro_rules! define_primitive {
    ($(($ty:ty, $dt:expr),)*) => {
        $(
            impl TypeInfo for $ty {
                fn get_field(context: Context<'_>) -> Result<Field> {
                    Ok(Field {
                        name: context.get_name().to_owned(),
                        data_type: $dt,
                        ..Field::default()
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

impl<T: TypeInfo> TypeInfo for &T {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo> TypeInfo for &mut T {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl TypeInfo for () {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let _ = context;
        Ok(Field {
            name: context.get_name().to_owned(),
            data_type: DataType::Null,
            nullable: true,
            metadata: Default::default(),
        })
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

fn new_string_field(context: Context<'_>) -> Field {
    let ty = if let Some(DefaultStringType(ty)) = context.get_options().get() {
        ty.clone()
    } else {
        DataType::LargeUtf8
    };
    new_field(context.get_name(), ty)
}

impl TypeInfo for &str {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(new_string_field(context))
    }
}

impl TypeInfo for String {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        Ok(new_string_field(context))
    }
}

impl TypeInfo for Box<str> {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        Ok(new_string_field(context))
    }
}

impl TypeInfo for Arc<str> {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        Ok(new_string_field(context))
    }
}

impl<const N: usize, T: TypeInfo> TypeInfo for [T; N] {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        let base_field = context.get_field::<T>("element")?;
        let n = i32::try_from(N)?;

        // TODO: allow to customize
        let data_type = if matches!(base_field.data_type, DataType::UInt8) {
            DataType::FixedSizeBinary(n)
        } else {
            DataType::FixedSizeList(Box::new(base_field), n)
        };

        Ok(Field {
            name: context.get_name().to_owned(),
            data_type,
            nullable: false,
            metadata: Default::default(),
        })
    }
}

fn new_list_field<T: TypeInfo>(context: Context<'_>) -> Result<Field, Error> {
    let larget_list = if let Some(LargeList(large_list)) = context.get_options().get() {
        *large_list
    } else {
        false
    };

    let base_field = context.get_field::<T>("element")?;

    Ok(Field {
        name: context.get_name().to_owned(),
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
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        new_list_field::<T>(context)
    }
}

impl<T: TypeInfo> TypeInfo for &[T] {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        new_list_field::<T>(context)
    }
}

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        let mut base_field = T::get_field(context)?;
        base_field.nullable = true;
        Ok(base_field)
    }
}

impl<K: TypeInfo, V: TypeInfo> TypeInfo for HashMap<K, V> {
    fn get_field(context: Context<'_>) -> Result<Field, Error> {
        let key_field = context.get_field::<K>("key")?;
        let value_field = context.get_field::<V>("value")?;
        let entry_field = new_field("entry", DataType::Struct(vec![key_field, value_field]));

        Ok(new_field(
            context.get_name(),
            DataType::Map(Box::new(entry_field), false),
        ))
    }
}

macro_rules! impl_tuples {
    ($( ( $($name:ident,)* ), )*) => {
        $(
            impl<$($name: TypeInfo),*> TypeInfo for ( $($name,)* ) {
                #[allow(unused_assignments)]
                fn get_field(context: Context<'_>) -> Result<Field> {
                    let mut idx = 0;
                    let mut fields = Vec::new();
                    $(
                        fields.push(context.get_field::<$name>(&idx.to_string())?);
                        idx += 1;
                    )*

                    Ok(Field {
                        name: context.get_name().to_owned(),
                        data_type: DataType::Struct(fields),
                        ..Field::default()
                    })
                }
            }
        )*
    };
}

impl_tuples!(
    (A,),
    (A, B,),
    (A, B, C,),
    (A, B, C, D,),
    (A, B, C, D, E,),
    (A, B, C, D, E, F,),
    (A, B, C, D, E, F, G,),
    (A, B, C, D, E, F, G, H,),
    (A, B, C, D, E, F, G, H, I,),
    (A, B, C, D, E, F, G, H, I, J,),
    (A, B, C, D, E, F, G, H, I, J, K,),
    (A, B, C, D, E, F, G, H, I, J, K, L,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,),
);

#[test]
fn examples() {
    assert_eq!(
        get_data_type::<i64>(&Options::default()),
        Ok(DataType::Int64)
    );
    assert_eq!(
        get_data_type::<[u8; 8]>(&Options::default()),
        Ok(DataType::FixedSizeBinary(8))
    );
}
