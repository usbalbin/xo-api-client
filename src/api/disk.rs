use std::sync::Arc;

use crate::{struct_to_map, RpcError, SrId, VmId};

use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams, Serialize};
use jsonrpsee_ws_client::WsClient;
use serde::Serializer;

use crate::types::VdiId;

pub struct DiskProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl DiskProcedures {
    pub async fn create(&self, args: NewDisk) -> Result<VdiId, RpcError> {
        struct_to_map!(let args = args);
        self.inner
            .request("disk.create", JsonRpcParams::Map(args))
            .await
    }
}

/*{
    name: { type: 'string' },
    size: { type: ['integer', 'string'] },
    sr: { type: 'string' },
    vm: { type: 'string', optional: true },
    bootable: { type: 'boolean', optional: true },
    mode: { type: 'string', optional: true },
    position: { type: 'string', optional: true },
}*/

#[derive(serde::Serialize)]
pub enum NewDisk {
    Attached {
        name: String,
        size: usize,
        sr: SrId,
        
        vm: VmId,

        // TODO: is this really optional when vm is set?
        bootable: Option<bool>,

        // TODO: is this really optional when vm is set?
        mode: Option<DiskMode>,

        // TODO: is this really optional when vm is set?
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(serialize_with = "option_usize_to_string")]
        position: Option<usize>,
    }, NotAttached {
        name: String,
        size: usize,
        sr: SrId,
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiskMode {
    System,
}

fn option_usize_to_string<S>(v: &Option<usize>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(v) => serde_with::rust::display_fromstr::serialize(v, serializer),
        None => ().serialize(serializer),
    }
}
