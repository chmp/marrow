use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

use marrow::datatypes::{DataType, Field};

use crate::{Error, Result};

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

impl Context<'_> {
    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_path(&self) -> &str {
        self.path
    }

    pub fn get_options(&self) -> &Options {
        self.options
    }

    pub fn get_field<T: DefaultArrayType>(&self, name: &str) -> Result<Field> {
        self.nest(name, T::get_field)
    }

    /// Call a function with a context for nested field
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

pub fn get_field<T: DefaultArrayType>(name: &str, options: &Options) -> Result<Field> {
    let context = Context {
        path: "$",
        name,
        options,
    };
    T::get_field(context)
}

pub fn get_data_type<T: DefaultArrayType>(options: &Options) -> Result<DataType> {
    Ok(get_field::<T>("item", options)?.data_type)
}

pub struct DefaultStringType(pub DataType);

pub struct LargeList(pub bool);

/// Get the Arrow type information for a given Rust type
///
/// The functions cannot be called directly. First construct a [Context], then call the
/// corresponding methods.
pub trait DefaultArrayType {
    /// See [get_field]
    fn get_field(context: Context<'_>) -> Result<Field>;
}
