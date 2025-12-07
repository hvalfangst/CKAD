use leptos::*;
use crate::ckad_data::{get_ckad_concepts, Category};
use crate::components::{ConceptCard, SearchBar};

#[component]
pub fn CkadWiki() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_category, set_selected_category) = create_signal(None::<String>);

    let categories = create_memo(move |_| get_ckad_concepts());

    let filtered_categories = create_memo(move |_| {
        let query = search_query.get().to_lowercase();
        let selected = selected_category.get();
        let cats = categories.get();

        if query.is_empty() && selected.is_none() {
            return cats;
        }

        cats.into_iter()
            .filter_map(|category| {
                // Filter by selected category first
                if let Some(ref selected_cat) = selected {
                    if &category.name != selected_cat {
                        return None;
                    }
                }

                // Then filter concepts by search query
                if !query.is_empty() {
                    let filtered_concepts: Vec<_> = category.concepts
                        .into_iter()
                        .filter(|concept| {
                            concept.title.to_lowercase().contains(&query)
                                || concept.command.to_lowercase().contains(&query)
                                || concept.description.as_ref()
                                    .map(|d| d.to_lowercase().contains(&query))
                                    .unwrap_or(false)
                        })
                        .collect();

                    if filtered_concepts.is_empty() {
                        return None;
                    }

                    Some(Category {
                        name: category.name,
                        concepts: filtered_concepts,
                    })
                } else {
                    Some(category)
                }
            })
            .collect::<Vec<_>>()
    });

    let reset_filters = move |_| {
        set_search_query.set(String::new());
        set_selected_category.set(None);
    };

    view! {
        <div class="wiki-container">
            <div class="wiki-header">
                <h1>"CKAD Exam Wiki"</h1>
                <p class="subtitle">"Interactive guide for Certified Kubernetes Application Developer exam"</p>
            </div>

            <SearchBar search_query=search_query set_search_query=set_search_query />

            <div class="category-filter">
                <h3>"Filter by Category"</h3>
                <div class="category-buttons">
                    {move || {
                        categories.get().iter().map(|cat| {
                            let cat_name = cat.name.clone();
                            let cat_name_for_click = cat.name.clone();
                            let cat_name_display = cat.name.clone();

                            view! {
                                <button
                                    class="category-button"
                                    class:active=move || {
                                        selected_category.get()
                                            .as_ref()
                                            .map(|s| s == &cat_name)
                                            .unwrap_or(false)
                                    }
                                    on:click=move |_| {
                                        let is_selected = selected_category.get()
                                            .as_ref()
                                            .map(|s| s == &cat_name_for_click)
                                            .unwrap_or(false);

                                        if is_selected {
                                            set_selected_category.set(None);
                                        } else {
                                            set_selected_category.set(Some(cat_name_for_click.clone()));
                                        }
                                    }
                                >
                                    {cat_name_display}
                                </button>
                            }
                        }).collect_view()
                    }}
                </div>
                {move || {
                    if selected_category.get().is_some() || !search_query.get().is_empty() {
                        view! {
                            <button class="reset-button" on:click=reset_filters>
                                "Clear Filters"
                            </button>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>

            <div class="concepts-container">
                {move || {
                    let filtered = filtered_categories.get();
                    if filtered.is_empty() {
                        view! {
                            <div class="no-results">
                                <p>"No concepts found matching your search."</p>
                            </div>
                        }.into_view()
                    } else {
                        filtered.into_iter().map(|category| {
                            view! {
                                <div class="category-section">
                                    <h2 class="category-title">{category.name}</h2>
                                    <div class="concepts-grid">
                                        {category.concepts.into_iter().map(|concept| {
                                            view! {
                                                <ConceptCard concept=concept />
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view().into_view()
                    }
                }}
            </div>
        </div>
    }
}
