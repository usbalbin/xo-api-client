use std::sync::Arc;

use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams};
use jsonrpsee_ws_client::WsClient;

use crate::{credentials::Credentials, RpcError};

pub struct SessionProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl SessionProcedures {
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
}

#[derive(serde::Deserialize)]
struct SigninResponse {}
