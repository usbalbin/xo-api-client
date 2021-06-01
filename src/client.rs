use std::{array, collections::BTreeMap};

use futures::StreamExt;
use jsonrpsee_ws_client::{
    traits::{Client as RpcCient, SubscriptionClient},
    v2::params::JsonRpcParams,
    JsonValue, WsClient, WsClientBuilder,
};
use tokio::task::JoinHandle;

use crate::{
    token_handler::TokenHandler, types::VmOrSnapshotId, vm::OtherInfo, ObjectType, RpcError,
    Snapshot, SnapshotId, Vm, VmId,
};

#[derive(serde::Deserialize)]
struct SigninResponse {}

pub struct Client {
    inner: WsClient,
    _bg: JoinHandle<()>,
}

#[derive(Debug)]
pub enum ConnectError<TH: TokenHandler> {
    TokenHandlerSave(TH::SaveErr),
    TokenHandlerLoad(TH::LoadErr),
    Rpc(RpcError),
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
    pub async fn connect<TH: TokenHandler>(
        url: &str,
        token_handler: &mut TH,
    ) -> Result<Self, ConnectError<TH>> {
        log::debug!("Connecting...");
        let inner = Client::connect_inner(url).await;

        let mut subscription = inner
            .register_notification::<BTreeMap<String, JsonValue>>("all")
            .await
            .unwrap();

        // TODO: What to do with this?
        let bg = tokio::spawn(async move {
            loop {
                if let Some(data) = subscription.notifs_rx.next().await {
                    log::trace!("Received: {:?}", data);
                }
            }
        });

        let token = token_handler
            .load()
            .await
            .map_err(ConnectError::TokenHandlerLoad)?;

        let _: SigninResponse = inner
            .request(
                "session.signIn",
                JsonRpcParams::Map(array::IntoIter::new([("token", token.into())]).collect()),
            )
            .await
            .expect("Sign in failed");

        Client::update_token(&inner, token_handler).await?;

        Ok(Client { inner, _bg: bg })
    }

    pub async fn sign_in<TH: TokenHandler>(
        url: &str,
        username: String,
        password: String,
        token_handler: &mut TH,
    ) -> Result<(), ConnectError<TH>> {
        let inner = Client::connect_inner(url).await;
        log::info!("Signing in...");

        #[derive(serde::Serialize)]
        pub struct Credentials {
            email: String,
            password: String,
        }

        let _: SigninResponse = inner
            .request(
                "session.signIn",
                JsonRpcParams::Map(
                    array::IntoIter::new([
                        ("email", username.into()),
                        ("password", password.into()),
                    ])
                    .collect(),
                ),
            )
            .await
            .map_err(ConnectError::Rpc)?;

        Client::update_token(&inner, token_handler).await?;

        Ok(())
    }

    async fn connect_inner(url: &str) -> WsClient {
        log::info!("Connecting to: {}", url);

        let con = {
            let builder = WsClientBuilder::default()
                .certificate_store(jsonrpsee_ws_client::transport::CertificateStore::Native)
                .connection_timeout(std::time::Duration::from_secs(10));

            builder.build(&url).await.unwrap()
        };
        log::info!("Connected");

        con
    }

    async fn update_token<TH: TokenHandler>(
        inner: &WsClient,
        token_handler: &mut TH,
    ) -> Result<(), ConnectError<TH>> {
        // TODO: consider specifying the `expiresIn` parameter
        let token: String = inner
            .request("token.create", JsonRpcParams::Map(BTreeMap::new()))
            .await
            .map_err(ConnectError::Rpc)?;

        token_handler
            .save(&token)
            .await
            .map_err(ConnectError::TokenHandlerSave)?;

        Ok(())
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
