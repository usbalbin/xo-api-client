mod client;
pub mod credentials;
mod object_type;
mod types;

#[macro_use]
mod macros;
pub mod vm;

pub use client::{Client, NewVmArgs, RestartError, RevertSnapshotError};
pub use jsonrpsee_types::{Error as RpcError, JsonValue};
pub use object_type::ObjectType;
pub use types::{
    NetworkId, PartialVdi, PartialVif, Snapshot, SnapshotId, SrId, Subscription, Template,
    TemplateId, VmId,
};
pub use vm::Vm;
