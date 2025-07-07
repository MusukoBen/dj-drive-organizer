use yew::prelude::*;
use web_sys::HtmlInputElement;

/// Vereinfachte DJ-Drive-Organizer-App
#[function_component]
pub fn SimpleApp() -> Html {
    let search_query = use_state(|| String::new());
    let is_syncing = use_state(|| false);
    let current_tab = use_state(|| "Tracks".to_string());
    
    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };
    
    let on_sync_click = {
        let is_syncing = is_syncing.clone();
        Callback::from(move |_| {
            is_syncing.set(true);
            // Simuliere Sync nach 3 Sekunden
            let is_syncing_clone = is_syncing.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(3000).await;
                is_syncing_clone.set(false);
            });
        })
    };
    
    let on_tab_click = |tab: &'static str| {
        let current_tab = current_tab.clone();
        Callback::from(move |_| {
            current_tab.set(tab.to_string());
        })
    };
    
    html! {
        <div class="flex h-screen bg-gray-900 text-white">
            // Sidebar
            <div class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
                // Logo
                <div class="p-4 border-b border-gray-700">
                    <div class="flex items-center space-x-2">
                        <div class="w-8 h-8 bg-blue-600 rounded flex items-center justify-center">
                            <span class="text-white font-bold">{"🎵"}</span>
                        </div>
                        <span class="font-semibold text-lg">{"DJ-Drive Pro"}</span>
                    </div>
                </div>
                
                // Drives
                <div class="flex-1 p-4">
                    <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide mb-3">
                        {"Drives & Partitions"}
                    </h3>
                    
                    <div class="space-y-2">
                        <div class="p-2 rounded hover:bg-gray-700 cursor-pointer">
                            <div class="flex items-center space-x-2 mb-1">
                                <span class="text-blue-400">{"💾"}</span>
                                <span class="font-medium">{"DJ-Main"}</span>
                                <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                            </div>
                            <div class="text-xs text-gray-400 ml-6">{"75% used"}</div>
                        </div>
                        
                        <div class="p-2 rounded hover:bg-gray-700 cursor-pointer">
                            <div class="flex items-center space-x-2 mb-1">
                                <span class="text-green-400">{"🔄"}</span>
                                <span class="font-medium">{"Backup"}</span>
                                <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                            </div>
                            <div class="text-xs text-gray-400 ml-6">{"25% used"}</div>
                        </div>
                    </div>
                    
                    <button class="w-full mt-4 p-2 text-left text-gray-400 hover:text-white hover:bg-gray-700 rounded flex items-center space-x-2">
                        <span>{"+"}</span>
                        <span>{"Add Drive"}</span>
                    </button>
                    
                    <button class="w-full mt-2 p-2 text-left text-gray-400 hover:text-white hover:bg-gray-700 rounded flex items-center space-x-2">
                        <span>{"⚙️"}</span>
                        <span>{"Partition Manager"}</span>
                    </button>
                </div>
                
                // Footer
                <div class="p-4 border-t border-gray-700 text-xs text-gray-400">
                    <div class="flex items-center space-x-1 mb-1">
                        <span>{"🔄"}</span>
                        <span>{"Last Backup: 2 days ago"}</span>
                    </div>
                    <div class="flex items-center space-x-1">
                        <span>{"⚠️"}</span>
                        <span>{"3 tracks out of sync"}</span>
                    </div>
                </div>
            </div>
            
            // Main Content
            <div class="flex-1 flex flex-col">
                // Header
                <div class="bg-gray-800 border-b border-gray-700 p-4">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-4 flex-1">
                            <div class="relative flex-1 max-w-md">
                                <input
                                    type="text"
                                    placeholder="Search tracks..."
                                    value={(*search_query).clone()}
                                    oninput={on_search_input}
                                    class="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
                                />
                                <div class="absolute inset-y-0 right-0 flex items-center pr-3">
                                    <span class="text-gray-400">{"🔍"}</span>
                                </div>
                            </div>
                            
                            <button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg border border-gray-600 flex items-center space-x-2">
                                <span>{"🔽"}</span>
                                <span>{"Filter"}</span>
                            </button>
                        </div>
                        
                        <div class="flex items-center space-x-3">
                            <button
                                onclick={on_sync_click}
                                disabled={*is_syncing}
                                class={format!("px-4 py-2 rounded-lg flex items-center space-x-2 {}",
                                    if *is_syncing { "bg-gray-600 cursor-not-allowed" } else { "bg-blue-600 hover:bg-blue-700" }
                                )}
                            >
                                <span>{"🔄"}</span>
                                <span>{if *is_syncing { "Syncing..." } else { "Quick Sync" }}</span>
                            </button>
                            
                            <button class="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg flex items-center space-x-2">
                                <span>{"✓"}</span>
                                <span>{"Health Check"}</span>
                            </button>
                        </div>
                    </div>
                </div>
                
                // Tabs
                <div class="border-b border-gray-700">
                    <div class="flex space-x-8 px-4">
                        {["Tracks", "Playlists", "Sync Profiles"].iter().map(|&tab| {
                            let is_active = *current_tab == tab;
                            let class = if is_active {
                                "py-3 px-1 border-b-2 border-blue-500 text-blue-400 font-medium"
                            } else {
                                "py-3 px-1 text-gray-400 hover:text-white cursor-pointer"
                            };
                            
                            html! {
                                <button onclick={on_tab_click(tab)} class={class}>
                                    {tab}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
                
                // Content
                <div class="flex-1 p-4">
                    {match (*current_tab).as_str() {
                        "Tracks" => html! { <TracksContent search_query={(*search_query).clone()} /> },
                        "Playlists" => html! { <PlaceholderContent title="Playlists" icon="🎵" /> },
                        "Sync Profiles" => html! { <PlaceholderContent title="Sync Profiles" icon="🔄" /> },
                        _ => html! { <PlaceholderContent title="Unknown" icon="❓" /> },
                    }}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TracksContentProps {
    search_query: String,
}

#[function_component]
fn TracksContent(props: &TracksContentProps) -> Html {
    let tracks = vec![
        ("Midnight City", "M83", "105", "Synthwave", "4:03", "✓", "text-green-400", vec!["bg-purple-500", "bg-blue-500"]),
        ("Strobe", "Deadmau5", "128", "Progressive House", "10:34", "⚠", "text-yellow-400", vec!["bg-blue-500"]),
        ("One More Time", "Daft Punk", "123", "French House", "5:20", "✓", "text-green-400", vec!["bg-green-500", "bg-blue-500", "bg-purple-500"]),
        ("Levels", "Avicii", "126", "Progressive House", "5:42", "✗", "text-red-400", vec!["bg-purple-500"]),
        ("Animals", "Martin Garrix", "128", "Big Room", "5:03", "✓", "text-green-400", vec!["bg-green-500", "bg-blue-500"]),
    ];
    
    let filtered_tracks: Vec<_> = tracks.iter()
        .filter(|(title, artist, _, _, _, _, _, _)| {
            props.search_query.is_empty() ||
            title.to_lowercase().contains(&props.search_query.to_lowercase()) ||
            artist.to_lowercase().contains(&props.search_query.to_lowercase())
        })
        .collect();
    
    html! {
        <div class="bg-gray-800 rounded-lg overflow-hidden">
            // Header
            <div class="bg-gray-700 px-4 py-3 border-b border-gray-600">
                <div class="grid grid-cols-12 gap-4 text-sm font-medium text-gray-300">
                    <div class="col-span-1">{"Status"}</div>
                    <div class="col-span-3">{"Title"}</div>
                    <div class="col-span-2">{"Artist"}</div>
                    <div class="col-span-1">{"BPM"}</div>
                    <div class="col-span-2">{"Genre"}</div>
                    <div class="col-span-1">{"Duration"}</div>
                    <div class="col-span-2">{"Partitions"}</div>
                </div>
            </div>
            
            // Rows
            <div class="divide-y divide-gray-700">
                {filtered_tracks.iter().enumerate().map(|(i, (title, artist, bpm, genre, duration, status_icon, status_color, partitions))| {
                    html! {
                        <div key={i} class="px-4 py-3 hover:bg-gray-750 cursor-pointer">
                            <div class="grid grid-cols-12 gap-4 items-center text-sm">
                                <div class="col-span-1">
                                    <span class={format!("text-lg {}", status_color)}>{status_icon}</span>
                                </div>
                                <div class="col-span-3">
                                    <div class="flex items-center space-x-2">
                                        <span class="text-blue-400">{"🎵"}</span>
                                        <span class="font-medium">{title}</span>
                                    </div>
                                </div>
                                <div class="col-span-2 text-gray-300">{artist}</div>
                                <div class="col-span-1 text-gray-300">{bpm}</div>
                                <div class="col-span-2 text-gray-300">{genre}</div>
                                <div class="col-span-1 text-gray-300">{duration}</div>
                                <div class="col-span-2">
                                    <div class="flex space-x-1">
                                        {partitions.iter().enumerate().map(|(j, color_class)| {
                                            html! {
                                                <div key={j} class={format!("w-3 h-3 rounded-full {}", color_class)}></div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PlaceholderContentProps {
    title: String,
    icon: String,
}

#[function_component]
fn PlaceholderContent(props: &PlaceholderContentProps) -> Html {
    html! {
        <div class="text-center py-12">
            <div class="text-6xl mb-4">{&props.icon}</div>
            <h2 class="text-2xl font-bold mb-2">{&props.title}</h2>
            <p class="text-gray-400">{format!("{} management coming soon...", props.title)}</p>
        </div>
    }
}
