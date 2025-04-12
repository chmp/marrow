use std::{
    num::NonZero,
    ops::{Bound, Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
    sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicU8, AtomicU16, AtomicU32,
        AtomicU64,
    },
    time::{Duration, SystemTime},
};

use marrow::datatypes::{DataType, Field, TimeUnit, UnionMode};

use crate::{Context, Result, TypeInfo};

use super::utils::new_string_field;

impl TypeInfo for String {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(new_string_field(context))
    }
}

/// Map an option to a nullable field
impl<T: TypeInfo> TypeInfo for Option<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let mut base_field = T::get_field(context)?;
        base_field.nullable = true;
        Ok(base_field)
    }
}

/// Map a `Result` to an Arrow Union with `Ok` and `Err` variants
impl<T: TypeInfo, E: TypeInfo> TypeInfo for Result<T, E> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let ok = context.get_field::<T>("Ok")?;
        let err = context.get_field::<E>("Err")?;

        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Union(vec![(0, ok), (1, err)], UnionMode::Dense),
            ..Default::default()
        })
    }
}

/// Map a `Range` to an Arrow `FixedSizeList(.., 2)`
impl<T: TypeInfo> TypeInfo for Range<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        <[T; 2]>::get_field(context)
    }
}

/// Map a `RangeInclusive` to an Arrow `FixedSizeList(.., 2)`
impl<T: TypeInfo> TypeInfo for RangeInclusive<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        <[T; 2]>::get_field(context)
    }
}

/// Map a `RangeTo` to the index type
impl<T: TypeInfo> TypeInfo for RangeTo<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

/// Map a `RangeToInclusive` to the index type
impl<T: TypeInfo> TypeInfo for RangeToInclusive<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

/// Map a `RangeFrom` to the index type
impl<T: TypeInfo> TypeInfo for RangeFrom<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        T::get_field(context)
    }
}

/// Map a `Bound` to an Arrow Union with variants `Included`, `Excluded`, `Unbounded`
impl<T: TypeInfo> TypeInfo for Bound<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        let included = context.get_field::<T>("Included")?;
        let excluded = context.get_field::<T>("Excluded")?;
        let unbounded = context.get_field::<()>("Unbounded")?;

        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Union(
                vec![(0, included), (1, excluded), (2, unbounded)],
                UnionMode::Dense,
            ),
            ..Default::default()
        })
    }
}

macro_rules! impl_nonzero {
    ($($ty:ident),* $(,)?) => {
        $(
            impl TypeInfo for NonZero<$ty> {
                fn get_field(context: Context<'_>) -> Result<Field> {
                    <$ty>::get_field(context)
                }
            }
        )*
    };
}

impl_nonzero!(u8, u16, u32, u64, i8, i16, i32, i64);

macro_rules! impl_atomic {
    ($(($atomic:ident, $ty:ident)),* $(,)?) => {
        $(
            impl TypeInfo for $atomic {
                fn get_field(context: Context<'_>) -> Result<Field> {
                    $ty::get_field(context)
                }
            }
        )*
    };
}

impl_atomic!(
    (AtomicBool, bool),
    (AtomicI8, i8),
    (AtomicI16, i16),
    (AtomicI32, i32),
    (AtomicI64, i64),
    (AtomicU8, u8),
    (AtomicU16, u16),
    (AtomicU32, u32),
    (AtomicU64, u64),
);

impl TypeInfo for Duration {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Duration(TimeUnit::Millisecond),
            ..Default::default()
        })
    }
}

impl TypeInfo for SystemTime {
    fn get_field(context: Context<'_>) -> Result<Field> {
        Ok(Field {
            name: String::from(context.get_name()),
            data_type: DataType::Timestamp(TimeUnit::Millisecond, None),
            ..Default::default()
        })
    }
}
