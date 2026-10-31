use yew::prelude::*;

use crate::app::{center::wrapper::Center, providers::root_context_provider::AppContextProvider};

mod center;
mod components;
mod providers;

#[component]
pub fn App() -> Html {
    html! {
        <AppContextProvider>
            <Center />
        </AppContextProvider>
    }
}
