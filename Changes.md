# Change log

## 0.2.0

- Implement `PartialEq` for `Array` and `View`, and `FieldMeta`
- Implement `Default` for `Field` and `FieldMeta`
- Place metadata in front of arrays in `StructArray::fields`, `DenseUnionArray::fields`
- Add `as_view` for `Array` and the array structs

## 0.1.0

Initial release to publish the arrow interop functionality of
[`serde_arrow`](https://github.com/chmp/serde_arrow) as a separate crate.

