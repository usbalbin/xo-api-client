use std::sync::Arc;

use crate::RpcError;

use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams, JsonValue};
use jsonrpsee_ws_client::WsClient;

use crate::procedure_args;

pub struct XoProcedures {
    pub(crate) inner: Arc<WsClient>,
}

impl XoProcedures {
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
}
