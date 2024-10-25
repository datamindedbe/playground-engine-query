use std::{collections::HashMap, panic};

use leptos::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct QueryEngine {
    id: String,
    short_name: String,
    description: String,
    url: String,
    logo: String
}

#[derive(Debug, Deserialize, Clone)]
struct Integration {
    id: String,
    short_name: String,
    description: String,
    logo: String
}

#[derive(Debug, Deserialize, Clone)]
struct Feature {
    supported: bool,
    evidence: String,
}

#[derive(Debug, Deserialize, Clone)]
struct IntegrationSupport {
    import: Feature,
    export: Feature,
}

#[derive(Debug)]
struct ClickedCell {
    query_engine: QueryEngine,
    integration: Integration,
    mouse_x: i32,
    mouse_y: i32
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

    let support_matrix: HashMap<String, HashMap<String, IntegrationSupport>> =
        serde_yaml::from_str(SUPPORT_MATRIX).expect("Failed to parse YAML");

    let (
        clicked_on_cell,
        set_clicked_on_cell
    ) = leptos::create_signal(None);

    let (support_matrix, _) = leptos::create_signal(support_matrix);

    let (filtered_integrations, _set_filtered_integrations) = leptos::create_signal(integrations.clone());
    let (filtered_query_engines, _set_filtered_query_engines) = leptos::create_signal(query_engines.clone());

    mount_to_body(move || {
        view! {
            <h1>"Engine Query"</h1>
            <table class="styled-table">
                <thead>
                    <tr>
                        <th></th>
                        <For
                            each=filtered_integrations
                            key=|i| i.id.clone()
                            let:integration
                        > 
                            <th>
                                <img class="logo-small" src={format!("static/images/{}", integration.logo)} />
                                <span class="logo-text">{&integration.short_name}</span>
                            </th>
                        </For>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=filtered_query_engines
                        key=|qe| qe.id.clone()
                        let:qe
                    >
                        <tr>
                            <td>
                                <img class="logo-small" src={format!("static/images/{}", qe.logo)} />
                                <span class="logo-text">{&qe.short_name}</span>
                            </td>
                            <For
                                each=filtered_integrations
                                key=|i| i.id.clone()
                                let:integration
                            >
                                <td
                                    on:click={
                                        // TODO: not sure if all these clone()s are necessary
                                        let qe = qe.clone();
                                        let integration = integration.clone();
                                        move |mouse_event| {
                                            set_clicked_on_cell(
                                                Some(ClickedCell {
                                                    query_engine: qe.clone(),
                                                    integration: integration.clone(),
                                                    mouse_x: mouse_event.client_x(),
                                                    mouse_y: mouse_event.client_y()
                                                })
                                            )
                                        }
                                    }
                                >
                                    {
                                        let support = support_matrix.get().get(&qe.id).and_then(|qe_support_map| qe_support_map.get(&integration.id)).cloned();
                                        if let Some(support) = support {
                                            view! {
                                                <div class="support-cell">
                                                    {
                                                        match (support.import.supported, support.export.supported) {
                                                            (true, true) => "✅",
                                                            (true, false) => "🔎",
                                                            (false, true) => "✍️",
                                                            (false, false) => "❌",
                                                        }
                                                    }
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="support-cell">
                                                    "❓"
                                                </div>
                                            }
                                        }
                                    }
                                </td>
                            </For>
                        </tr>
                    </For>
                </tbody>
            </table>
        }
    })
}
