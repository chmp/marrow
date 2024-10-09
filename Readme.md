# `marrow` - minimalist Arrow interop

[Crate info](https://crates.io/crates/marrow)
| [API docs](https://docs.rs/marrow/)

`marrow` allows building and viewing arrow arrays of different implementations using a unified
interface. The motivation behind `marrow` is to allow libraries to target multiple different arrow
versions simultaneously.

## License

```text
Copyright (c) 2024 Christopher Prohm

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Tests in `test_with_arrow` use snippets from the arrow documentation licensed under the Apache
Software License 2.0. Copies of the license and notice files can be found at
[`test_with_arrow/LICENSE.arrow.txt`](test_with_arrow/LICENSE.arrow.txt) and
[`test_with_arrow/NOTICE.arrow.txt`](test_with_arrow/NOTICE.arrow.txt).
