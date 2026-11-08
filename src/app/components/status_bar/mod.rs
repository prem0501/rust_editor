use yew::{Html, component, html};

use crate::app::components::status_bar::lang_selector::LangSelector;
pub mod lang_selector;

#[component]
pub fn StatusBar() -> Html {
    html! {
      <div class="navbar-wrapper">
        <LangSelector />
      </div>
    }
}
