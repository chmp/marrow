# Change log

## 0.2.0

- Place metadata in front of arrays in `StructArray::fields`, `DenseUnionArray::fields`
- Rework map arrays to use explicit keys and values array to simplify interaction the underlying
  arrays
- Implement `PartialEq` for `Array` and `View`, and `FieldMeta`
- Implement `Default` for `Field` and `FieldMeta`
- Add `as_view` for `Array` and the array structs
- Add `MarrowError::new` and `MarrowError::with_cause`

## 0.1.0

Initial release to publish the arrow interop functionality of
[`serde_arrow`](https://github.com/chmp/serde_arrow) as a separate crate.

