use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use marrow::datatypes::Field;

use crate::{Context, Result, TypeInfo};

use super::utils::{new_list_field, new_map_field};

/// Map a vec to an Arrow List
impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `VecDeque` to an Arrow List
impl<T: TypeInfo> TypeInfo for VecDeque<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `LinkedList` to an Arrow List
impl<T: TypeInfo> TypeInfo for LinkedList<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BinaryHeap` to an Arrow List
impl<T: TypeInfo> TypeInfo for BinaryHeap<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BTreeSet` to an Arrow List
impl<T: TypeInfo> TypeInfo for BTreeSet<T> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `HashSet` to an Arrow List
impl<T: TypeInfo, S> TypeInfo for HashSet<T, S> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_list_field::<T>(context)
    }
}

/// Map a `BTreeMap` to an Arrow Map
impl<K: TypeInfo, V: TypeInfo> TypeInfo for BTreeMap<K, V> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_map_field::<K, V>(context)
    }
}

/// Map a `HashMap` to an Arrow Map
impl<K: TypeInfo, V: TypeInfo> TypeInfo for HashMap<K, V> {
    fn get_field(context: Context<'_>) -> Result<Field> {
        new_map_field::<K, V>(context)
    }
}
