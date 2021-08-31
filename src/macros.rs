#[macro_export]
macro_rules! procedure_args {
    ($($key:expr => $value:expr,)+) => (procedure_args!($($key => $value),+));

    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut map = ::std::collections::BTreeMap::new();
            $(
                let _ = map.insert($key, serde_json::Value::from($value));
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! procedure_object {
    ($($key:expr => $value:expr,)+) => (procedure_args!($($key => $value),+));

    ($($key:literal => $value:expr),*) => (procedure_object!($(String::from($key) => $value),+));

    ($($key:expr => $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut map = ::serde_json::Map::new();
            $(
                let _ = map.insert($key, serde_json::Value::from($value));
            )*
            map
        }
    };
}
