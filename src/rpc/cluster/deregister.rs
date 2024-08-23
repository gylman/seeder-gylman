use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    models::prelude::{LivenessClusterModel, ValidationClusterModel},
    rpc::prelude::*,
    sequencer_types::prelude::*,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deregister {
    platform: PlatForm,
    sequencing_function_type: SequencingFunctionType,
    service_type: ServiceType,

    cluster_id: ClusterId,
    address: Address,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeregisterResponse {
    pub success: bool,
}

impl Deregister {
    pub const METHOD_NAME: &'static str = "deregister";

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<()>,
    ) -> Result<DeregisterResponse, RpcError> {
        let parameter = parameter.parse::<Deregister>()?;

        match parameter.sequencing_function_type {
            SequencingFunctionType::Liveness => {
                let mut liveness_cluster_model = LivenessClusterModel::get_mut(
                    &parameter.platform,
                    &parameter.service_type,
                    &parameter.cluster_id,
                )?;

                liveness_cluster_model.remove_sequencer(&parameter.address);
                liveness_cluster_model.update()?;
            }

            SequencingFunctionType::Validation => {
                let mut validation_cluster_model = ValidationClusterModel::get_mut(
                    &parameter.platform,
                    &parameter.service_type,
                    &parameter.cluster_id,
                )?;

                validation_cluster_model.remove_validator(&parameter.address);
                validation_cluster_model.update()?;
            }
        }
        Ok(DeregisterResponse { success: true })
    }
}
