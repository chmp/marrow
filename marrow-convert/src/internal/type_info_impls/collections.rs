use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use marrow::datatypes::Field;

use crate::{
    Result,
    types::{Context, DefaultArrayType},
};

use super::utils::{new_list_field, new_map_field};

/// Map a vec to an Arrow List
impl<T: DefaultArrayType> DefaultArrayType for Vec<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `VecDeque` to an Arrow List
impl<T: DefaultArrayType> DefaultArrayType for VecDeque<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `LinkedList` to an Arrow List
impl<T: DefaultArrayType> DefaultArrayType for LinkedList<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BinaryHeap` to an Arrow List
impl<T: DefaultArrayType> DefaultArrayType for BinaryHeap<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BTreeSet` to an Arrow List
impl<T: DefaultArrayType> DefaultArrayType for BTreeSet<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `HashSet` to an Arrow List
impl<T: DefaultArrayType, S> DefaultArrayType for HashSet<T, S> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BTreeMap` to an Arrow Map
impl<K: DefaultArrayType, V: DefaultArrayType> DefaultArrayType for BTreeMap<K, V> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_map_field::<K, V>(context)
    }
}

/// Map a `HashMap` to an Arrow Map
impl<K: DefaultArrayType, V: DefaultArrayType> DefaultArrayType for HashMap<K, V> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_map_field::<K, V>(context)
    }
}
