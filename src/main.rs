use std::{collections::HashMap, fs, panic};

use leptos::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QueryEngine {
    id: String,
    short_name: String,
    description: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Integration {
    id: String,
    short_name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct Feature {
    supported: bool,
    evidence: String,
}

#[derive(Debug, Deserialize)]
struct IntegrationSupport {
    import: Feature,
    export: Feature,
}

const ENGINES: &str = include_str!("../query_engines.yaml");
const INTEGRATIONS: &str = include_str!("../integrations.yaml");
const SUPPORT_MATRIX: &str = include_str!("../support_matrix.yaml");

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let query_engines: Vec<QueryEngine> =
        serde_yaml::from_str(ENGINES).expect("Unable to parse YAML");

    let integrations: Vec<Feature> =
        serde_yaml::from_str(INTEGRATIONS).expect("Unable to parse YAML");

    let parsed_data: HashMap<String, HashMap<String, IntegrationSupport>> =
        serde_yaml::from_str(SUPPORT_MATRIX).expect("Failed to parse YAML");

    // let debug_qe = format!("{:?}", query_engines);

    mount_to_body(move || {
        view! {
           <p>{format!("{:?}", query_engines)}</p>
           <p>{format!("{:?}", integrations)}</p>
            <p>{format!("{:?}", parsed_data)}</p>

        }
    })
}
