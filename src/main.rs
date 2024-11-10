mod markdown;

use std::{collections::HashMap, panic};

use leptos::*;
use markdown::Markdown;
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
    caveats: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
struct IntegrationSupport {
    import: Feature,
    export: Feature,
}

#[derive(Debug, Clone)]
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
            <div class="content">
            <h1>"Engine Query"</h1>
            <p>
                Answer all your queries about query engines.
            </p>
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
                                <div class="logo-in-th-container"><img class="logo-in-th" src={format!("static/images/{}", integration.logo)} /></div>
                                <div class="th-rotated-text">
                                    <span>
                                        {&integration.short_name}
                                    </span>
                                </div>
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
                                    class="support-matrix-cell-td"
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
                                        let support_text = if let Some(support) = support {
                                            let support_text = match (support.import.supported, support.export.supported) {
                                                (true, true) => "✅",
                                                (true, false) => "🔎",
                                                (false, true) => "✍️",
                                                (false, false) => "❌",
                                            };
                                            let has_caveats = support.import.caveats.is_some() || support.export.caveats.is_some();
                                            let caveats_text = if has_caveats {
                                                "*"
                                            } else {
                                                ""
                                            };
                                            format!("{}{}", support_text, caveats_text)
                                        } else {
                                            "❓".to_string()
                                        };
                                        view! {
                                            <div class="support-matrix-cell">{ support_text }</div>
                                        }
                                    }
                                </td>
                            </For>
                        </tr>
                    </For>
                </tbody>
            </table>
            <p>
            <b>Legend:</b>
                <ul>
                    <li>"✅ = Can read & write"</li>
                    <li>"🔎 = Can read but not write"</li>
                    <li>"✍️ = Can write but not read"</li>
                    <li>"❌ = Not supported"</li>
                    <li>"❓ = Unknown ("<a href="https://github.com/datamindedbe/playground-engine-query/edit/main/support_matrix.yaml">"please contribute!"</a>")"</li>
                </ul>
            </p>
            {
                move || {
                    if let Some(c) = clicked_on_cell.get() {
                        let support = support_matrix.get().get(&c.query_engine.id).and_then(|qe_support_map| qe_support_map.get(&c.integration.id)).cloned();
                        view! {
                            <div class="support-details-popup" style={move || format!("--clicked-at-x: {}px; --clicked-at-y: {}px;", c.mouse_x, c.mouse_y)}>
                                <img
                                    class="popup-close-button"
                                    src="static/images/cross-mark.png"
                                    on:click={
                                        move |_mouse_event| {
                                            set_clicked_on_cell(None)
                                        }
                                    }
                                />
                                <p class="popup-subtitle">"Can "
                                    <img class="logo-small" src={format!("static/images/{}", c.query_engine.logo)} />
                                    <span class="logo-text">{&c.query_engine.short_name}</span>
                                    " "
                                    <span class="popup-subtitle-emphasis">"read"</span>
                                    " from "
                                    <img class="logo-small" src={format!("static/images/{}", c.integration.logo)} />
                                    <span class="logo-text">{&c.integration.short_name}</span>
                                    " ?"
                                </p>
                                <p>
                                {
                                    if let Some(support) = &support {
                                        let evidence_str = if let Some(caveats) = &support.import.caveats {
                                            format!("{}\n\n⚠️ {}", support.import.evidence, caveats)
                                        } else {
                                            support.import.evidence.clone()
                                        };
                                        if support.import.supported {
                                            view! {
                                                <div>
                                                    <p><span class="popup-yesno">"Yes."</span></p>
                                                    <p><Markdown src={evidence_str} /></p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div>
                                                    <p><span class="popup-yesno">"No."</span></p>
                                                    <p><Markdown src={evidence_str} /></p>
                                                </div>
                                            }
                                        }
                                    } else {
                                        view! {
                                            <div>
                                                <p>"Unknown ("<a href="https://github.com/datamindedbe/playground-engine-query/edit/main/support_matrix.yaml">"please contribute!"</a>")"</p>
                                            </div>
                                        }   
                                    }
                                }
                                </p>
                                <p class="popup-subtitle">"Can "
                                    <img class="logo-small" src={format!("static/images/{}", c.query_engine.logo)} />
                                    <span class="logo-text">{&c.query_engine.short_name}</span>
                                    " "
                                    <span class="popup-subtitle-emphasis">"write"</span>
                                    " to "
                                    <img class="logo-small" src={format!("static/images/{}", c.integration.logo)} />
                                    <span class="logo-text">{&c.integration.short_name}</span>
                                    " ?"
                                </p>
                                <p>
                                {
                                    if let Some(support) = &support {
                                        let evidence_str = if let Some(caveats) = &support.export.caveats {
                                            format!("{}\n\n⚠️ {}", support.export.evidence, caveats)
                                        } else {
                                            support.export.evidence.clone()
                                        };
                                        if support.export.supported {
                                            view! {
                                                <div>
                                                    <p><span class="popup-yesno">"Yes."</span></p>
                                                    <p><Markdown src={evidence_str} /></p>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div>
                                                    <p><span class="popup-yesno">"No."</span></p>
                                                    <p><Markdown src={evidence_str} /></p>
                                                </div>
                                            }
                                        }
                                    } else {
                                        view! {
                                            <div>
                                                <p>"Unknown ("<a href="https://github.com/datamindedbe/playground-engine-query/edit/main/support_matrix.yaml">"please contribute!"</a>")"</p>
                                            </div>
                                        }   
                                    }
                                }
                                </p>
                            </div>
                        }
                    } else {
                        view! {
                            <div style="display: hidden;" />
                        }
                    }
                }
            }
            </div>
        }
    })
}
