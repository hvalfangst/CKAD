use leptos::*;

#[component]
pub fn SearchBar(
    search_query: ReadSignal<String>,
    set_search_query: WriteSignal<String>,
) -> impl IntoView {
    let on_input = move |ev| {
        let value = event_target_value(&ev);
        set_search_query.set(value);
    };

    let clear_search = move |_| {
        set_search_query.set(String::new());
    };

    view! {
        <div class="search-container">
            <div class="search-bar">
                <input
                    type="text"
                    class="search-input"
                    placeholder="Search commands, concepts, or resources..."
                    prop:value=move || search_query.get()
                    on:input=on_input
                />
                <button
                    class="clear-button"
                    class:hidden=move || search_query.get().is_empty()
                    on:click=clear_search
                >
                    "✕"
                </button>
            </div>
            {move || {
                let query = search_query.get();
                if !query.is_empty() {
                    view! {
                        <p class="search-info">
                            "Searching for: " <strong>{query}</strong>
                        </p>
                    }
                } else {
                    view! {
                        <p class="search-info">"Browse all CKAD exam concepts below"</p>
                    }
                }
            }}
        </div>
    }
}
