use yew::prelude::*;

use crate::app::components::topnav::open_files::OpenedFiles;

#[component]
pub fn Topbar() -> Html {
    html! {
      <nav class="navbar-wrapper">
        <OpenedFiles/>
        <input placeholder="Search files" />
      </nav>
    }
}
