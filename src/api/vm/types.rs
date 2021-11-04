use std::{collections::BTreeMap, net::Ipv4Addr};

use jsonrpsee_types::{DeserializeOwned, JsonValue};

use crate::{declare_id_type, impl_xo_object, types::XoObject};

/// Type representing a VM
///
/// Note that the "other" property contains a lot of different data. In the Xen Orchestra the user may
/// even add additional values. For this reason the struct is made generic over the type `O`.
/// See the trait [`OtherInfo`] for more info
///
/// Also see https://github.com/vatesfr/xen-orchestra/blob/a505cd9567233aab7ca6488b2fb8a0b6c610fa08/packages/xo-server/src/xapi-object-to-xo.mjs#L273
#[derive(serde::Deserialize, Debug)]
pub struct Vm<O> {
    pub id: VmId,
    pub name_label: String,
    pub name_description: String,
    pub power_state: PowerState,

    #[serde(rename = "$pool")]
    pub pool: String,
    pub tags: Vec<String>,

    #[serde(default)]
    pub(crate) addresses: BTreeMap<String, String>,

    #[serde(deserialize_with = "map_from_optional_map", default)]
    pub os_version: BTreeMap<String, String>,

    pub other: O,
}

impl<O: DeserializeOwned> XoObject for Vm<O> {
    const OBJECT_TYPE: &'static str = "VM";
    type IdType = VmId;
}

/// Type describing power state of VM
#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq)]
pub enum PowerState {
    Running,
    Halted,
    Suspended,
    Paused,
}

impl<'a, O: serde::de::DeserializeOwned> Vm<O> {
    /// Check if VM is running.
    pub fn is_running(&self) -> bool {
        matches!(self.power_state, PowerState::Running)
    }

    /// Try to guess OS distro of VM
    ///
    /// Note: This only works for running VMs, returns `None` when distro
    /// can not be determined.
    pub fn distro(&self) -> Option<&str> {
        match &self.os_version.get("distro") {
            Some(distro) => Some(distro),
            None if self.os_version.contains_key("spmajor") => Some("windows"),
            None => None,
        }
    }

    /// Get iterator of all valid IPv4 addresses for VM.
    ///
    /// Note: This only works for running VMs, returns empty iterator otherwise
    pub fn ipv4_addresses(&self) -> impl Iterator<Item = Ipv4Addr> + '_ {
        self.addresses
            .iter()
            .filter(|(tag, _ip)| tag.contains("ipv4"))
            .flat_map(|(_tag, ip)| ip.split(' '))
            .filter_map(move |ip| match ip.parse() {
                Ok(ip) => Some(ip),
                Err(e) => {
                    log::warn!("Invalid IP found for VM: {}, {:?}", self.name_label, e);
                    None
                }
            })
    }
}

fn map_from_optional_map<'de, D>(des: D) -> Result<BTreeMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let option: Option<_> = serde::de::Deserialize::deserialize(des)?;
    Ok(option.unwrap_or_default())
}

/// This is the "other" section of VM from XO.
///
/// This secton contains some XO specific data like backup related values and what template
/// the VM was created from. Other that that there is also the "Custom Fields" from the
/// "Advanced" tab of the VM in XO. However note that all fields added from there will have
/// "XenCenter.CustomFields." as prefix in their key.
///
/// A type that can be deserialized from a flat string to string object.
pub trait OtherInfo: serde::de::DeserializeOwned {}

impl OtherInfo for BTreeMap<String, String> {}
impl OtherInfo for std::collections::HashMap<String, String> {}

declare_id_type! {
    /// Unique id of a virtual machine
    pub struct VmId;
}

declare_id_type! {
    /// Unique id of a virtual machine snapshot
    pub struct SnapshotId;
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

/// Type representing snapshot of a VM
#[derive(serde::Deserialize, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub name_label: String,
    pub name_description: String,
}
impl_xo_object!(Snapshot => "VM-snapshot", SnapshotId);
