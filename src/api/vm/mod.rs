mod types;
use futures::TryStreamExt;
use std::{collections::BTreeMap, sync::Arc};

use crate::{
    procedure_object, struct_to_map, PartialVdi, PartialVif, RpcError, SrId, Template, TemplateId,
};

use futures::StreamExt;
use jsonrpsee_ws_client::WsClient;
pub use types::{OtherInfo, PowerState, Vm};

use jsonrpsee_types::{traits::Client as _, v2::params::JsonRpcParams};

use crate::{procedure_args, types::VmOrSnapshotId, SnapshotId, VmId};

use super::{Client, NewDisk, RestartError, RevertSnapshotError};

pub struct VmProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl VmProcedures {
    /// xo-cli: vm.create
    ///             [affinityHost=<string>]
    ///             [bootAfterCreate=<boolean>]
    ///             [cloudConfig=<string>]
    ///             [networkConfig=<string>]
    ///             [coreOs=<boolean>]
    ///             [clone=<boolean>]
    ///             [coresPerSocket=<string|number>]
    ///             [resourceSet=<string>]
    ///             [installation=<object>]
    ///             [vgpuType=<string>]
    ///             [gpuGroup=<string>]
    ///             name_label=<string>
    ///             [name_description=<string>]
    ///             [pv_args=<string>]
    ///             [share=<boolean>]
    ///             template=<string>
    ///             [VIFs=<array>]
    ///             [VDIs=<array>]
    ///             [existingDisks=<object>]
    ///             [hvmBootFirmware=<string>]
    ///             [copyHostBiosStrings=<boolean>] *=<any>
    /// https://github.com/vatesfr/xen-orchestra/blob/5bb2767d62432756e8d9b317c81e5f60c6c663b7/packages/xo-server/src/api/vm.mjs#L47
    pub async fn create(&self, params: NewVmArgs) -> Result<VmId, RpcError> {
        struct_to_map!(let params = params);

        self.inner
            .request("vm.create", JsonRpcParams::Map(params))
            .await
    }

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
            .request("vm.restart", JsonRpcParams::Map(params))
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
            .request("vm.snapshot", JsonRpcParams::Map(params))
            .await
            .map_err(Into::into)
    }

    /// Roll back Vm to an earlier snapshot
    ///
    /// xo-cli: vm.revert snapshot=<string>
    pub async fn revert_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<(), RevertSnapshotError> {
        #[derive(serde::Deserialize, Debug)]
        #[serde(transparent)]
        struct RevertResult(bool);

        let params = procedure_args! { "snapshot" => snapshot_id.clone() };

        let revert_result = self
            .inner
            .request::<RevertResult>("vm.revert", JsonRpcParams::Map(params))
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
            .request::<DeleteResult>("vm.delete", JsonRpcParams::Map(params))
            .await?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
pub struct NewVmArgs {
    name_label: String,
    template: TemplateId,

    //affinityHost: state.affinityHost && state.affinityHost.id,
    #[serde(skip_serializing_if = "Option::is_none")]
    clone: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "existingDisks")]
    vdis_from_template: Option<BTreeMap<usize, PartialVdi>>,
    //installation,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VDIs")]
    new_vdis: Option<Vec<NewDisk>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VIFs")]
    vifs: Option<Vec<PartialVif>>,
    //resourceSet: resourceSet && resourceSet.id,
    // vm.set parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "coresPerSocket")]
    cores_per_socket: Option<usize>,

    /// Total CPU core count across all sockets
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CPUs")]
    total_core_count: Option<usize>,

    //cpusMax: this._getCpusMax(),
    //cpuWeight: state.cpuWeight === '' ? null : state.cpuWeight,
    //cpuCap: state.cpuCap === '' ? null : state.cpuCap,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_description: Option<String>,
    //memory: memory,
    //memoryMax: memoryDynamicMax,
    //memoryMin: memoryDynamicMin,
    //memoryStaticMax,
    //pv_args: state.pv_args,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "autoPoweron")]
    auto_poweron: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bootAfterCreate")]
    boot_after_create: Option<bool>,
    //copyHostBiosStrings:
    //    state.hvmBootFirmware !== 'uefi' && !this._templateHasBiosStrings() && state.copyHostBiosStrings,
    //secureBoot: state.secureBoot,
    //share: state.share,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudConfig")]
    cloud_config: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "networkConfig")]
    network_config: Option<String>,
    //coreOs: this._isCoreOs(),
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    //vgpuType: get(() => state.vgpuType.id),
    //gpuGroup: get(() => state.vgpuType.gpuGroup),
    //hvmBootFirmware: state.hvmBootFirmware === '' ? undefined : state.hvmBootFirmware,
}

