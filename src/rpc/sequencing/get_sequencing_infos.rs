use std::collections::HashMap;

use sequencer::{models::SequencingInfoModel, types::SequencingInfo};

use crate::rpc::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetSequencingInfos {}

// TODO:
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLivenessInfosResponse {
    sequencing_infos: HashMap<String, SequencingInfo>,
}

impl GetSequencingInfos {
    pub const METHOD_NAME: &'static str = "get_sequencing_infos";

    pub async fn handler(
        _parameter: RpcParameter,
        _context: Arc<()>,
    ) -> Result<GetLivenessInfosResponse, RpcError> {
        let sequencing_info_model = SequencingInfoModel::get()?;

        println!(
            "sequencing_infos: {:?}",
            sequencing_info_model.sequencing_infos()
        );

        let sequencing_infos = sequencing_info_model
            .sequencing_infos()
            .clone()
            .into_iter()
            .map(|(sequencing_info_key, sequencing_info)| {
                (sequencing_info_key.to_string(), sequencing_info)
            })
            .collect();

        Ok(GetLivenessInfosResponse { sequencing_infos })
    }
}
