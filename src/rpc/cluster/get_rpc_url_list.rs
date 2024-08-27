use std::sync::Arc;

use radius_sequencer_sdk::{
    liveness::publisher::Publisher,
    signature::{ChainType, Signature},
};
use tracing::info;

use crate::{error::Error, models::prelude::*, rpc::prelude::*, sequencer_types::prelude::*};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GetRpcUrlListMessage {
    address: Vec<u8>,
    chain_type: ChainType,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,
    cluster_id: ClusterId,
}

impl std::fmt::Display for GetRpcUrlListMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlList {
    signature: Signature,
    message: GetRpcUrlListMessage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRpcUrlListResponse {
    pub rpc_url_list: Vec<(Address, IpAddress)>,
}

impl GetRpcUrlList {
    pub const METHOD_NAME: &'static str = "get_rpc_url_list";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<Publisher>,
    ) -> Result<GetRpcUrlListResponse, RpcError> {
        let parameter = parameter.parse::<GetRpcUrlList>()?;

        info!("get_rpc_url_list: {:?}", parameter.message.cluster_id);

        // verify siganture
        parameter.signature.verify_signature(
            parameter.message.to_string().as_bytes(),
            &parameter.message.address,
            parameter.message.chain_type,
        )?;

        if !context
            .is_registered(parameter.message.cluster_id.clone())
            .await?
        {
            tracing::error!("Not registered on the Liveness contract.");

            // return Err(Error::Publisher(
            //     radius_sequencer_sdk::liveness::publisher::PublisherError::IsRegistered(
            //         "Not registered on the Liveness contract.".to_string(),
            //     ),
            // )
            // .into());
        }

        let block_height = context.get_block_number().await?;

        let sequencer_list = context
            .get_sequencer_list(parameter.message.cluster_id.clone(), block_height)
            .await?;

        let platform = match parameter.message.chain_type {
            ChainType::Ethereum => PlatForm::Ethereum,
            _ => PlatForm::Local,
        };

        let address_list = match parameter.message.sequencing_function_type {
            SequencingFunctionType::Liveness => {
                LivenessClusterModel::get(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?
                .sequencer_address_list
            }
            SequencingFunctionType::Validation => {
                ValidationClusterModel::get(
                    &platform,
                    &parameter.message.service_type,
                    &parameter.message.cluster_id,
                )?
                .validator_address_list
            }
        };

        let rpc_url_list: Vec<(Address, IpAddress)> = address_list
            .iter()
            .filter_map(|sequencer_rpc_address| {
                let mut sized_address: [u8; 20] = [0; 20];
                sized_address.copy_from_slice(&sequencer_rpc_address.to_string().as_bytes()[..20]);
                let alloy_address =
                    radius_sequencer_sdk::liveness::types::Address::new(sized_address);

                if sequencer_list.contains(&alloy_address) {
                    SequencerModel::get(sequencer_rpc_address.clone())
                        .ok()
                        .and_then(|sequencer_model| {
                            sequencer_model
                                .rpc_url
                                .map(|rpc_url| (sequencer_model.address, rpc_url))
                        })
                } else {
                    None
                }
            })
            .collect();

        Ok(GetRpcUrlListResponse { rpc_url_list })
    }
}
