use crate::app::providers::root_context_provider::AppStateContext;
use monaco::{api::CodeEditorOptions, sys::editor::BuiltinTheme, yew::CodeEditor};
use yew::prelude::*;

fn get_options(lang: &String) -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language(lang.to_owned())
        .with_value("".to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
}
#[component]
pub fn MonacoWrapper() -> Html {
    let ctx = use_context::<AppStateContext>().unwrap();
    let current_lang = &ctx.current_lang;
    let options = get_options(&current_lang).to_sys_options();
    html! {
      <CodeEditor classes="monaco-wrapper" options={options}/>
    }
}
