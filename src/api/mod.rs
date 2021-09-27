pub mod session;
pub mod token;
pub mod vm;
pub mod xo;

use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee_types::{traits::SubscriptionClient, DeserializeOwned, JsonValue, Subscription};
use jsonrpsee_ws_client::{WsClient, WsClientBuilder};

use crate::{
    procedure_object,
    types::{XoObject, XoObjectMap},
    RpcError,
};

use self::{
    session::SessionProcedures,
    token::TokenProcedures,
    vm::{OtherInfo, Snapshot, Vm, VmId, VmProcedures},
    xo::XoProcedures,
};

macro_rules! declare_object_getter {
    ($item_type:ty : single: fn $fn_name_single:ident, multi: fn $fn_name_multi:ident) => {
        /// Get all $item_type s from server
        /// * `filter` is an optional filter
        /// * `limit` is an optional max limit on number of results
        pub async fn $fn_name_multi(
            &self,
            filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
            limit: impl Into<Option<usize>>,
        ) -> Result<BTreeMap<<$item_type as XoObject>::IdType, $item_type>, RpcError> {
            self.get_objects_of_type(filter, limit).await
        }

        /// Get one $item_type from server
        pub async fn $fn_name_single(
            &self,
            id: <$item_type as XoObject>::IdType,
        ) -> Result<Option<$item_type>, GetSingleObjectError> {
            self.get_object_of_type(id).await
        }
    };
}

#[derive(Debug)]
pub enum GetSingleObjectError {
    MultipleMatches,
    Rpc(RpcError),
}

/// Client used to communicate with Xen Orchestra's API
///
/// Example of listing all VMs with the tag `Test`
/// ```no_run
/// use std::collections::BTreeMap;
/// use xo_api_client::{credentials::EmailAndPassword, Client, api::vm::{Vm, VmId}};
///
/// // We dont care about any of the data under the "other" attribute
/// // in this example
/// #[derive(serde::Deserialize)]
/// struct OtherInfo {}
///
/// impl xo_api_client::api::vm::OtherInfo for OtherInfo {}
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
///     con
///         .session
///         .sign_in(EmailAndPassword { email, password })
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
    inner: Arc<WsClient>,

    pub vm: VmProcedures,
    pub xo: XoProcedures,
    pub token: TokenProcedures,
    pub session: SessionProcedures,
}

impl Client {
    /// Connect to xo-server
    ///
    /// Note that `url` is the websocket URL to the API endpoint, usually something like
    /// wss://example.com/api/ or ws://example.com/api/ for unencrypted
    pub async fn connect(url: &str) -> Result<Self, RpcError> {
        log::debug!("Connecting to: {}", url);

        let inner = Arc::new(
            WsClientBuilder::default()
                .connection_timeout(std::time::Duration::from_secs(10))
                .build(url)
                .await?,
        );

        log::debug!("Connected");

        Ok(Client {
            inner: Arc::clone(&inner),
            vm: VmProcedures {
                inner: Arc::clone(&inner),
            },
            xo: XoProcedures {
                inner: Arc::clone(&inner),
            },
            token: TokenProcedures {
                inner: Arc::clone(&inner),
            },
            session: SessionProcedures {
                inner: Arc::clone(&inner),
            },
        })
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

    /// Get all objects of specified type from server
    /// * `R` is a type that can represent that collection of objects
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    async fn get_objects_of_type<R: XoObjectMap>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<R, RpcError> {
        let mut filter = filter.into().unwrap_or_default();
        filter.insert("type".to_string(), R::Object::OBJECT_TYPE.into());

        self.xo.get_all_objects(filter, limit).await
    }

    /// Get single object of specified type from server
    /// * `R` is a type that can represent that type of object
    /// * `id` is the id of the object
    async fn get_object_of_type<R: XoObject>(
        &self,
        id: R::IdType,
    ) -> Result<Option<R>, GetSingleObjectError>
    where
        R::IdType: Ord,
    {
        let filter = procedure_object!(
            "id" => id.clone(),
            "type" => R::OBJECT_TYPE.to_string()
        );

        // TODO: Can we get rid of the BTreeMap here?
        let mut result: BTreeMap<R::IdType, R> = self
            .xo
            .get_all_objects(filter, Some(2))
            .await
            .map_err(GetSingleObjectError::Rpc)?;

        match result.remove(&id) {
            None => Ok(None),
            Some(vm) if result.is_empty() => Ok(Some(vm)),
            _ => Err(GetSingleObjectError::MultipleMatches),
        }
    }

    /// Get one VM from server
    pub async fn get_vm<O: OtherInfo>(
        &self,
        id: VmId,
    ) -> Result<Option<Vm<O>>, GetSingleObjectError> {
        self.get_object_of_type::<Vm<O>>(id).await
    }

    /// Get all VMs from server
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    pub async fn get_vms<O: OtherInfo>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<BTreeMap<VmId, Vm<O>>, RpcError> {
        self.get_objects_of_type(filter, limit).await
    }

    declare_object_getter!(Snapshot : single: fn get_snapshot, multi: fn get_snapshots);
}
