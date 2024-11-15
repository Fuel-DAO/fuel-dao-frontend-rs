use crate::{ outbound::collection_canister_calls::fetch_collections_data, state::canisters::Canisters};
use candid::{Nat, Principal};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
enum Tab {
    All,
    Available,
    Upcoming,
}

// Define a structure for the token metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TokenMetadata {
    weight: f64,
    drive_type: String,
    purchase_price: Nat,
    token: Principal,
    name: String,
    // Add other fields based on the metadata record
    // ...
}






#[component]
pub fn Collections() -> impl IntoView {
    
    let selected_tab = create_rw_signal(Tab::All);

    // Create a resource to fetch collection data and token metadata
    let collection_data = create_resource(
        move || (), // Dependency: none in this case
        move |_| {
            // let cans = cans.clone();
            async move { fetch_collections_data().await }
        },
    );

    view! {
        <section class="p-6 bg-gray-100">
            // Top Filter Bar
            <div class="flex justify-between items-center mb-8">
                <div class="flex space-x-4">
                    <Tabs selected_tab tab=Tab::All label="All".to_string() />
                    <Tabs selected_tab tab=Tab::Available label="Available".to_string() />
                    <Tabs selected_tab tab=Tab::Upcoming label="Upcoming".to_string() />
                </div>

                // Sort Button (For future sorting functionality)
                <div>
                    <button class="flex items-center px-4 py-2 bg-white text-black rounded-full shadow-md font-medium">
                        "Sort"
                        <svg
                            class="w-4 h-4 ml-2"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 9l-7 7-7-7"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>

            // Card Grid
            <Suspense fallback=move || {
                view! { <div>"Loading collections..."</div> }
            }>
                {move || match collection_data.get() {
                    Some(Ok(collections)) => {
                        let filtered_cars = collections
                            .iter()
                            .filter(|collection| {
                                match selected_tab.get() {
                                    Tab::All => true,
                                    Tab::Available => collection.status == "Available",
                                    Tab::Upcoming => collection.status == "Upcoming",
                                }
                            })
                            .collect::<Vec<_>>();
                        view! {
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                                {filtered_cars
                                    .into_iter()
                                    .map(|collection| {
                                        let href = format!(
                                            "/collections/{}/{}",
                                            collection.id.token_canister.to_text(),
                                            collection.id.asset_canister.to_text(),
                                        );
                                        view! {
                                            <a href=href class="block">
                                                <div class="bg-white rounded-lg shadow-md overflow-hidden">
                                                    <div class="relative">
                                                        <img
                                                            src="/public/img/car_image.jpg"
                                                            alt=collection.name.clone()
                                                            class="w-full h-48 object-cover"
                                                        />
                                                        <span class="absolute top-2 left-2 bg-white text-black font-semibold text-xs px-2 py-1 rounded-full">
                                                            {collection.status.clone()}
                                                        </span>
                                                    </div>
                                                    <div class="p-4">
                                                        <h3 class="text-lg font-semibold">
                                                            {collection.name.clone()}
                                                        </h3>
                                                        <p class="text-sm text-gray-600">
                                                            {collection
                                                                .metadata
                                                                .as_ref()
                                                                .map_or(
                                                                    "No description available.".to_string(),
                                                                    |meta| meta.drive_type.clone(),
                                                                )}
                                                        </p>
                                                    </div>
                                                </div>
                                            </a>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        }
                    }
                    Some(Err(e)) => {
                        view! { <div>{format!("Error fetching collections: {}", e)}</div> }
                    }
                    None => view! { <div>"Loading..."</div> },
                }}
            </Suspense>
        </section>
    }
}

#[component]
fn Tabs(selected_tab: RwSignal<Tab>, tab: Tab, label: String) -> impl IntoView {
    let current_tab = tab.clone();
    view! {
        <button
            class=move || {
                format!(
                    "px-4 py-2 rounded-md shadow-md font-medium {}",
                    if selected_tab.get() == current_tab {
                        "bg-white text-black"
                    } else {
                        "text-gray-500"
                    },
                )
            }
            on:click=move |_| selected_tab.set(tab.clone())
        >
            {label}
        </button>
    }
}
