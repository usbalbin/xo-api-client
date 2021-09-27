mod types;
pub use types::{OtherInfo, PowerState, Vm};

use jsonrpsee_types::{traits::Client as _, v2::params::ParamsSer};
use jsonrpsee_ws_client::WsClient;
use std::sync::Arc;

use crate::{procedure_args, types::VmOrSnapshotId, RpcError, SnapshotId, VmId};

use super::{RestartError, RevertSnapshotError};

pub struct VmProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl VmProcedures {
    /// This function will try to initiate a soft restart of the VM
    /// The there is no guarantee that the VM has started once the returned
    /// future resolves
    ///
    /// xo-cli: vm.restart id=<string> [force=<boolean>]
    pub async fn restart_nonblocking(&self, vm_id: VmId) -> Result<(), RestartError> {
        #[derive(serde::Deserialize, Debug)]
        #[serde(transparent)]
        struct RestartResult(bool);

        let params = procedure_args! { "id" => vm_id };

        let restart_suceeded: RestartResult = self
            .inner
            .request("vm.restart", Some(ParamsSer::Map(params)))
            .await
            .map_err(RestartError::Rpc)?;

        if let RestartResult(false) = restart_suceeded {
            return Err(RestartError::ReportedFail);
        }

        Ok(())
    }

    /// Create snapshot of the specified VM
    ///
    /// `save_memory`: Should the RAM memory of the VM be saved? Setting this to true does make the
    /// snapshot take a lot more time, may even freeze the VM for some time. If you are unsure,
    /// set this to `false`
    ///
    /// xo-cli: vm.snapshot [description=<string>] id=<string> [name=<string>] [saveMemory=<boolean>]
    pub async fn snapshot(
        &self,
        vm_id: VmId,
        name: String,
        description: String,
        save_memory: bool,
    ) -> Result<SnapshotId, RpcError> {
        let params = procedure_args! {
            "id" => vm_id,
            "name" => name,
            "description" => description,
            "saveMemory"=> save_memory,
        };

        self.inner
            .request("vm.snapshot", Some(ParamsSer::Map(params)))
            .await
            .map_err(Into::into)
    }

    /// Roll back Vm to an earlier snapshot
    ///
    /// xo-cli: vm.revert snapshot=<string>
    pub async fn revert(&self, snapshot_id: SnapshotId) -> Result<(), RevertSnapshotError> {
        #[derive(serde::Deserialize, Debug)]
        #[serde(transparent)]
        struct RevertResult(bool);

        let params = procedure_args! { "snapshot" => snapshot_id.clone() };

        let revert_result = self
            .inner
            .request::<RevertResult>("vm.revert", Some(ParamsSer::Map(params)))
            .await
            .map_err(RevertSnapshotError::Rpc)?;

        if let RevertResult(false) = revert_result {
            log::warn!("revert_snapshot: {:?} false", snapshot_id);
            return Err(RevertSnapshotError::ReportedFail);
        }

        Ok(())
    }

    /// This may be used for deleting snapshots as well as entire VMs, so be careful!
    ///
    /// xo-cli: vm.delete id=<string>
    pub async fn delete(
        &self,
        vm_or_snapshot_id: impl Into<VmOrSnapshotId>,
    ) -> Result<(), RpcError> {
        #[derive(serde::Deserialize)]
        #[serde(transparent)]
        struct DeleteResult(([(); 0], [(); 1]));

        let vm_or_snapshot_id = vm_or_snapshot_id.into();

        let params = procedure_args! { "id" => vm_or_snapshot_id };

        self.inner
            .request::<DeleteResult>("vm.delete", Some(ParamsSer::Map(params)))
            .await?;

        Ok(())
    }
}
