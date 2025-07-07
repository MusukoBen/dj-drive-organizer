use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::types::*;
use crate::services::*;

/// Hauptseite der Anwendung
#[function_component]
pub fn MainPage() -> Html {
    let app_state = use_app_state();
    
    // Initialisiere Mock-Daten beim ersten Laden
    use_effect_with((), {
        let app_state = app_state.clone();
        move |_| {
            let mut state = app_state.borrow_mut();
            if state.tracks.is_empty() {
                state.tracks = TrackService::create_mock_tracks();
                state.drives = TrackService::create_mock_drives();
            }
            || {}
        }
    });
    
    html! {
        <div class="flex h-screen bg-gray-900 text-white">
            <Sidebar />
            <div class="flex-1 flex flex-col">
                <Header />
                <MainContent />
            </div>
        </div>
    }
}

/// Sidebar-Komponente
#[function_component]
pub fn Sidebar() -> Html {
    let app_state = use_app_state();
    let drives = app_state.borrow().drives.clone();
    
    html! {
        <div class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
            // App-Logo und Titel
            <div class="p-4 border-b border-gray-700">
                <div class="flex items-center space-x-2">
                    <div class="w-8 h-8 bg-blue-600 rounded flex items-center justify-center">
                        <span class="text-white font-bold">{"🎵"}</span>
                    </div>
                    <span class="font-semibold text-lg">{"DJ-Drive Pro"}</span>
                </div>
            </div>
            
            // Laufwerke & Partitionen
            <div class="flex-1 p-4">
                <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide mb-3">
                    {"Drives & Partitions"}
                </h3>
                
                <div class="space-y-2">
                    { for drives.iter().map(|drive| html! {
                        <DriveItem key={drive.id.clone()} drive={drive.clone()} />
                    })}
                </div>
                
                // Add Drive Button
                <button class="w-full mt-4 p-2 text-left text-gray-400 hover:text-white hover:bg-gray-700 rounded flex items-center space-x-2">
                    <span>{"+"}</span>
                    <span>{"Add Drive"}</span>
                </button>
                
                // Partition Manager
                <button class="w-full mt-2 p-2 text-left text-gray-400 hover:text-white hover:bg-gray-700 rounded flex items-center space-x-2">
                    <span>{"⚙️"}</span>
                    <span>{"Partition Manager"}</span>
                </button>
            </div>
            
            // Footer mit Last Backup Info
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
    }
}

/// Einzelnes Laufwerk in der Sidebar
#[derive(Properties, PartialEq)]
pub struct DriveItemProps {
    pub drive: Drive,
}

#[function_component]
pub fn DriveItem(props: &DriveItemProps) -> Html {
    let drive = &props.drive;
    let used_space = drive.total_space - drive.free_space;
    let usage_percent = (used_space as f64 / drive.total_space as f64 * 100.0) as u32;
    
    // Farbe basierend auf Laufwerkstyp
    let (icon, color_class) = match drive.drive_type {
        DriveType::Main => ("💾", "text-blue-400"),
        DriveType::Backup => ("🔄", "text-green-400"),
        DriveType::External => ("🔌", "text-purple-400"),
    };
    
    html! {
        <div class="p-2 rounded hover:bg-gray-700 cursor-pointer">
            <div class="flex items-center space-x-2 mb-1">
                <span class={color_class}>{icon}</span>
                <span class="font-medium">{&drive.name}</span>
                if drive.is_connected {
                    <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                } else {
                    <div class="w-2 h-2 bg-red-400 rounded-full"></div>
                }
            </div>
            <div class="text-xs text-gray-400 ml-6">
                {format!("{}% used", usage_percent)}
            </div>
        </div>
    }
}

/// Header-Komponente mit Suche und Buttons
#[function_component]
pub fn Header() -> Html {
    let app_state = use_app_state();
    let search_query = app_state.borrow().search_query.clone();
    let is_syncing = app_state.borrow().is_syncing;
    
    let on_search_input = {
        let app_state = app_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut state = app_state.borrow_mut();
            state.search_query = input.value();
        })
    };
    
    let on_quick_sync = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            SyncService::start_sync(&app_state);
            // Simuliere Sync-Completion nach 3 Sekunden
            let app_state_clone = app_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(3000).await;
                SyncService::complete_sync(&app_state_clone);
            });
        })
    };
    
    html! {
        <div class="bg-gray-800 border-b border-gray-700 p-4">
            <div class="flex items-center justify-between">
                // Suchleiste und Filter
                <div class="flex items-center space-x-4 flex-1">
                    <div class="relative flex-1 max-w-md">
                        <input
                            type="text"
                            placeholder="Search tracks..."
                            value={search_query}
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
                
                // Action Buttons
                <div class="flex items-center space-x-3">
                    <button
                        onclick={on_quick_sync}
                        disabled={is_syncing}
                        class={format!("px-4 py-2 rounded-lg flex items-center space-x-2 {}",
                            if is_syncing { "bg-gray-600 cursor-not-allowed" } else { "bg-blue-600 hover:bg-blue-700" }
                        )}
                    >
                        <span>{"🔄"}</span>
                        <span>{if is_syncing { "Syncing..." } else { "Quick Sync" }}</span>
                    </button>
                    
                    <button class="px-4 py-2 bg-green-600 hover:bg-green-700 rounded-lg flex items-center space-x-2">
                        <span>{"✓"}</span>
                        <span>{"Health Check"}</span>
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Hauptinhalt mit Tabs und Track-Tabelle
#[function_component]
pub fn MainContent() -> Html {
    let app_state = use_app_state();
    let current_tab = app_state.borrow().current_tab.clone();
    
    html! {
        <div class="flex-1 flex flex-col">
            <TabNavigation />
            
            <div class="flex-1 p-4">
                {match current_tab {
                    AppTab::Tracks => html! { <TracksView /> },
                    AppTab::Playlists => html! { <PlaylistsView /> },
                    AppTab::SyncProfiles => html! { <SyncProfilesView /> },
                }}
            </div>
        </div>
    }
}

/// Tab-Navigation
#[function_component]
pub fn TabNavigation() -> Html {
    let app_state = use_app_state();
    let current_tab = app_state.borrow().current_tab.clone();
    
    let on_tracks_click = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let mut state = app_state.borrow_mut();
            state.current_tab = AppTab::Tracks;
        })
    };
    
    let on_playlists_click = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let mut state = app_state.borrow_mut();
            state.current_tab = AppTab::Playlists;
        })
    };
    
    let on_sync_profiles_click = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let mut state = app_state.borrow_mut();
            state.current_tab = AppTab::SyncProfiles;
        })
    };
    
    html! {
        <div class="border-b border-gray-700">
            <div class="flex space-x-8 px-4">
                <TabButton
                    label="Tracks"
                    active={matches!(current_tab, AppTab::Tracks)}
                    onclick={on_tracks_click}
                />
                <TabButton
                    label="Playlists"
                    active={matches!(current_tab, AppTab::Playlists)}
                    onclick={on_playlists_click}
                />
                <TabButton
                    label="Sync Profiles"
                    active={matches!(current_tab, AppTab::SyncProfiles)}
                    onclick={on_sync_profiles_click}
                />
            </div>
        </div>
    }
}

