use futures::{StreamExt, TryStreamExt};

use std::collections::{BTreeMap, HashMap};

use jsonrpsee_types::{
    traits::{Client as RpcCient, SubscriptionClient},
    v2::params::JsonRpcParams,
    DeserializeOwned, JsonValue, Subscription,
};
use jsonrpsee_ws_client::{WsClient, WsClientBuilder};

use crate::{
    credentials::{Credentials, Token},
    procedure_args, procedure_object,
    types::{
        PartialVdi, PartialVif, Template, TemplateId, Vbd, VbdId, Vdi, VdiId, Vif, VifId,
        VmOrSnapshotId,
    },
    vm::OtherInfo,
    ObjectType, RpcError, Snapshot, SnapshotId, Vm, VmId,
};

macro_rules! declare_object_getter {
    ($item_type_enum:expr, fn $fn_name:ident : $index_type:ident => $item_type:ty) => {
        /// Get all $item_type s from server
        /// * `filter` is an optional filter
        /// * `limit` is an optional max limit on number of results
        pub async fn $fn_name(
            &self,
            filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
            limit: impl Into<Option<usize>>,
        ) -> Result<BTreeMap<$index_type, $item_type>, RpcError> {
            // ::<BTreeMap<$index_type, $item_type>>
            self.get_objects_of_type/*::<BTreeMap<$index_type, $item_type>>*/(
                $item_type_enum,
                filter,
                limit,
            )
            .await
        }
    };
}

/// Error during restart of VM
#[derive(Debug)]
pub enum RestartError {
    ReportedFail,
    Rpc(RpcError),
}

/// Error during revert of VM snapshot
#[derive(Debug)]
pub enum RevertSnapshotError {
    ReportedFail,
    Rpc(RpcError),
}

/// Client used to communicate with Xen Orchestra's API
///
/// Example of listing all VMs with the tag `Test`
/// ```no_run
/// use std::collections::BTreeMap;
/// use xo_api_client::{credentials::EmailAndPassword, Client, Vm, VmId};
///
/// // We dont care about any of the data under the "other" attribute
/// // in this example
/// #[derive(serde::Deserialize)]
/// struct OtherInfo {}
///
/// impl xo_api_client::vm::OtherInfo for OtherInfo {}
///
/// #[tokio::main]
/// async fn main() {
///     let url = "ws://localhost:8080/api/";
///     let email = String::from("admin@admin.net");
///     let password = String::from("admin");
///
///     let con = Client::connect(url)
///         .await
///         .expect("Failed to connect to server");
///
///     con.sign_in(EmailAndPassword { email, password })
///         .await
///         .expect("Failed to sign in");
///
///     let all_vms: BTreeMap<VmId, Vm<OtherInfo>> =
///         con.get_vms(None, None).await.expect("Failed to list VMs");
///
///     let test_vms = all_vms
///         .iter()
///         .filter(|(_id, vm)| vm.tags.iter().any(|tag| tag == "Test"));
///
///     println!("All VMs with the tag 'Test':");
///     for (id, vm) in test_vms {
///         println!("ID: {:?}, Name: {}", id, vm.name_label);
///     }
/// }
/// ```
pub struct Client {
    inner: WsClient,
}

impl Client {
    /// Connect to xo-server
    ///
    /// Note that `url` is the websocket URL to the API endpoint, usually something like
    /// wss://example.com/api/ or ws://example.com/api/ for unencrypted
    pub async fn connect(url: &str) -> Result<Self, RpcError> {
        log::debug!("Connecting to: {}", url);

        let inner = WsClientBuilder::default()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(url)
            .await?;

        log::debug!("Connected");

        Ok(Client { inner })
    }

    /// Sign in to xo-server, this is required for access to most of the other methods
    ///
    /// xo-cli: session.signIn
    pub async fn sign_in(&self, credentials: impl Into<Credentials>) -> Result<(), RpcError> {
        log::debug!("Signing in...");

        #[derive(serde::Serialize)]
        pub struct Credentials {
            email: String,
            password: String,
        }

        let _: SigninResponse = self
            .inner
            .request(
                "session.signIn",
                JsonRpcParams::Map(credentials.into().into()),
            )
            .await?;

        log::debug!("Signed in");

        Ok(())
    }

