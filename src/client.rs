use std::{array, collections::BTreeMap};

use jsonrpsee_ws_client::{
    traits::{Client as RpcCient, SubscriptionClient},
    v2::params::JsonRpcParams,
    JsonValue, WsClient, WsClientBuilder,
};
use tokio::task::JoinHandle;

use crate::{
    credentials::{Credentials, Token},
    types::VmOrSnapshotId,
    vm::OtherInfo,
    ObjectType, RpcError, Snapshot, SnapshotId, Vm, VmId,
};

#[derive(serde::Deserialize)]
struct SigninResponse {}

pub struct Client {
    inner: WsClient,
    _bg: JoinHandle<()>,
}

#[derive(Debug)]
pub enum RestartError {
    ReportedFail,
    Rpc(RpcError),
}

#[derive(Debug)]
pub enum RevertSnapshotError {
    ReportedFail,
    Rpc(RpcError),
}

impl Client {
    pub async fn connect(url: &str) -> Result<Self, RpcError> {
        log::debug!("Connecting to: {}", url);

        let inner = WsClientBuilder::default()
            .connection_timeout(std::time::Duration::from_secs(10))
            .build(&url)
            .await?;

        log::debug!("Connected");

        // xo-server tends to send notifications to the clients procedure "all", make sure to
        // listen to this or jsonrpsee_ws_client will report errors
        let mut subscription = inner
            .subscribe_to_method::<BTreeMap<String, JsonValue>>("all")
            .await?;

        // TODO: What to do with this?
        // This spawns a background task handling the "all" notification mentioned above
        let bg = tokio::spawn(async move {
            loop {
                if let Some(data) = subscription.next().await.transpose() {
                    log::trace!("Received: {:?}", data);
                }
            }
        });

        Ok(Client { inner, _bg: bg })
    }

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

    pub async fn create_token(&self) -> Result<Token, RpcError> {
        // TODO: consider specifying the `expiresIn` parameter
        let token: Token = self
            .inner
            .request("token.create", JsonRpcParams::Map(BTreeMap::new()))
            .await?;

        Ok(token)
    }

    /// xo-cli: xo.getAllObjects [filter=<object>] [limit=<number>] [ndjson=<boolean>]
    pub async fn get_all_objects<R: serde::de::DeserializeOwned>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<R, RpcError> {
        let args = match (filter.into(), limit.into()) {
            (Some(filter), Some(limit)) => {
                array::IntoIter::new([("filter", filter.into()), ("limit", limit.into())]).collect()
            }
            (Some(filter), None) => array::IntoIter::new([("filter", filter.into())]).collect(),
            (None, Some(limit)) => array::IntoIter::new([("limit", limit.into())]).collect(),
            (None, None) => array::IntoIter::new([]).collect(),
        };

        self.inner
            .request("xo.getAllObjects", JsonRpcParams::Map(args))
            .await
    }

    pub async fn get_objects_of_type<R: serde::de::DeserializeOwned>(
        &self,
        object_type: ObjectType,
        filter: Option<serde_json::Map<String, JsonValue>>,
        limit: Option<usize>,
    ) -> Result<R, RpcError> {
        let filter = match filter {
            Some(mut filter) => {
                filter.insert("type".to_string(), object_type.into());
                filter
            }
            None => array::IntoIter::new([("type".to_owned(), object_type.into())]).collect(),
        };

        let objects = self.get_all_objects(filter, limit).await?;

        Ok(objects)
    }

    pub async fn get_vms<O: OtherInfo>(
        &self,
        filter: Option<serde_json::Map<String, JsonValue>>,
        limit: Option<usize>,
    ) -> Result<BTreeMap<VmId, Vm<O>>, RpcError> {
        self.get_objects_of_type::<BTreeMap<VmId, Vm<O>>>(ObjectType::Vm, filter, limit)
            .await
    }

    pub async fn get_snapshots(
        &self,
        filter: Option<serde_json::Map<String, JsonValue>>,
        limit: Option<usize>,
    ) -> Result<BTreeMap<SnapshotId, Snapshot>, RpcError> {
        self.get_objects_of_type::<BTreeMap<SnapshotId, Snapshot>>(
            ObjectType::VmSnapshot,
            filter,
            limit,
        )
        .await
    }

    /// This function will try to initiate a soft restart of the server
    /// The there is not guarantee that the VM has started once the returned
    /// future resolves
    ///
    /// xo-cli: vm.restart id=<string> [force=<boolean>]
    pub async fn restart_nonblocking(&self, vm_id: VmId) -> Result<(), RestartError> {
        #[derive(serde::Deserialize, Debug)]
        #[serde(transparent)]
        struct RestartResult(bool);

        let params = array::IntoIter::new([("id", vm_id.into())]).collect();

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

    /// save_memory: Should the RAM memory of the VM be saved? Setting this to true does make the
    /// snapshot take a lot more time, may even freeze the VM for some time
    ///
    /// xo-cli: vm.snapshot [description=<string>] id=<string> [name=<string>] [saveMemory=<boolean>]
    pub async fn snapshot(
        &self,
        vm_id: VmId,
        name: String,
        description: String,
        save_memory: bool,
    ) -> Result<SnapshotId, RpcError> {
        let params = array::IntoIter::new([
            ("id", vm_id.into()),
            ("name", name.into()),
            ("description", description.into()),
            ("saveMemory", save_memory.into()),
        ])
        .collect();

        self.inner
            .request("vm.snapshot", JsonRpcParams::Map(params))
            .await
            .map_err(Into::into)
    }

    /// xo-cli: vm.revert snapshot=<string>
    pub async fn revert_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<(), RevertSnapshotError> {
        #[derive(serde::Deserialize, Debug)]
        #[serde(transparent)]
        struct RevertResult(bool);

        let params = array::IntoIter::new([("snapshot", snapshot_id.clone().into())]).collect();

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

        let params = array::IntoIter::new([("id", vm_or_snapshot_id.into())]).collect();

        self.inner
            .request::<DeleteResult>("vm.delete", JsonRpcParams::Map(params))
            .await?;

        Ok(())
    }
}
