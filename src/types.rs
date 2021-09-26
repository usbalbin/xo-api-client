use std::collections::{self, BTreeMap};
use std::hash;

use jsonrpsee_types::JsonValue;

pub use jsonrpsee_types::Subscription;

macro_rules! impl_to_json_value {
    ($t:ty) => {
        impl From<$t> for JsonValue {
            fn from(val: $t) -> Self {
                JsonValue::String(val.0)
            }
        }
    };
}

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

impl_from_str!(VmId);

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct VmOrSnapshotId(pub(crate) String);

impl From<SnapshotId> for VmOrSnapshotId {
    fn from(id: SnapshotId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

impl_from_str!(VmOrSnapshotId);

impl From<VmId> for VmOrSnapshotId {
    fn from(id: VmId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TemplateId(pub(crate) String);

impl_from_str!(TemplateId);

/// See https://github.com/vatesfr/xen-orchestra/blob/a505cd9567233aab7ca6488b2fb8a0b6c610fa08/packages/xo-server/src/xapi-object-to-xo.mjs#L273
#[non_exhaustive]
#[derive(serde::Deserialize)]
pub struct Template {
    pub id: TemplateId,

    #[serde(rename = "VIFs")]
    pub(crate) vifs: Vec<VifId>,

    #[serde(rename = "$VBDs")]
    pub(crate) vbds: Vec<VbdId>,
}
impl_xo_object!(Template => "VM-template", TemplateId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CloudConfigId(pub(crate) String);
impl_from_str!(CloudConfigId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NetworkConfigId(pub(crate) String);
impl_from_str!(NetworkConfigId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SrId(pub(crate) String);
impl_from_str!(SrId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NetworkId(pub(crate) String);
impl_from_str!(NetworkId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VdiId(pub(crate) String);
impl_from_str!(VdiId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VbdId(pub(crate) String);
impl_from_str!(VbdId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VifId(pub(crate) String);
impl_from_str!(VifId);

/// Unique id of a virtual machine snapshot
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(pub(crate) String);
impl_from_str!(SnapshotId);

impl From<SnapshotId> for String {
    fn from(val: SnapshotId) -> Self {
        val.0
    }
}

/// Type representing snapshot of a VM
#[non_exhaustive]
#[derive(serde::Deserialize, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub name_label: String,
    pub name_description: String,
}
impl_xo_object!(Snapshot => "VM-snapshot", SnapshotId);

#[non_exhaustive]
#[derive(serde::Serialize)]
pub struct PartialVdi {
    pub(crate) name_description: String,
    pub(crate) name_label: String,
    pub(crate) size: usize,

    #[serde(rename = "$SR")]
    pub(crate) sr: SrId,
}

#[non_exhaustive]
#[derive(serde::Serialize)]
pub struct NewVdi {
    
}

impl PartialVdi {
    pub fn new(name_label: String, size: usize, sr: SrId) -> Self {
        PartialVdi {
            name_label,
            size,
            sr,
            name_description: String::new(),
        }
    }
}

#[non_exhaustive]
#[derive(serde::Serialize)]
pub struct PartialVif {
    pub(crate) network: NetworkId,
}

impl PartialVif {
    pub fn new(network: NetworkId) -> Self {
        PartialVif { network }
    }
}

#[non_exhaustive]
#[derive(serde::Deserialize)]
pub struct Vdi {
    pub id: VdiId,

    //type: 'VDI' | 'VDI-unmanaged' | 'VDI-snapshot',
    pub missing: bool,
    pub name_description: String,
    pub name_label: String,
    //parent: Vdi or VdiUnmanaged?,
    pub size: usize,
    // snapshots: Vec<VdiSnapshot>,
    pub tags: Vec<String>,
    pub usage: usize,
    //VDI_type: String,
    // current_operations: Vec<IdOfOperations>,
    #[serde(rename = "$SR")]
    pub sr: SrId,

    #[serde(rename = "$VBDs")]
    pub vbds: Vec<VbdId>,
}
impl_xo_object!(Vdi => "VDI", VdiId);

#[non_exhaustive]
#[derive(serde::Deserialize)]
pub struct Vbd {
    pub id: VbdId,
    //type: 'VBD',
    pub attached: bool,
    pub bootable: bool,
    pub device: Option<String>, //xvda, xvdb etc.
    pub is_cd_drive: bool,

    #[serde(with = "serde_with::rust::display_fromstr")]
    pub position: usize,
    pub read_only: bool,

    #[serde(rename = "VDI")]
    pub vdi: VdiId,

    #[serde(rename = "VM")]
    pub vm: VmId,
}
impl_xo_object!(Vbd => "VBD", VbdId);

#[non_exhaustive]
#[derive(serde::Deserialize)]
pub struct Vif {
    pub id: VifId,
    //type: 'VIF',

    //allowedIpv4Addresses: obj.ipv4_allowed,
    //allowedIpv6Addresses: obj.ipv6_allowed,
    pub attached: bool,
    //device: obj.device, // (comment by XO) TODO: should it be cast to a number?
    //lockingMode: obj.locking_mode,
    #[serde(rename = "MAC")]
    pub mac: String,

    #[serde(rename = "MTU")]
    pub mtu: u16,
    //other_config: obj.other_config,

    // // See: https://xapi-project.github.io/xen-api/networking.html
    // txChecksumming: bool,

    // in kB/s
    //rateLimit:
    #[serde(rename = "$network")]
    pub(crate) network: NetworkId,

    #[serde(rename = "$VM")]
    pub vm: VmId,
}
impl_xo_object!(Vif => "VIF", VifId);

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

impl<T: XoObject> XoObjectMap for BTreeMap<T::IdType, T> where
    <T as XoObject>::IdType: Ord
{
    type Object = T;
}

impl<T: XoObject> XoObjectMap for collections::HashMap<T::IdType, T> where
    <T as XoObject>::IdType: Eq + hash::Hash
{
    type Object = T;
}
