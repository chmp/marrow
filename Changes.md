# Change log

## 0.2.0

- Rework map arrays to use explicit keys and values array to simplify interaction the underlying
  arrays
- Rework `StructArray` and `DenseUnionArray`: place metadata in front of arrays in
  `StructArray::fields`, `DenseUnionArray::fields`
- Add `MarrowError::new` and `MarrowError::with_cause`
- Add `as_view` for `Array` and the array structs
- Implement `PartialEq` for `Array` and `View`, and `FieldMeta`
- Implement `Default` for `Field` and `FieldMeta`

## 0.1.0

Initial release to publish the arrow interop functionality of
[`serde_arrow`](https://github.com/chmp/serde_arrow) as a separate crate.

