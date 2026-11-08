use yew::prelude::*;

use crate::app::{
    center::wrapper::Center,
    components::sidebar::sidebar::Sidebar,
    providers::{
        file_editor_provider::FileEditorStateProvider, root_context_provider::AppContextProvider,
    },
};

mod center;
mod components;
mod providers;

#[component]
pub fn App() -> Html {
    html! {
        <AppContextProvider>
            <FileEditorStateProvider>
                <div class="app-wrapper">
                    <Sidebar />
                    <Center />
                </div>
            </FileEditorStateProvider>
        </AppContextProvider>
    }
}