    /// Subscribe to method "all"
    ///
    /// xo-server tends to send notifications to the client's JSON RPC procedure "all"
    /// subscribe_to_notification_all returns a value that can be used to read those
    /// notifications
    pub async fn subscribe_to_notification_all<T>(&self) -> Result<Subscription<T>, RpcError>
    where
        T: DeserializeOwned,
    {
        self.inner.subscribe_to_method::<T>("all").await
    }

    /// Create authentication token
    ///
    /// xo-cli: token.create [expiresIn=<number|string>]
    pub async fn create_token(&self) -> Result<Token, RpcError> {
        // TODO: consider specifying the `expiresIn` parameter
        let token: Token = self
            .inner
            .request("token.create", JsonRpcParams::Map(BTreeMap::new()))
            .await?;

        Ok(token)
    }

    /// Get all objects from server
    /// * `R` is a type that can hold that entire result set with all different types
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    /// xo-cli: xo.getAllObjects [filter=<object>] [limit=<number>] [ndjson=<boolean>]
    pub async fn get_all_objects<R: serde::de::DeserializeOwned>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<R, RpcError> {
        let args = match (filter.into(), limit.into()) {
            (Some(filter), Some(limit)) => {
                procedure_args! { "filter" => filter, "limit" => limit }
            }
            (Some(filter), None) => procedure_args! { "filter" => filter },
            (None, Some(limit)) => procedure_args! { "limit" => limit },
            (None, None) => procedure_args! {},
        };

        self.inner
            .request("xo.getAllObjects", JsonRpcParams::Map(args))
            .await
    }

