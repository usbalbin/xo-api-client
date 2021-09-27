use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee_types::{traits::Client, v2::params::ParamsSer};
use jsonrpsee_ws_client::WsClient;

use crate::{credentials::Token, RpcError};

pub struct TokenProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl TokenProcedures {
    /// Create authentication token
    ///
    /// xo-cli: token.create [expiresIn=<number|string>]
    pub async fn create(&self) -> Result<Token, RpcError> {
        // TODO: consider specifying the `expiresIn` parameter
        let token: Token = self
            .inner
            .request("token.create", Some(ParamsSer::Map(BTreeMap::new())))
            .await?;

        Ok(token)
    }
}
