use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use marrow::datatypes::Field;

use crate::{Context, Result, TypeInfo};

impl<T: TypeInfo + ?Sized> TypeInfo for PhantomData<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let mut field = T::get_field(context)?;
        field.nullable = true;
        Ok(field)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for Box<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for Cell<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for RefCell<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for Mutex<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for RwLock<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for Rc<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<T: TypeInfo + ?Sized> TypeInfo for Arc<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

impl<'a, T: TypeInfo + ToOwned + ?Sized + 'a> TypeInfo for Cow<'a, T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}
