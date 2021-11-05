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
            let mut map = ::serde_json::Map::<String, JsonValue>::new();
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

#[macro_export]
macro_rules! impl_to_json_value {
    ($t:ty) => {
        impl From<$t> for ::serde_json::Value {
            fn from(val: $t) -> Self {
                Self::String(val.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_str {
    ($t:path) => {
        impl From<&str> for $t {
            fn from(s: &str) -> Self {
                $t(s.to_string())
            }
        }
        impl From<String> for $t {
            fn from(s: String) -> Self {
                $t(s)
            }
        }
        impl_to_json_value!($t);
    };
}

#[macro_export]
macro_rules! impl_xo_object {
    ($t:ty => $object_type:expr, $id:ty) => {
        impl crate::types::XoObject for $t {
            const OBJECT_TYPE: &'static str = $object_type;
            type IdType = $id;
        }
    };
}

#[macro_export]
macro_rules! declare_id_type {
    (
        $(#[$meta:meta])*
        $v:vis struct $t:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash)]
        #[serde(transparent)]
        $v struct $t(pub(crate) String);

        impl crate::types::XoObjectId for $t {}

        crate::impl_to_json_value!($t);
    };
}