/// Einzelner Tab-Button
#[derive(Properties, PartialEq)]
pub struct TabButtonProps {
    pub label: String,
    pub active: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn TabButton(props: &TabButtonProps) -> Html {
    let class = if props.active {
        "py-3 px-1 border-b-2 border-blue-500 text-blue-400 font-medium"
    } else {
        "py-3 px-1 text-gray-400 hover:text-white cursor-pointer"
    };
    
    html! {
        <button onclick={props.onclick.clone()} class={class}>
            {&props.label}
        </button>
    }
}

/// Tracks-Ansicht (Haupttabelle)
#[function_component]
pub fn TracksView() -> Html {
    let app_state = use_app_state();
    let state = app_state.borrow();
    let filtered_tracks = TrackService::filter_tracks(
        &state.tracks,
        &state.search_query,
        &state.filter_genre
    );
    
    html! {
        <div class="bg-gray-800 rounded-lg overflow-hidden">
            // Tabellen-Header
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
            
            // Track-Zeilen
            <div class="divide-y divide-gray-700">
                { for filtered_tracks.iter().map(|track| html! {
                    <TrackRow key={track.id.clone()} track={track.clone()} />
                })}
            </div>
        </div>
    }
}

/// Einzelne Track-Zeile
#[derive(Properties, PartialEq)]
pub struct TrackRowProps {
    pub track: Track,
}

#[function_component]
pub fn TrackRow(props: &TrackRowProps) -> Html {
    let track = &props.track;
    
    let status_icon = match track.status {
        TrackStatus::Synced => ("✓", "text-green-400"),
        TrackStatus::Warning => ("⚠", "text-yellow-400"),
        TrackStatus::Error => ("✗", "text-red-400"),
        TrackStatus::Pending => ("○", "text-gray-400"),
    };
    
    html! {
        <div class="px-4 py-3 hover:bg-gray-750 cursor-pointer">
            <div class="grid grid-cols-12 gap-4 items-center text-sm">
                // Status
                <div class="col-span-1">
                    <span class={format!("text-lg {}", status_icon.1)}>{status_icon.0}</span>
                </div>
                
                // Title
                <div class="col-span-3">
                    <div class="flex items-center space-x-2">
                        <span class="text-blue-400">{"🎵"}</span>
                        <span class="font-medium">{&track.title}</span>
                    </div>
                </div>
                
                // Artist
                <div class="col-span-2 text-gray-300">{&track.artist}</div>
                
                // BPM
                <div class="col-span-1 text-gray-300">{track.bpm}</div>
                
                // Genre
                <div class="col-span-2 text-gray-300">{&track.genre}</div>
                
                // Duration
                <div class="col-span-1 text-gray-300">{&track.duration}</div>
                
                // Partitions
                <div class="col-span-2">
                    <div class="flex space-x-1">
                        { for track.partitions.iter().map(|color| {
                            let color_class = match color {
                                PartitionColor::Blue => "bg-blue-500",
                                PartitionColor::Green => "bg-green-500",
                                PartitionColor::Purple => "bg-purple-500",
                                PartitionColor::Orange => "bg-orange-500",
                                PartitionColor::Red => "bg-red-500",
                            };
                            html! {
                                <div class={format!("w-3 h-3 rounded-full {}", color_class)}></div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Playlists-Ansicht (Platzhalter)
#[function_component]
pub fn PlaylistsView() -> Html {
    html! {
        <div class="text-center py-12">
            <div class="text-6xl mb-4">{"🎵"}</div>
            <h2 class="text-2xl font-bold mb-2">{"Playlists"}</h2>
            <p class="text-gray-400">{"Playlist management coming soon..."}</p>
        </div>
    }
}

/// Sync Profiles-Ansicht (Platzhalter)
#[function_component]
pub fn SyncProfilesView() -> Html {
    html! {
        <div class="text-center py-12">
            <div class="text-6xl mb-4">{"🔄"}</div>
            <h2 class="text-2xl font-bold mb-2">{"Sync Profiles"}</h2>
            <p class="text-gray-400">{"Sync profile management coming soon..."}</p>
        </div>
    }
}
