use leptos::*;
use crate::ckad_data::Concept;

#[component]
pub fn ConceptCard(concept: Concept) -> impl IntoView {
    let (is_copied, set_is_copied) = create_signal(false);

    let copy_to_clipboard = move |command: String| {
        move |_| {
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let promise = clipboard.write_text(&command);
                let _ = promise;
                set_is_copied.set(true);

                set_timeout(
                    move || {
                        set_is_copied.set(false);
                    },
                    std::time::Duration::from_secs(2)
                );
            }
        }
    };

    let command_clone = concept.command.clone();

    view! {
        <div class="concept-card">
            <div class="concept-header">
                <h3 class="concept-title">{concept.title}</h3>
                <button
                    class="copy-button"
                    class:copied=move || is_copied.get()
                    on:click=copy_to_clipboard(command_clone)
                >
                    {move || if is_copied.get() { "✓ Copied!" } else { "Copy" }}
                </button>
            </div>

            {concept.description.map(|desc| {
                view! {
                    <p class="concept-description">{desc}</p>
                }
            })}

            <pre class="command-block">
                <code>{concept.command}</code>
            </pre>
        </div>
    }
}
