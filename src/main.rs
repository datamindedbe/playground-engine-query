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

    let integrations: Vec<Integration> =
        serde_yaml::from_str(INTEGRATIONS).expect("Unable to parse YAML");

    let parsed_data: HashMap<String, HashMap<String, IntegrationSupport>> =
        serde_yaml::from_str(SUPPORT_MATRIX).expect("Failed to parse YAML");

    mount_to_body(move || {
        // view! {
        //    <p>{format!("{:?}", query_engines)}</p>
        //    <p>{format!("{:?}", integrations)}</p>
        //     <p>{format!("{:?}", parsed_data)}</p>

        // }

        let row_keys: Vec<_> = parsed_data.keys().cloned().collect();
        let mut column_keys = std::collections::HashSet::new();
        for row in parsed_data.values() {
            for col in row.keys() {
                column_keys.insert(col.clone());
            }
        }
        let column_keys: Vec<_> = column_keys.into_iter().collect();

        view! {
            <h1>"Engine Query"</h1>
            <table>
                <thead>
                    <tr>
                        <th></th>
                        {column_keys.iter().map(|col| view! { <th>{col}</th> }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    {row_keys.iter().map(|row| view! {
                        <tr>
                            <td>{row}</td>
                            {column_keys.iter().map(|col| {
                                let cell_data = parsed_data.get(row).and_then(|inner| inner.get(col));
                                view! {
                                    <td>
                                        {if let Some(support) = cell_data {
                                            view! {
                                                <p>
                                                     {
                                                        match (support.import.supported, support.export.supported) {
                                                            (true, true) => "✅",
                                                            (true, false) => "🔎",
                                                            (false, true) => "✍️",
                                                            (false, false) => "❌",
                                                        }
                                                    }
                                                </p>

                                            }
                                        } else {
                                            view! {<p>"N/A"</p> }
                                        }}
                                    </td>
                                }
                            }).collect::<Vec<_>>()}
                        </tr>
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        }
    })
}
