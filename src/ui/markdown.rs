use leptos::*;

#[component]
pub fn Markdown(
    src: impl AsRef<str>
) -> impl IntoView {
    let compiled_html = markdown::to_html_with_options(
        src.as_ref(),
        &markdown::Options::gfm()
    );
    match compiled_html {
        Ok(compiled_html) => {
            view! {
                <div inner_html=compiled_html/>
            }
        },
        Err(e) => {
            view! {
                <div>{ format!("Invalid markdown: {}", e) }</div>
            }
        }
    }
}
