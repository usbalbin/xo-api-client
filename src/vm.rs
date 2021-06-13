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

    #[serde(default)]
    addresses: BTreeMap<String, String>,

    #[serde(deserialize_with = "map_from_optional_map", default)]
    pub os_version: BTreeMap<String, String>,

    pub other: O,
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

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashMap},
        iter::FromIterator,
        net::Ipv4Addr,
    };

    use crate::vm::PowerState;

    #[test]
    fn debian() {
        let debian = file_to_vm("test_data/vm/debian_10.json").0;

        assert_eq!(&debian.id.0, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
        assert_eq!(
            debian.addresses,
            slice_to_map::<BTreeMap<_, _>>(&[
                ("0/ipv4/0", "10.0.1.52"),
                ("0/ipv6/0", "fe80::dead:beaf:dead:beaf")
            ])
        );
        assert_eq!(
            debian.ipv4_addresses().collect::<Vec<_>>(),
            vec!["10.0.1.52".parse::<Ipv4Addr>().unwrap()]
        );
        assert!(debian.is_running());
        assert_eq!(debian.power_state, PowerState::Running);
        assert_eq!(debian.name_label, "debian 10");
        assert_eq!(debian.name_description, "Some description");
        assert_eq!(debian.tags, vec!["Test"]);
        assert_eq!(
            debian.os_version,
            slice_to_map(&[
                ("distro", "debian"),
                ("major", "10"),
                ("minor", "10"),
                ("name", "Debian GNU/Linux 10 (buster)"),
                ("uname", "4.19.0-13-amd64")
            ])
        );
        assert_eq!(debian.distro().unwrap(), "debian");
        assert_eq!(debian.pool, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
    }

    #[test]
    fn pfsense() {
        let pfsense = file_to_vm("test_data/vm/pfsense_2_5_1.json").1;

        assert_eq!(&pfsense.id.0, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
        assert_eq!(
            pfsense.addresses,
            slice_to_map::<BTreeMap<_, _>>(&[
                ("0/ipv4/0", "10.0.0.13 10.0.0.12 10.0.0.16 10.0.0.7"),
                ("0/ipv4/1", "10.0.0.12"),
                ("0/ipv4/2", "10.0.0.16"),
                ("0/ipv4/3", "10.0.0.7"),
                ("1/ipv4/0", "192.168.71.2 192.168.71.1"),
                ("1/ipv4/1", "192.168.71.1"),
                ("2/ipv4/0", "192.168.72.2 192.168.72.1"),
                ("2/ipv4/1", "192.168.72.1")
            ])
        );
        assert_eq!(
            pfsense.ipv4_addresses().collect::<Vec<_>>(),
            vec![
                "10.0.0.13",
                "10.0.0.12",
                "10.0.0.16",
                "10.0.0.7",
                "10.0.0.12",
                "10.0.0.16",
                "10.0.0.7",
                "192.168.71.2",
                "192.168.71.1",
                "192.168.71.1",
                "192.168.72.2",
                "192.168.72.1",
                "192.168.72.1"
            ]
            .iter()
            .map(|x| x.parse::<Ipv4Addr>().unwrap())
            .collect::<Vec<_>>()
        );
        assert!(pfsense.is_running());
        assert_eq!(pfsense.power_state, PowerState::Running);
        assert_eq!(pfsense.name_label, "pfsense 2.5.1");
        assert_eq!(pfsense.name_description, "Foo description");
        assert_eq!(pfsense.tags, vec!["pfsense", "Test"]);
        assert_eq!(
            pfsense.os_version,
            slice_to_map(&[
                ("distro", "FreeBSD"),
                ("name", "FreeBSD 12.2-STABLE"),
                ("uname", "12.2-STABLE")
            ])
        );
        assert_eq!(pfsense.distro().unwrap(), "FreeBSD");
        assert_eq!(pfsense.pool, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
    }

    #[test]
    fn ubuntu() {
        let ubuntu = file_to_vm("test_data/vm/ubuntu_18_04.json").1;

        assert_eq!(&ubuntu.id.0, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
        assert_eq!(
            ubuntu.addresses,
            slice_to_map::<BTreeMap<_, _>>(&[
                ("0/ipv4/0", "10.0.3.25"),
                ("0/ipv6/0", "fe80::dead:beaf:dead:beaf")
            ])
        );
        assert_eq!(
            ubuntu.ipv4_addresses().collect::<Vec<_>>(),
            vec!["10.0.3.25".parse::<Ipv4Addr>().unwrap()]
        );
        assert!(ubuntu.is_running());
        assert_eq!(ubuntu.power_state, PowerState::Running);
        assert_eq!(ubuntu.name_label, "ubuntu 18.04");
        assert_eq!(ubuntu.name_description, "Ubuntu Linux (64-bit)");
        assert_eq!(ubuntu.tags, vec!["Important", "Other tag"]);
        assert_eq!(
            ubuntu.os_version,
            slice_to_map(&[
                ("distro", "ubuntu"),
                ("major", "18"),
                ("minor", "04"),
                ("name", "Ubuntu 18.04.5 LTS"),
                ("uname", "4.15.0-136-generic")
            ])
        );
        assert_eq!(ubuntu.distro().unwrap(), "ubuntu");
        assert_eq!(ubuntu.pool, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
    }

    #[test]
    fn windows() {
        let windows = file_to_vm("test_data/vm/windows_10.json").0;

        assert_eq!(&windows.id.0, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
        assert_eq!(
            windows.addresses,
            slice_to_map::<BTreeMap<_, _>>(&[
                ("0/ipv4/0", "192.168.7.42"),
                ("0/ipv6/0", "fe80:0000:0000:0000:dead:beaf:dead:beaa"),
                ("1/ipv4/0", "192.168.8.42"),
                ("1/ipv6/0", "fe80:0000:0000:0000:dead:beaf:dead:beab"),
                ("2/ipv4/0", "192.168.9.42"),
                ("2/ipv6/0", "fe80:0000:0000:0000:dead:beaf:dead:beac"),
                ("3/ipv4/0", "169.254.149.176"),
                ("3/ipv6/0", "fe80:0000:0000:0000:dead:beaf:dead:bead")
            ])
        );
        assert_eq!(
            windows.ipv4_addresses().collect::<Vec<_>>(),
            vec![
                "192.168.7.42",
                "192.168.8.42",
                "192.168.9.42",
                "169.254.149.176",
            ]
            .iter()
            .map(|x| x.parse::<Ipv4Addr>().unwrap())
            .collect::<Vec<_>>()
        );
        assert!(windows.is_running());
        assert_eq!(windows.power_state, PowerState::Running);
        assert_eq!(windows.name_label, "windows 10");
        assert_eq!(windows.name_description, "Here is a description");
        assert_eq!(windows.tags, [String::new(); 0]);
        assert_eq!(
            windows.os_version,
            slice_to_map(&[("spmajor", "0"), ("spminor", "0")])
        );
        assert_eq!(windows.distro().unwrap(), "windows");
        assert_eq!(windows.pool, "deadbeaf-dead-beaf-dead-beafdeadbeaf");
    }

    #[test]
    fn other_info() {
        let (debian_hash, debian_tree) = file_to_vm("test_data/vm/debian_10.json");
        let (pfsense_hash, pfsense_tree) = file_to_vm("test_data/vm/pfsense_2_5_1.json");
        let (ubuntu_hash, ubuntu_tree) = file_to_vm("test_data/vm/ubuntu_18_04.json");
        let (windows_hash, windows_tree) = file_to_vm("test_data/vm/windows_10.json");

        let debian_expected = [
            ("XenCenter.CustomFields.foo", "bar"),
            ("XenCenter.CustomFields.baz", "quix"),
            ("auto_poweron", "true"),
            ("base_template_name", "Debian Buster 10"),
            (
                "import_task",
                "OpaqueRef:deadbeaf-dead-beaf-dead-beafdeadbeaf",
            ),
            ("install-methods", "cdrom,nfs,http,ftp"),
            ("linux_template", "true"),
            ("mac_seed", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("xo:copy_of", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
        ];
        assert_eq!(debian_hash.other, slice_to_map(&debian_expected));
        assert_eq!(debian_tree.other, slice_to_map(&debian_expected));

        let pfsense_expected = [
            ("auto_poweron", "true"),
            ("base_template_name", "Other install media"),
            (
                "import_task",
                "OpaqueRef:deadbeaf-dead-beaf-dead-beafdeadbeaf",
            ),
            ("install-methods", "cdrom"),
            ("mac_seed", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("xo:copy_of", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
        ];
        assert_eq!(pfsense_hash.other, slice_to_map(&pfsense_expected));
        assert_eq!(pfsense_tree.other, slice_to_map(&pfsense_expected));

        let ubuntu_expected = [
            ("auto_poweron", "true"),
            ("base_template_name", "Other install media"),
            ("install-methods", "cdrom"),
            ("mac_seed", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("vgpu_pci", ""),
            ("xo:backup:sr", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("xo:base_delta", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("xo:copy_of", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
        ];
        assert_eq!(ubuntu_hash.other, slice_to_map(&ubuntu_expected));
        assert_eq!(ubuntu_tree.other, slice_to_map(&ubuntu_expected));

        let windows_expected = [
            ("auto_poweron", "true"),
            ("base_template_name", "Windows 10 (64-bit)"),
            (
                "import_task",
                "OpaqueRef:deadbeaf-dead-beaf-dead-beafdeadbeaf",
            ),
            ("install-methods", "cdrom"),
            ("mac_seed", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
            ("xo:copy_of", "deadbeaf-dead-beaf-dead-beafdeadbeaf"),
        ];
        assert_eq!(windows_hash.other, slice_to_map(&windows_expected));
        assert_eq!(windows_tree.other, slice_to_map(&windows_expected));
    }

    fn file_to_vm(
        path: &str,
    ) -> (
        super::Vm<HashMap<String, String>>,
        super::Vm<BTreeMap<String, String>>,
    ) {
        let s = std::fs::read_to_string(path).unwrap();
        (
            serde_json::from_str(&s).unwrap(),
            serde_json::from_str(&s).unwrap(),
        )
    }

    fn slice_to_map<T: FromIterator<(String, String)>>(slice: &[(&str, &str)]) -> T {
        slice
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }
}