impl NewVmArgs {
    pub fn new_raw(name_label: String, template: TemplateId) -> Self {
        NewVmArgs {
            name_label,
            template,

            clone: None,

            vdis_from_template: None,

            new_vdis: None,

            vifs: None,
            cores_per_socket: None,
            total_core_count: None,

            name_description: None,

            auto_poweron: None,
            boot_after_create: None,
            cloud_config: None,
            network_config: None,
            tags: None,
        }
    }

    pub async fn new(
        name_label: String,
        template: &Template,
        con: &Client,
    ) -> Result<Self, RpcError> {
        let mut this = Self::new_raw(name_label, template.id.clone());

        let vdis_from_template: BTreeMap<usize, PartialVdi> = futures::stream::iter(&template.vbds)
            .filter_map(|vbd_id| async move {
                let vbd = match con
                    .get_vbds(procedure_object! { "id" => vbd_id.0.clone() }, None)
                    .await
                {
                    Ok(mut r) => r.remove(vbd_id)?,
                    Err(e) => return Some(Err(e)),
                };

                if vbd.is_cd_drive {
                    return None;
                };

                let vdi = match con
                    .get_vdis(procedure_object! { "id" => vbd.vdi.0.clone() }, None)
                    .await
                {
                    Ok(mut r) => r.remove(&vbd.vdi)?,
                    Err(e) => return Some(Err(e)),
                };

                Some(Ok((
                    vbd.position,
                    PartialVdi {
                        name_label: vdi.name_label,
                        name_description: vdi.name_description,
                        size: vdi.size,
                        sr: vdi.sr,
                    },
                )))
            })
            .try_collect()
            .await?;

        let vifs: Vec<PartialVif> = futures::stream::iter(&template.vifs)
            .filter_map(|vif_id| async move {
                let response = con
                    .get_vifs(procedure_object! { "id" => vif_id.clone() }, None)
                    .await;

                let vif = match response {
                    Ok(mut response) => response.remove(vif_id)?,
                    Err(e) => return Some(Err(e)),
                };

                // TODO: Is it possible to avoid filter_map here
                Some(Ok(PartialVif {
                    network: vif.network,
                }))
            })
            .try_collect()
            .await?;

        this.vdis_from_template = Some(vdis_from_template);
        this.vifs = Some(vifs);

        Ok(this)
    }

    pub fn grow_vdi_from_template(
        &mut self,
        vdi_index: usize,
        new_size: usize,
    ) -> Result<(), GrowVdiFromTemplateError> {
        let vdis = match &mut self.vdis_from_template {
            Some(vdis) => vdis,
            None => return Err(GrowVdiFromTemplateError::NoSuchDisk { vdi_index }),
        };

        let vdi = match vdis.get_mut(&vdi_index) {
            Some(vdi) => vdi,
            None => return Err(GrowVdiFromTemplateError::NoSuchDisk { vdi_index }),
        };

        if new_size < vdi.size {
            return Err(GrowVdiFromTemplateError::NewSizeIsSmaller {
                current_size: vdi.size,
                new_size,
            });
        }

        vdi.size = new_size;
        Ok(())
    }

    pub fn set_sr_for_vdi_from_template(
        &mut self,
        vdi_index: usize,
        sr: SrId,
    ) -> Result<(), NoSuchDiskError> {
        let vdis = match &mut self.vdis_from_template {
            Some(vdis) => vdis,
            None => return Err(NoSuchDiskError { vdi_index }),
        };

        let vdi = match vdis.get_mut(&vdi_index) {
            Some(vdi) => vdi,
            None => return Err(NoSuchDiskError { vdi_index }),
        };

        vdi.sr = sr;
        Ok(())
    }

    //TODO: add methods for adding vifs and vdis
}

#[derive(Debug)]
pub enum GrowVdiFromTemplateError {
    NoSuchDisk {
        vdi_index: usize,
    },
    NewSizeIsSmaller {
        new_size: usize,
        current_size: usize,
    },
}

#[derive(Debug)]
pub struct NoSuchDiskError {
    #[allow(dead_code)]
    vdi_index: usize,
}
