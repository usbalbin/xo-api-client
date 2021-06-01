use std::time::{Duration, SystemTime};

use jsonrpsee_ws_client::JsonValue;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(serde::Deserialize, Debug)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub name: String,
    pub vm_name: String,

    #[serde(deserialize_with = "duration_from_seconds")]
    pub snapshot_age: Duration,
}

fn duration_from_seconds<'de, D>(des: D) -> Result<Duration, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = serde::de::Deserialize::deserialize(des)?;
    let snapshot_time = serde_json::from_str(s).map_err(serde::de::Error::custom)?;

    let now_since_unix_epoch = SystemTime::UNIX_EPOCH.elapsed().unwrap();
    let snapshot_since_epoch = Duration::from_secs(snapshot_time);
    let age = now_since_unix_epoch
        .checked_sub(snapshot_since_epoch)
        .unwrap_or(Duration::ZERO);

    Ok(age)
}
