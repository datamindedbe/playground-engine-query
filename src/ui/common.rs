//! This file contains several common components reused throughout the UI.

use leptos::*;

use crate::data::{QueryEngine, Integration, IntegrationSupport};
use crate::ui::markdown::Markdown;

#[component]
pub fn QueryEngineName<'qe>(qe: &'qe QueryEngine) -> impl IntoView {
    view! {
        <img class="logo-small" src={format!("static/images/{}", qe.logo.clone())} />
        <span class="logo-text">{qe.short_name.clone()}</span>
    }
}

#[component]
pub fn IntegrationName<'i>(integration: &'i Integration) -> impl IntoView {
    view! {
        <img class="logo-small" src={format!("static/images/{}", integration.logo.clone())} />
        <span class="logo-text">{integration.short_name.clone()}</span>
    }
}

#[component]
pub fn PleaseContributeLink() -> impl IntoView {
    view! {
        <a href="https://github.com/datamindedbe/playground-engine-query/edit/main/support_matrix.yaml">"please contribute!"</a>
    }
}

#[component]
pub fn IntegrationDetails<'a>(
    query_engine: &'a QueryEngine,
    integration: &'a Integration,
    support: Option<&'a IntegrationSupport>,
    read: bool,
) -> impl IntoView {
    let read_or_write_str = if read { "read" } else { "write" };
    let to_or_from_str = if read { "from" } else { "to" };
    view! {
        <p class="popup-subtitle">"Can "
            <QueryEngineName qe={&query_engine} />
            " "
            <span class="popup-subtitle-emphasis">{ read_or_write_str }</span>
            " "
            { to_or_from_str }
            " "
            <IntegrationName integration={&integration} />
            " ?"
        </p>
        <p>
        {
            if let Some(support) = support {
                let feature = if read { &support.import } else { &support.export };
                let evidence_str = if let Some(caveats) = &feature.caveats {
                    format!("{}\n\n⚠️ {}", feature.evidence, caveats)
                } else {
                    feature.evidence.clone()
                };
                if feature.supported {
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
                        <p>"Unknown ("<PleaseContributeLink />")"</p>
                    </div>
                }   
            }
        }
        </p>
    }
}
