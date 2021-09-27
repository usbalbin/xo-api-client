pub mod api;
mod client;
pub mod credentials;
mod object_type;
mod types;

#[macro_use]
mod macros;

pub use api::{vm::Vm, Client, RestartError, RevertSnapshotError};
pub use jsonrpsee_types::{Error as RpcError, JsonValue};
pub use object_type::ObjectType;
pub use types::{
    NetworkId, PartialVdi, PartialVif, Snapshot, SnapshotId, SrId, Subscription, Template,
    TemplateId, VmId,
};
