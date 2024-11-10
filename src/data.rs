//! This file contains all logic related to parsing the YAML files
//! and querying the resulting data structures.

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct QueryEngine {
    pub id: String,
    pub short_name: String,
    pub description: String,
    pub url: String,
    pub logo: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Integration {
    pub id: String,
    pub short_name: String,
    pub description: String,
    pub logo: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feature {
    pub supported: bool,
    pub evidence: String,
    pub caveats: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationSupport {
    pub import: Feature,
    pub export: Feature,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub engines: Vec<QueryEngine>,
    pub integrations: Vec<Integration>,
    pub support_matrix: HashMap<String, HashMap<String, IntegrationSupport>>
}

impl Data {
    pub fn get_support(&self, engine: &str, integration: &str) -> Option<&IntegrationSupport> {
        self.support_matrix.get(engine)?.get(integration)
    }
}

const ENGINES_YAML: &str = include_str!("../query_engines.yaml");
const INTEGRATIONS_YAML: &str = include_str!("../integrations.yaml");
const SUPPORT_MATRIX_YAML: &str = include_str!("../support_matrix.yaml");

/**
 * Get the data parsed from the YAML files included statically in the binary.
 * Panics if the YAML files are not parsable.
 */
pub fn parse_data_from_static_yamls() -> &'static Data {
    static STATIC_DATA: std::sync::OnceLock<Data> = std::sync::OnceLock::new();
    STATIC_DATA.get_or_init(|| {
        let engines: Vec<QueryEngine> = serde_yaml::from_str(ENGINES_YAML).expect("Unable to parse YAML");
        let integrations: Vec<Integration> = serde_yaml::from_str(INTEGRATIONS_YAML).expect("Unable to parse YAML");
        let support_matrix: HashMap<String, HashMap<String, IntegrationSupport>> = serde_yaml::from_str(SUPPORT_MATRIX_YAML).expect("Unable to parse YAML");
        Data { engines, integrations, support_matrix }
    })
}
