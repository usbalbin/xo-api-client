use std::time::{Duration, SystemTime};

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
    pub name: String,
    pub vm_name: String,

    /// Approximation of how much time has passed from the snapshot was created
    /// to when this Snapshot object was queried from the server
    /// Note that his is only aproximation
    #[serde(deserialize_with = "duration_from_seconds")]
    pub snapshot_age: Duration,
}

// TODO: how accurate is this aproximation?
fn duration_from_seconds<'de, D>(des: D) -> Result<Duration, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let snapshot_time = serde::de::Deserialize::deserialize(des)?;

    // Duration from unix epoch to now
    let now_since_unix_epoch = SystemTime::UNIX_EPOCH.elapsed().unwrap();

    // Duration from unix epoch to snapshot creation
    let snapshot_since_epoch = Duration::from_secs(snapshot_time);

    // Age is the difference
    let age = now_since_unix_epoch
        .checked_sub(snapshot_since_epoch)
        .unwrap_or_else(|| Duration::from_secs(0));

    Ok(age)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Impossible {}
