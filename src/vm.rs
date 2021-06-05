use std::{collections::BTreeMap, net::Ipv4Addr};

use crate::VmId;

/// Type representing a VM
///
/// Note that the "other" property contains a lot of different data. In the Xen Orchestra the user may
/// even add additional values. For this reason the struct is made generic over the type `O`.
/// See the trait [`OtherInfo`] for more info
#[derive(serde::Deserialize, Debug)]
pub struct Vm<O> {
    pub id: VmId,
    pub name_label: String,
    pub name_description: String,
    pub power_state: PowerState,

    #[serde(rename = "$pool")]
    pub pool: String,
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    addresses: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub os_version: Option<BTreeMap<String, String>>,

    pub other: O,
}

/// Type describing power state of VM
#[derive(Debug, Clone, Copy, serde::Deserialize)]
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
        match &self.os_version {
            Some(os_version) => match os_version.get("distro") {
                Some(distro) => Some(distro),
                None if os_version.contains_key("spmajor") => Some("windows"),
                None => None,
            },
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

/// This is the "other" section of VM from XO.
///
/// This secton contains some XO specific data like backup related values and what template
/// the VM was created from. Other that that there is also the "Custom Fields" from the
/// "Advanced" tab of the VM in XO. However note that all fields added from there will have
/// "XenCenter.CustomFields." as prefix in their key.
///
/// A type that can be deserialized from a flat string to string object.
pub trait OtherInfo: serde::de::DeserializeOwned {}
