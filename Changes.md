# Change log

## 0.2.0

- Rework map arrays to use explicit keys and values array to simplify interaction the underlying
  arrays
- Implement sparse unions, rename `DenseUnion` to `Union` and change offsets to be `Option<Vec<i32>>`
- Implement interval arrays and the `Interval` data type
- Implement run encoded array
- Rename `Dictionary::indices` to `Dictionary::keys`
- Rework `StructArray` and `UnionArray`: place metadata in front of arrays in `StructArray::fields`,
  `UnionArray::fields`
- Add `MarrowError::new` and `MarrowError::with_cause`
- Add `as_view` for `Array` and the array structs
- Implement `PartialEq` for `Array` and `View`, and `FieldMeta`
- Implement `Default` for `Field` and `FieldMeta`
- Remove the sorted flag from the dictionary `DataType` it is not supported by `arrow`
- Add `Array::data_type()` and  `View::data_type()`

## 0.1.0

Initial release to publish the arrow interop functionality of
[`serde_arrow`](https://github.com/chmp/serde_arrow) as a separate crate.

