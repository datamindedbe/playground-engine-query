use leptos::*;

use crate::data::{Data, Integration, QueryEngine};
use crate::ui::common::{
    QueryEngineName,
    PleaseContributeLink,
    IntegrationDetails,
};

#[derive(Debug, Clone)]
struct ClickedCell {
    query_engine: QueryEngine,
    integration: Integration,
    mouse_x: i32,
    mouse_y: i32
}

#[component]
pub fn SupportMatrix(
    data: &'static Data
) -> impl IntoView {
    let (filtered_integrations, _set_filtered_integrations) = leptos::create_signal(data.integrations.clone());
    let (filtered_query_engines, _set_filtered_query_engines) = leptos::create_signal(data.engines.clone());

    let (
        clicked_on_cell,
        set_clicked_on_cell
    ) = leptos::create_signal(None);
    view! {
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
                            <QueryEngineName qe={&qe} />
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
                                    let support = data.get_support(&qe.id, &integration.id);
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
                <li>"❓ = Unknown ("<PleaseContributeLink />")"</li>
            </ul>
        </p>
        {
            move || {
                if let Some(c) = clicked_on_cell.get() {
                    let support = data.get_support(&c.query_engine.id, &c.integration.id);
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
                            <IntegrationDetails
                                query_engine={&c.query_engine}
                                integration={&c.integration}
                                support={support}
                                read={true}
                            />
                            <IntegrationDetails
                                query_engine={&c.query_engine}
                                integration={&c.integration}
                                support={support}
                                read={false}
                            />
                        </div>
                    }
                } else {
                    view! {
                        <div style="display: hidden;" />
                    }
                }
            }
        }
    }
}
