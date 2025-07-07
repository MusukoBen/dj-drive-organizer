use std::rc::Rc;
use std::cell::RefCell;
use yew::prelude::*;
use crate::types::*;

/// Zentraler App-State-Service
pub type AppStateService = Rc<RefCell<AppState>>;

/// Hook für App-State-Zugriff
#[hook]
pub fn use_app_state() -> AppStateService {
    use_context::<AppStateService>().expect("AppStateService not found in context")
}

/// Context Provider für App-State
#[derive(Properties, PartialEq)]
pub struct AppStateProviderProps {
    pub children: Children,
}

#[function_component]
pub fn AppStateProvider(props: &AppStateProviderProps) -> Html {
    let state = use_state(|| Rc::new(RefCell::new(AppState::default())));
    
    html! {
        <ContextProvider<AppStateService> context={(*state).clone()}>
            { for props.children.iter() }
        </ContextProvider<AppStateService>>
    }
}

/// Service für Track-Management
pub struct TrackService;

impl TrackService {
    /// Erstellt Mock-Daten für Demo-Zwecke
    pub fn create_mock_tracks() -> Vec<Track> {
        vec![
            Track {
                id: "1".to_string(),
                title: "Midnight City".to_string(),
                artist: "M83".to_string(),
                genre: "Synthwave".to_string(),
                bpm: 105,
                duration: "4:03".to_string(),
                file_path: "/music/M83 - Midnight City.mp3".to_string(),
                file_size: 8_500_000,
                status: TrackStatus::Synced,
                partitions: vec![PartitionColor::Purple, PartitionColor::Blue],
            },
            Track {
                id: "2".to_string(),
                title: "Strobe".to_string(),
                artist: "Deadmau5".to_string(),
                genre: "Progressive House".to_string(),
                bpm: 128,
                duration: "10:34".to_string(),
                file_path: "/music/Deadmau5 - Strobe.mp3".to_string(),
                file_size: 24_300_000,
                status: TrackStatus::Warning,
                partitions: vec![PartitionColor::Blue],
            },
            Track {
                id: "3".to_string(),
                title: "One More Time".to_string(),
                artist: "Daft Punk".to_string(),
                genre: "French House".to_string(),
                bpm: 123,
                duration: "5:20".to_string(),
                file_path: "/music/Daft Punk - One More Time.mp3".to_string(),
                file_size: 12_100_000,
                status: TrackStatus::Synced,
                partitions: vec![PartitionColor::Green, PartitionColor::Blue, PartitionColor::Purple],
            },
            Track {
                id: "4".to_string(),
                title: "Levels".to_string(),
                artist: "Avicii".to_string(),
                genre: "Progressive House".to_string(),
                bpm: 126,
                duration: "5:42".to_string(),
                file_path: "/music/Avicii - Levels.mp3".to_string(),
                file_size: 13_200_000,
                status: TrackStatus::Error,
                partitions: vec![PartitionColor::Purple],
            },
            Track {
                id: "5".to_string(),
                title: "Animals".to_string(),
                artist: "Martin Garrix".to_string(),
                genre: "Big Room".to_string(),
                bpm: 128,
                duration: "5:03".to_string(),
                file_path: "/music/Martin Garrix - Animals.mp3".to_string(),
                file_size: 11_800_000,
                status: TrackStatus::Synced,
                partitions: vec![PartitionColor::Green, PartitionColor::Blue],
            },
        ]
    }
    
    /// Erstellt Mock-Laufwerke
    pub fn create_mock_drives() -> Vec<Drive> {
        vec![
            Drive {
                id: "main".to_string(),
                name: "DJ-Main".to_string(),
                drive_type: DriveType::Main,
                total_space: 1_000_000_000_000, // 1TB
                free_space: 250_000_000_000,    // 250GB
                mount_path: "/Volumes/DJ-Main".to_string(),
                is_connected: true,
            },
            Drive {
                id: "backup".to_string(),
                name: "Backup".to_string(),
                drive_type: DriveType::Backup,
                total_space: 2_000_000_000_000, // 2TB
                free_space: 1_500_000_000_000,  // 1.5TB
                mount_path: "/Volumes/Backup".to_string(),
                is_connected: true,
            },
        ]
    }
    
    /// Filtert Tracks basierend auf Suchkriterien
    pub fn filter_tracks(tracks: &[Track], query: &str, genre_filter: &Option<String>) -> Vec<Track> {
        tracks.iter()
            .filter(|track| {
                let matches_query = query.is_empty() || 
                    track.title.to_lowercase().contains(&query.to_lowercase()) ||
                    track.artist.to_lowercase().contains(&query.to_lowercase());
                
                let matches_genre = genre_filter.as_ref()
                    .map(|genre| &track.genre == genre)
                    .unwrap_or(true);
                
                matches_query && matches_genre
            })
            .cloned()
            .collect()
    }
}

/// Service für Sync-Operationen
pub struct SyncService;

impl SyncService {
    /// Startet einen Sync-Vorgang
    pub fn start_sync(state: &AppStateService) {
        let mut state_ref = state.borrow_mut();
        state_ref.is_syncing = true;
        
        // In einer echten App würde hier die Tauri-API aufgerufen
        log::info!("Starting sync operation...");
    }
    
    /// Beendet einen Sync-Vorgang
    pub fn complete_sync(state: &AppStateService) {
        let mut state_ref = state.borrow_mut();
        state_ref.is_syncing = false;
        state_ref.last_sync = Some("2 days ago".to_string());
        
        log::info!("Sync operation completed");
    }
}
