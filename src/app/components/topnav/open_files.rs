use yew::prelude::*;

use crate::app::providers::file_editor_provider::FileEditorStateContext;

#[component]
pub fn OpenedFiles() -> Html {
    let context = use_context::<FileEditorStateContext>().unwrap();
    html! {
      <ul>
          {
            context.all_opened_files.clone().iter().map(|f| {
              if let Some(file) = f {
                html! {
                  <li>
                    <button>{file.name.clone()}</button>
                  </li>
                }
              } else {
                html! {
                  <></>
                }
              }
            }).collect::<Html>()
          }
      </ul>
    }
}
