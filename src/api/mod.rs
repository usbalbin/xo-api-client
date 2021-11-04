pub mod session;
pub mod token;
pub mod vm;
pub mod xo;

use std::sync::Arc;

use jsonrpsee_types::{traits::SubscriptionClient, DeserializeOwned, Subscription};
use jsonrpsee_ws_client::{WsClient, WsClientBuilder};

use crate::RpcError;

use self::{
    session::SessionProcedures, token::TokenProcedures, vm::VmProcedures, xo::XoProcedures,
};

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
///         con.xo.get_objects(None, None).await.expect("Failed to list VMs");
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
}
