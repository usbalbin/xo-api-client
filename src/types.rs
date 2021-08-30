use std::collections::HashMap;

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

/// Unique id of a virtual machine
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VmId(pub(crate) String);

impl From<VmId> for String {
    fn from(vm_id: VmId) -> String {
        vm_id.0
    }
}

impl_to_json_value!(VmId);

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct VmOrSnapshotId(pub(crate) String);

impl From<SnapshotId> for VmOrSnapshotId {
    fn from(id: SnapshotId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

impl_to_json_value!(VmOrSnapshotId);

impl From<VmId> for VmOrSnapshotId {
    fn from(id: VmId) -> Self {
        VmOrSnapshotId(id.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TemplateId(pub(crate) String);

/// See https://github.com/vatesfr/xen-orchestra/blob/a505cd9567233aab7ca6488b2fb8a0b6c610fa08/packages/xo-server/src/xapi-object-to-xo.mjs#L273
#[derive(serde::Deserialize)]
pub struct Template {
    pub id: TemplateId,

    #[serde(rename = "VIFs")]
    pub(crate) vifs: HashMap<usize, VifId>,

    #[serde(rename = "$VBDs")]
    pub(crate) vbds: Vec<VbdId>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CloudConfigId(pub(crate) String);
impl_to_json_value!(CloudConfigId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NetworkConfigId(pub(crate) String);
impl_to_json_value!(NetworkConfigId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SrId(pub(crate) String);
impl_to_json_value!(SrId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NetworkId(pub(crate) String);
impl_to_json_value!(NetworkId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VdiId(pub(crate) String);
impl_to_json_value!(VdiId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VbdId(pub(crate) String);
impl_to_json_value!(VbdId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct VifId(pub(crate) String);
impl_to_json_value!(VifId);

/// Unique id of a virtual machine snapshot
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(pub(crate) String);
impl_to_json_value!(SnapshotId);

impl From<SnapshotId> for String {
    fn from(val: SnapshotId) -> Self {
        val.0
    }
}

/// Type representing snapshot of a VM
#[derive(serde::Deserialize, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub name_label: String,
    pub name_description: String,
}

#[derive(serde::Serialize)]
pub(crate) struct PartialVdi {
    pub(crate) name_description: String,
    pub(crate) name_label: String,
    pub(crate) size: usize,

    #[serde(rename = "$SR")]
    pub(crate) sr: SrId,
}

#[derive(serde::Serialize)]
pub(crate) struct PartialVif {
    pub(crate) network: NetworkId,
}

#[derive(serde::Deserialize)]
pub struct Vdi {
    id: VdiId,

    //type: 'VDI' | 'VDI-unmanaged' | 'VDI-snapshot',
    missing: bool,
    pub(crate) name_description: String,
    pub(crate) name_label: String,
    //parent: Vdi or VdiUnmanaged?,
    pub(crate) size: usize,
    //snapshots: Vec<VdiSnapshot>,
    //tags: Vec<String>,
    usage: usize,
    //VDI_type: String,
    //current_operations: Vec<IdOfOperations>,
    #[serde(rename = "$SR")]
    pub(crate) sr: SrId,

    #[serde(rename = "$VBDs")]
    vbds: Vec<VbdId>,
}

#[derive(serde::Deserialize)]
pub struct Vbd {
    id: VbdId,
    //type: 'VBD',
    attached: bool,
    bootable: bool,
    device: Option<String>, //xvda, xvdb etc.
    pub(crate) is_cd_drive: bool,
    pub(crate) position: usize,
    read_only: bool,

    #[serde(rename = "VDI")]
    pub(crate) vdi: VdiId,

    #[serde(rename = "VM")]
    vm: VmId,
}

#[derive(serde::Deserialize)]
pub struct Vif {
    id: VifId,
    //type: 'VIF',

    //allowedIpv4Addresses: obj.ipv4_allowed,
    //allowedIpv6Addresses: obj.ipv6_allowed,
    attached: bool,
    //device: obj.device, // TODO: should it be cast to a number?
    //lockingMode: obj.locking_mode,
    //MAC: String,
    //MTU: usize,
    //other_config: obj.other_config,

    // See: https://xapi-project.github.io/xen-api/networking.html
    txChecksumming: bool,

    // in kB/s
    //rateLimit:
    #[serde(rename = "$network")]
    pub(crate) network: NetworkId,

    #[serde(rename = "$VM")]
    vm: VmId,
}

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
