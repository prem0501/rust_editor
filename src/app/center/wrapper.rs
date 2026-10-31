use yew::prelude::*;

use crate::app::components::{editor::monaco::MonacoWrapper, status_bar::StatusBar};

#[function_component]
pub fn Center() -> Html {
    html! {
        <main>
          <MonacoWrapper/>
          <StatusBar />
        </main>
    }
}
