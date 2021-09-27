use std::{collections, hash};

use jsonrpsee_types::JsonValue;

pub use jsonrpsee_types::Subscription;

macro_rules! impl_xo_object {
    ($t:ty => $object_type:expr, $id:ty) => {
        impl XoObject for $t {
            const OBJECT_TYPE: &'static str = $object_type;
            type IdType = $id;
        }

        impl XoObjectId for $id {}
    };
}

/// Unique id of a virtual machine
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VmId(pub(crate) String);

impl From<VmId> for String {
    fn from(vm_id: VmId) -> String {
        vm_id.0
    }
}

impl From<VmId> for JsonValue {
    fn from(val: VmId) -> Self {
        JsonValue::String(val.0)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct VmOrSnapshotId(pub(crate) String);

impl From<SnapshotId> for VmOrSnapshotId {
    fn from(id: SnapshotId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

impl From<VmOrSnapshotId> for JsonValue {
    fn from(id: VmOrSnapshotId) -> Self {
        JsonValue::String(id.0)
    }
}

impl From<VmId> for VmOrSnapshotId {
    fn from(id: VmId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

/// Unique id of a virtual machine snapshot
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(pub(crate) String);

impl From<SnapshotId> for String {
    fn from(val: SnapshotId) -> Self {
        val.0
    }
}

impl From<SnapshotId> for JsonValue {
    fn from(val: SnapshotId) -> Self {
        JsonValue::String(val.0)
    }
}

/// Type representing snapshot of a VM
#[derive(serde::Deserialize, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub name_label: String,
    pub name_description: String,
}
impl_xo_object!(Snapshot => "VM-snapshot", SnapshotId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impossible {}

#[cfg(test)]
mod tests {
    #[test]
    fn debian() {
        let s = include_str!("../test_data/snapshot/debian_10.json");
        let debian_snapshot: super::Snapshot = serde_json::from_str(&s).unwrap();

        assert_eq!(debian_snapshot.id.0, "deadbeaf-dead-beaf-dead-beafdeadbea0");
        assert_eq!(debian_snapshot.name_label, "[XO My Backup Job] debian 10");
        assert_eq!(debian_snapshot.name_description, "");

        let s = include_str!("../test_data/snapshot/pfsense_2_5_1.json");
        let pfsense_snapshot: super::Snapshot = serde_json::from_str(&s).unwrap();

        assert_eq!(
            pfsense_snapshot.id.0,
            "deadbeaf-dead-beaf-dead-beafdeadbea1"
        );
        assert_eq!(
            pfsense_snapshot.name_label,
            "[XO My Backup Job] pfsense 2.5.1"
        );
        assert_eq!(pfsense_snapshot.name_description, "Foo description");
    }
}

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
