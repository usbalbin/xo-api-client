use std::{collections::BTreeMap, sync::Arc};

use crate::{
    procedure_object,
    types::{XoObject, XoObjectMap},
    RpcError,
};

use jsonrpsee_types::{traits::Client, v2::params::ParamsSer, JsonValue};
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
            .request("xo.getAllObjects", Some(ParamsSer::Map(args)))
            .await
    }

    /// Get all objects of specified type from server
    /// * `R` is a type that can represent that collection of objects
    /// * `filter` is an optional filter
    /// * `limit` is an optional max limit on number of results
    pub async fn get_objects<R: XoObjectMap>(
        &self,
        filter: impl Into<Option<serde_json::Map<String, JsonValue>>>,
        limit: impl Into<Option<usize>>,
    ) -> Result<R, RpcError> {
        let mut filter = filter.into().unwrap_or_default();
        filter.insert("type".to_string(), R::Object::OBJECT_TYPE.into());

        self.get_all_objects(filter, limit).await
    }

    /// Get single object of specified type from server
    /// * `R` is a type that can represent that type of object
    /// * `id` is the id of the object
    pub async fn get_object<R: XoObject>(
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
            .get_all_objects(filter, Some(2))
            .await
            .map_err(GetSingleObjectError::Rpc)?;

        match result.remove(&id) {
            None => Ok(None),
            Some(vm) if result.is_empty() => Ok(Some(vm)),
            _ => Err(GetSingleObjectError::MultipleMatches),
        }
    }
}

#[derive(Debug)]
pub enum GetSingleObjectError {
    MultipleMatches,
    Rpc(RpcError),
}
