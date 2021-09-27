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
            let mut map = ::serde_json::Map::<String, ::serde_json::Value>::new();
            $(
                let _ = map.insert($key, $value.into());
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! struct_to_map {
    (let $var:ident = $s:expr) => {
        // TODO: Find a better solution to this. Currently we go from
        // struct -> JsonValue ->
        // serde::Map -> BTreeMap<&str, JsonValue>
        // which is less than optimal...
        let __params = ::serde_json::to_value($s).unwrap();
        let $var = match &__params {
            ::serde_json::Value::Object(params) => params
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect::<::std::collections::BTreeMap<&str, ::serde_json::Value>>(),
            _ => unreachable!(),
        };
    };
}
