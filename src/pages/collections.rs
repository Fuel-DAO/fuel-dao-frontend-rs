use leptos::*;

#[derive(Clone, PartialEq)]
enum Tab {
    All,
    Available,
    Upcoming,
}

#[component]
pub fn Collections() -> impl IntoView {
    // Signal to track the selected tab
    let selected_tab = create_rw_signal(Tab::All);

    // Example data: You can replace this with your actual car data
    let cars = vec![
        ("Model S Plaid - SAMPLE", "The Tesla Model S Plaid is the epitome of electric luxury and performance...", "Live", "Available"),
        ("Test EV Vehicle", "asdfasdsads", "Live", "Upcoming"),
        ("Tesla Electric Sample 1", "This is a test listing for Dfinity R&D Demo.", "Live", "Available"),
    ];

    // Filter cars based on the selected tab
    let filtered_cars = move || {
        match selected_tab.get() {
            Tab::All => cars.clone(),
            Tab::Available => cars.iter().filter(|(_, _, _, status)| *status == "Available").cloned().collect(),
            Tab::Upcoming => cars.iter().filter(|(_, _, _, status)| *status == "Upcoming").cloned().collect(),
        }
    };

    

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
                        <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                        </svg>
                    </button>
                </div>
            </div>

            // Card Grid
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                {move || filtered_cars().into_iter().map(|(title, description, status, _)| view! {
                    <div class="bg-white rounded-lg shadow-md overflow-hidden">
                        <div class="relative">
                            <img src="/public/img/car_image.jpg" alt=title class="w-full h-48 object-cover"/>
                            <span class="absolute top-2 left-2 bg-white text-black font-semibold text-xs px-2 py-1 rounded-full">{status}</span>
                        </div>
                        <div class="p-4">
                            <h3 class="text-lg font-semibold">{title}</h3>
                            <p class="text-sm text-gray-600">{description}</p>
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}


#[component]
fn Tabs(selected_tab: RwSignal<Tab>, tab: Tab, label: String ) -> impl IntoView {
    let current = selected_tab.clone();
    let current_tab = tab.clone();
    view! {
        <button 
            class=move || format!(
                "px-4 py-2 rounded-md shadow-md font-medium {}",
                if selected_tab.get() == current_tab.clone() { "bg-white text-black" } else { "text-gray-500" }
            )
            on:click=move |_| current.set(tab.clone())
        >
            {label}
        </button>
    }
}