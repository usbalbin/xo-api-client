use jsonrpsee_ws_client::JsonValue;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impossible {}

#[cfg(test)]
mod tests {
    #[test]
    fn debian() {
        let s = std::fs::read_to_string("test_data/snapshot/debian_10.json").unwrap();
        let debian_snapshot: super::Snapshot = serde_json::from_str(&s).unwrap();

        assert_eq!(debian_snapshot.id.0, "deadbeaf-dead-beaf-dead-beafdeadbea0");
        assert_eq!(debian_snapshot.name_label, "[XO My Backup Job] debian 10");
        assert_eq!(debian_snapshot.name_description, "");

        let s = std::fs::read_to_string("test_data/snapshot/pfsense_2_5_1.json").unwrap();
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
