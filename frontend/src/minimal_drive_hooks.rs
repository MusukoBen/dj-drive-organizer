use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::tauri_api::{FileMeta, ScanProgress, invoke_scan_drive};

#[derive(Debug, Clone, PartialEq)]
pub struct DriveState {
    pub scan_results: Vec<FileMeta>,
    pub scan_progress: Option<ScanProgress>,
    pub is_scanning: bool,
    pub error_message: Option<String>,
}

impl Default for DriveState {
    fn default() -> Self {
        Self {
            scan_results: Vec::new(),
            scan_progress: None,
            is_scanning: false,
            error_message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DriveStateAction {
    StartScan,
    UpdateProgress(ScanProgress),
    CompleteScan(Vec<FileMeta>),
    ScanError(String),
}

fn drive_state_reducer(state: &DriveState, action: DriveStateAction) -> DriveState {
    match action {
        DriveStateAction::StartScan => DriveState {
            is_scanning: true,
            ..state.clone()
        },
        DriveStateAction::UpdateProgress(progress) => DriveState {
            scan_progress: Some(progress),
            ..state.clone()
        },
        DriveStateAction::CompleteScan(files) => DriveState {
            scan_results: files,
            is_scanning: false,
            scan_progress: None,
            ..state.clone()
        },
        DriveStateAction::ScanError(error) => DriveState {
            error_message: Some(error),
            is_scanning: false,
            ..state.clone()
        },
    }
}

#[hook]
pub fn use_drive_state() -> (DriveState, Callback<DriveStateAction>) {
    let state = use_state(DriveState::default);
    let dispatch = {
        let state_clone = state.clone();
        Callback::from(move |action: DriveStateAction| {
            state_clone.set(drive_state_reducer(&state_clone, action));
        })
    };
    ((*state).clone(), dispatch)
}

#[hook]
pub fn use_drive_scan() -> (DriveState, Callback<String>) {
    let (state, dispatch) = use_drive_state();
    let start_scan = {
        let dispatch_clone = dispatch.clone();
        Callback::from(move |path: String| {
            dispatch_clone.emit(DriveStateAction::StartScan);
            let dispatch_for_async = dispatch_clone.clone();
            spawn_local(async move {
                match invoke_scan_drive(path).await {
                    Ok(files) => {
                        dispatch_for_async.emit(DriveStateAction::CompleteScan(files));
                    }
                    Err(e) => {
                        dispatch_for_async.emit(DriveStateAction::ScanError(format!("Error: {}", e)));
                    }
                }
            });
        })
    };
    (state, start_scan)
}
