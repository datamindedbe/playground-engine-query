mod markdown;
mod support_matrix;
mod common;

use leptos::*;
use support_matrix::SupportMatrix;
use crate::data::Data;

/**
 * The main function of the UI, draws the UI and mounts it to the HTML body.
 */
pub fn mount_ui_to_body(data: &'static Data) {
    mount_to_body(move || {
        view! {
            <div class="content">
                <h1>"Engine Query"</h1>
                <p>
                    Answer all your queries about query engines.
                </p>
                <SupportMatrix data={data} />
            </div>
        }
    })
}