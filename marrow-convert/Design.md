# Overall design

- M:N relationship between Rust and Arrow types
  - A single Rust type can be converted into different Arrow types
  - Different Rust types can be converted into the same Arrow type
  - E.g., `jiff::Timestamp` and `chrono::DateTime<chrono::Utc>` can both be converted to the Arrow
    `Timestamp` type
  - E.g., `jiff::Timestamp` can both be converted to the Arrow `Timestamp` and the Arrow `Utf8` typ
- Allow to fully specify the builders at compile time