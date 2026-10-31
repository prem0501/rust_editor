use wasm_bindgen::JsCast;
use weblog::console_log;
use yew::prelude::*;

use crate::app::providers::root_context_provider::{ActionDef, Actions, AppStateContext};

#[function_component]
pub fn LangSelector() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();
    let languages = app_state.languages.clone();
    let current_lang = app_state.current_lang.clone();
    console_log!("LangSelector {}", current_lang.clone());
    let on_lang_change: Callback<Event> = {
        let langs = languages.clone();
        Callback::from(move |e: Event| {
            let target: Option<web_sys::EventTarget> = e.target();
            let input: Option<web_sys::HtmlSelectElement> =
                target.and_then(|t: web_sys::EventTarget| {
                    t.dyn_into::<web_sys::HtmlSelectElement>().ok()
                });
            if let Some(value) = input {
                let v: Option<&String> = langs.iter().find(|lang| value.value() == **lang);
                console_log!(v);
                match v {
                    Some(lang) => {
                        app_state.dispatch(ActionDef {
                            message: Actions::SetLang,
                            payload: lang.to_string(),
                        });
                    }
                    None => console_log!("Language not matched"),
                }
            };
        })
    };
    let lang_options = languages
        .clone()
        .iter()
        .map(|lang: &String| {
            html! {
                <option value={lang.clone()} selected={*lang == current_lang}>{lang}</option>
            }
        })
        .collect::<Html>();
    html! {
      <select onchange={on_lang_change} value={current_lang} name="lang_change">
        { lang_options }
      </select>
    }
}