    /// Get all object of specified type from server
    /// * `R` is a type that can represent that collection of objects
    /// * `object_type` is the type of object to fetch
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    pub async fn get_objects_of_type<R: serde::de::DeserializeOwned>(
        &self,
        object_type: ObjectType,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<R, RpcError> {
        let mut filter = filter.into().unwrap_or_default();
        filter.insert("type".to_string(), object_type.into());

        let objects = self.get_all_objects(filter, limit).await?;

        Ok(objects)
    }

    /// Get all VMs from server
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    pub async fn get_vms<O: OtherInfo>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<BTreeMap<VmId, Vm<O>>, RpcError> {
        self.get_objects_of_type/*::<BTreeMap<VmId, Vm<O>>>*/(ObjectType::Vm, filter, limit)
            .await
    }
    declare_object_getter!(ObjectType::VmTemplate, fn get_template : TemplateId => Template);
    declare_object_getter!(ObjectType::VmSnapshot, fn get_snapshots : SnapshotId => Snapshot);
    declare_object_getter!(ObjectType::Vbd, fn get_vbds : VbdId => Vbd);
    declare_object_getter!(ObjectType::Vdi, fn get_vdis : VdiId => Vdi);
    declare_object_getter!(ObjectType::Vif, fn get_vifs : VifId => Vif);

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
    pub async fn delete_snapshot(
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
    pub async fn create_vm(&self, params: NewVmArgs) -> Result<VmId, RpcError> {
        // TODO: Find a better solution to this. Currently we go from
        // NewVmArgs -> JsonValue ->
        // serde::Map -> BTreeMap<&str, JsonValue>
        // which is less than optimal...
        let params = serde_json::to_value(params).unwrap();
        let params = match &params {
            JsonValue::Object(params) => params
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect(),
            _ => unreachable!(),
        };

        let response = self.inner
            .request::</*CreateResult*/JsonValue>("vm.create", JsonRpcParams::Map(params))
            .await?;
        println!("response: {:?}", response);
        todo!()
    }
}
/*
{
  affinityHost: { type: 'string', optional: true },

  bootAfterCreate: {
    type: 'boolean',
    optional: true,
  },

  cloudConfig: {
    type: 'string',
    optional: true,
  },

  networkConfig: {
    type: 'string',
    optional: true,
  },

  coreOs: {
    type: 'boolean',
    optional: true,
  },

  clone: {
    type: 'boolean',
    optional: true,
  },

  coresPerSocket: {
    type: ['string', 'number'],
    optional: true,
  },

  resourceSet: {
    type: 'string',
    optional: true,
  },

  installation: {
    type: 'object',
    optional: true,
    properties: {
      method: { type: 'string' },
      repository: { type: 'string' },
    },
  },

  vgpuType: {
    type: 'string',
    optional: true,
  },

  gpuGroup: {
    type: 'string',
    optional: true,
  },

  // Name/description of the new VM.
  name_label: { type: 'string' },
  name_description: { type: 'string', optional: true },

  // PV Args
  pv_args: { type: 'string', optional: true },

  share: {
    type: 'boolean',
    optional: true,
  },

  // TODO: add the install repository!
  // VBD.insert/eject
  // Also for the console!

  // UUID of the template the VM will be created from.
  template: { type: 'string' },

  // Virtual interfaces to create for the new VM.
  VIFs: {
    optional: true,
    type: 'array',
    items: {
      type: 'object',
      properties: {
        // UUID of the network to create the interface in.
        network: { type: 'string' },

        mac: {
          optional: true, // Auto-generated per default.
          type: 'string',
        },

        allowedIpv4Addresses: {
          optional: true,
          type: 'array',
          items: { type: 'string' },
        },

        allowedIpv6Addresses: {
          optional: true,
          type: 'array',
          items: { type: 'string' },
        },
      },
    },
  },

  // Virtual disks to create for the new VM.
  VDIs: {
    optional: true, // If not defined, use the template parameters.
    type: 'array',
    items: {
      type: 'object',
      properties: {
        size: { type: ['integer', 'string'] },
        SR: { type: 'string' },
        type: { type: 'string' },
      },
    },
  },

  // TODO: rename to *existingVdis* or rename *VDIs* to *disks*.
  existingDisks: {
    optional: true,
    type: 'object',

    // Do not for a type object.
    items: {
      type: 'object',
      properties: {
        size: {
          type: ['integer', 'string'],
          optional: true,
        },
        $SR: {
          type: 'string',
          optional: true,
        },
      },
    },
  },

  hvmBootFirmware: { type: 'string', optional: true },

  copyHostBiosStrings: { type: 'boolean', optional: true },

  // other params are passed to `editVm`
  '*': { type: 'any' },
}
*/

#[derive(serde::Serialize)]
pub struct NewVmArgs {
    name_label: String,
    template: TemplateId,

    //affinityHost: state.affinityHost && state.affinityHost.id,
    #[serde(skip_serializing_if = "Option::is_none")]
    clone: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "existingDisks")]
    existing_disks: Option<HashMap<usize, PartialVdi>>,
    //installation,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VDIs")]
    vdis: Option<Vec<PartialVdi>>,

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

            existing_disks: None,

            vdis: None,

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

        let existing_disks: HashMap<usize, PartialVdi> = futures::stream::iter(&template.vbds)
            .filter_map(|vbd_id| async move {
                println!("vbd");
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

                println!("vdi");
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
        /*
        forEach(template.vbds, |vbdId| {
            // VbdId -> Vbd, Vbd.VdiId -> Vdi

            const vbd = getObject(storeState, vbdId, resourceSet)
            if (!vbd || vbd.is_cd_drive) {
                return
            }
            const vdi = getObject(storeState, vbd.VDI, resourceSet)
            if (vdi) {
                existingDisks.insert(vbd.position, PartialVdi {
                    name_label: vdi.name_label,
                    name_description: vdi.name_description,
                    size: vdi.size,
                    sr: vdi.sr,
                });
            }
        });*/

        let vifs: Vec<PartialVif> = futures::stream::iter(&template.vifs)
            .filter_map(|vif_id| async move {
                println!("vifs");
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

        /*
        let VIFs = []
        const defaultNetworkIds = this._getDefaultNetworkIds(template)
        forEach(template.VIFs, vifId => {
            const vif = getObject(storeState, vifId, resourceSet)
            VIFs.push({
                network: pool || vif.$network
            })
        })
        if (VIFs.length === 0) {
            VIFs = defaultNetworkIds.map(id => ({ network: id }))
        }
        */

        this.existing_disks = Some(existing_disks);
        this.vifs = Some(vifs);

        Ok(this)
    }
}
#[derive(serde::Deserialize)]
struct SigninResponse {}
