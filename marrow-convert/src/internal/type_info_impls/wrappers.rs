use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use marrow::datatypes::Field;

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for PhantomData<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let mut field = T::get_field(context)?;
        field.nullable = true;
        Ok(field)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for Box<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for Cell<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for RefCell<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for Mutex<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for RwLock<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for Rc<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: DefaultArrayType + ?Sized> DefaultArrayType for Arc<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<'a, T: DefaultArrayType + ToOwned + ?Sized + 'a> DefaultArrayType for Cow<'a, T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}
