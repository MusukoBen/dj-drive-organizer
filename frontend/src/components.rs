use yew::prelude::*;
use crate::services::AppStateProvider;
use crate::pages::MainPage;

/// Haupt-App-Komponente
#[function_component]
pub fn App() -> Html {
    html! {
        <AppStateProvider>
            <MainPage />
        </AppStateProvider>
    }
}
