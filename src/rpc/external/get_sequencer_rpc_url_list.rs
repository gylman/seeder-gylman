use std::sync::Arc;

use crate::{address::Address, rpc::prelude::*, state::AppState, types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlList {
    sequencer_address_list: Vec<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencerRpcUrlListResponse {
    pub sequencer_rpc_url_list: Vec<(String, Option<String>)>,
}

impl GetSequencerRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_sequencer_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<GetSequencerRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<GetSequencerRpcUrlList>()?;

        let sequencer_rpc_url_list: Vec<(String, Option<String>)> = parameter
            .sequencer_address_list
            .into_iter()
            .filter_map(|address| {
                let address = address.to_string();
                SequencerNodeInfoModel::get(&address)
                    .ok()
                    .map(|sequencer| (address, sequencer.rpc_url))
            })
            .collect();

        Ok(GetSequencerRpcUrlListResponse {
            sequencer_rpc_url_list,
        })
    }
}
