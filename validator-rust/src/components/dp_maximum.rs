use crate::errors::*;


use std::collections::HashMap;

use crate::{proto, base};
use crate::hashmap;
use crate::components::{Component, Expandable, Report};

use crate::base::{NodeProperties, Value, ValueProperties, Array};
use crate::utilities::json::{JSONRelease, AlgorithmInfo, privacy_usage_to_json, value_to_json};
use crate::utilities::{prepend, broadcast_privacy_usage, get_ith_release};


impl Component for proto::DpMaximum {
    fn propagate_property(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        _public_arguments: &HashMap<String, Value>,
        _properties: &base::NodeProperties,
    ) -> Result<ValueProperties> {
        Err("DPMaximum is abstract, and has no property propagation".into())
    }

}

impl Expandable for proto::DpMaximum {
    fn expand_component(
        &self,
        _privacy_definition: &proto::PrivacyDefinition,
        component: &proto::Component,
        _properties: &base::NodeProperties,
        component_id: &u32,
        maximum_id: &u32,
    ) -> Result<proto::ComponentExpansion> {
        let mut current_id = *maximum_id;
        let mut computation_graph: HashMap<u32, proto::Component> = HashMap::new();

        // Maximum
        current_id += 1;
        let id_maximum = current_id;
        computation_graph.insert(id_maximum, proto::Component {
            arguments: hashmap!["data".to_owned() => *component.arguments.get("data").unwrap()],
            variant: Some(proto::component::Variant::from(proto::Maximum {})),
            omit: true,
            batch: component.batch,
        });

//        let id_candidates = component.arguments.get("candidates").unwrap().clone();

        // sanitizing
        computation_graph.insert(component_id.clone(), proto::Component {
            arguments: hashmap!["data".to_owned() => id_maximum],
            variant: Some(proto::component::Variant::from(proto::LaplaceMechanism {
                privacy_usage: self.privacy_usage.clone()
            })),
            omit: false,
            batch: component.batch,
        });

        Ok(proto::ComponentExpansion {
            computation_graph,
            properties: HashMap::new(),
            releases: HashMap::new(),
            traversal: vec![id_maximum]
        })
    }
}

impl Report for proto::DpMaximum {
    fn summarize(
        &self,
        node_id: &u32,
        component: &proto::Component,
        _public_arguments: &HashMap<String, Value>,
        properties: &NodeProperties,
        release: &Value,
        variable_names: Option<&Vec<String>>,
    ) -> Result<Option<Vec<JSONRelease>>> {
        let data_property = properties.get("data")
            .ok_or("data: missing")?.array()
            .map_err(prepend("data:"))?.clone();

        let mut releases = Vec::new();

        let minimums = data_property.min_f64()?;
        let maximums = data_property.max_f64()?;

        let num_columns = data_property.num_columns()?;
        let privacy_usages = broadcast_privacy_usage(&self.privacy_usage, num_columns as usize)?;

        for column_number in 0..(num_columns as usize) {
            let variable_name = variable_names
                .and_then(|names| names.get(column_number)).cloned()
                .unwrap_or_else(|| "[Unknown]".to_string());

            releases.push(JSONRelease {
                description: "DP release information".to_string(),
                statistic: "DPMaximum".to_string(),
                variables: serde_json::json!(variable_name),
                release_info: match release.array()? {
                    Array::F64(v) => value_to_json(&get_ith_release(v, &column_number)?.into())?,
                    Array::I64(v) => value_to_json(&get_ith_release(v, &column_number)?.into())?,
                    _ => return Err("maximum must be numeric".into())
                },
                privacy_loss: privacy_usage_to_json(&privacy_usages[column_number].clone()),
                accuracy: None,
                batch: component.batch as u64,
                node_id: *node_id as u64,
                postprocess: false,
                algorithm_info: AlgorithmInfo {
                    name: "".to_string(),
                    cite: "".to_string(),
                    mechanism: self.implementation.clone(),
                    argument: serde_json::json!({
                        "constraint": {
                            "lowerbound": minimums[column_number],
                            "upperbound": maximums[column_number]
                        }
                    }),
                },
            });
        }
        Ok(Some(releases))
    }
}
