#![cfg_attr(any(), rustfmt::skip)]

macro_rules! define_test_module {
    ($feature:literal, $mod:ident, $array_mod:ident, $schema_mod:ident, $($test_mod:ident),* $(,)?) => {
        #[cfg(all(test, feature = $feature))]
        mod $mod {
            $(
                mod $test_mod {
                    #[allow(unused)]
                    use { $array_mod as arrow_array, $schema_mod as arrow_schema };
                    
                    include!(concat!("tests/", stringify!($test_mod), ".rs"));
                }
            )*
        }
    };
}

// arrow-version:insert: define_test_module!("arrow-{version}", arrow_{version}, arrow_array_{version}, arrow_schema_{version}, utils, arrays, data_types, union_arrays);
define_test_module!("arrow-53", arrow_53, arrow_array_53, arrow_schema_53, utils, arrays, data_types, union_arrays);
define_test_module!("arrow-52", arrow_52, arrow_array_52, arrow_schema_52, utils, arrays, data_types, union_arrays);
define_test_module!("arrow-51", arrow_51, arrow_array_51, arrow_schema_51, utils, arrays, data_types);
define_test_module!("arrow-50", arrow_50, arrow_array_50, arrow_schema_50, utils, arrays, data_types);
