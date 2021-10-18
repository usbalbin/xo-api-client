use std::collections::BTreeMap;

use jsonrpsee_types::{
    traits::{Client as RpcCient, SubscriptionClient},
    v2::params::ParamsSer,
    DeserializeOwned, JsonValue, Subscription,
};
use jsonrpsee_ws_client::{WsClient, WsClientBuilder};

use crate::{
    credentials::{Credentials, Token},
    procedure_args, procedure_object,
    types::VmOrSnapshotId,
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
                Some(ParamsSer::Map(credentials.into().into())),
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
            .request("token.create", Some(ParamsSer::Map(BTreeMap::new())))
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
            .request("xo.getAllObjects", Some(ParamsSer::Map(args)))
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
        let filter = match filter.into() {
            Some(mut filter) => {
                filter.insert("type".to_string(), object_type.into());
                filter
            }
            None => procedure_object! { "type" => object_type },
        };

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

    declare_object_getter!(ObjectType::VmSnapshot, fn get_snapshots : SnapshotId => Snapshot);

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
            .request::<DeleteResult>("vm.delete", Some(ParamsSer::Map(params)))
            .await?;

        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct SigninResponse {}
