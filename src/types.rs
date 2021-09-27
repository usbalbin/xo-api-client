use std::{collections, hash};

use jsonrpsee_types::JsonValue;

pub use jsonrpsee_types::Subscription;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impossible {}

pub trait XoObjectId: serde::de::DeserializeOwned + Clone + Into<JsonValue> {}

pub trait XoObject: serde::de::DeserializeOwned {
    const OBJECT_TYPE: &'static str;
    type IdType: XoObjectId;
}

pub trait XoObjectMap: serde::de::DeserializeOwned {
    type Object: XoObject;
}

impl<T: XoObject> XoObjectMap for collections::BTreeMap<T::IdType, T>
where
    <T as XoObject>::IdType: Ord,
{
    type Object = T;
}

impl<T: XoObject> XoObjectMap for collections::HashMap<T::IdType, T>
where
    <T as XoObject>::IdType: Eq + hash::Hash,
{
    type Object = T;
}
