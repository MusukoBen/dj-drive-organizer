use yew::prelude::*;
use crate::minimal_drive_hooks::use_drive_scan;

#[function_component]
pub fn SimpleDriveDemo() -> Html {
    let (state, start_scan) = use_drive_scan();
    
    let on_scan = {
        let start_scan = start_scan.clone();
        Callback::from(move |_| {
            start_scan.emit("/path/to/test".to_string());
        })
    };
    
    html! {
        <div>
            <h1>{"Drive State Hook Demo"}</h1>
            <button onclick={on_scan}>{"Start Scan"}</button>
            <p>{format!("Is scanning: {}", state.is_scanning)}</p>
            <p>{format!("Files found: {}", state.scan_results.len())}</p>
            if let Some(error) = state.error_message {
                <p style="color: red;">{format!("Error: {}", error)}</p>
            }
        </div>
    }
}
