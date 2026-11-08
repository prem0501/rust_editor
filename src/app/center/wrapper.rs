use yew::prelude::*;

use crate::app::components::{
    editor::monaco::MonacoWrapper, status_bar::StatusBar, topnav::topbar::Topbar,
};

#[function_component]
pub fn Center() -> Html {
    html! {
        <main class="center-wrapper">
          <Topbar/>
          <MonacoWrapper/>
          <StatusBar />
        </main>
    }
}
