use serde::{Deserialize, Serialize};

/// Repräsentiert einen Audio-Track mit allen Metadaten
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub bpm: u32,
    pub duration: String, // Format: "MM:SS"
    pub file_path: String,
    pub file_size: u64,
    pub status: TrackStatus,
    pub partitions: Vec<PartitionColor>,
}

/// Status eines Tracks (grün, gelb, rot wie im Screenshot)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackStatus {
    Synced,    // Grün ✓
    Warning,   // Gelb ⚠
    Error,     // Rot ✗
    Pending,   // Grau
}

/// Farben für Partition-Zuordnung
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PartitionColor {
    Blue,
    Green,
    Purple,
    Orange,
    Red,
}

/// Laufwerk/Partition Information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    pub id: String,
    pub name: String,
    pub drive_type: DriveType,
    pub total_space: u64,
    pub free_space: u64,
    pub mount_path: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DriveType {
    Main,
    Backup,
    External,
}


/// Playlist-Struktur
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<String>, // Track IDs
    pub created_at: String,
    pub updated_at: String,
}

/// Sync-Profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncProfile {
    pub id: String,
    pub name: String,
    pub source_drive: String,
    pub target_drives: Vec<String>,
    pub sync_rules: SyncRules,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncRules {
    pub include_genres: Vec<String>,
    pub exclude_genres: Vec<String>,
    pub min_bpm: Option<u32>,
    pub max_bpm: Option<u32>,
    pub auto_sync: bool,
}
