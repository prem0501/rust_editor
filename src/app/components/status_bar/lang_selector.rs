use crate::app::components::ui::select::Select;
use wasm_bindgen::JsCast;
use weblog::console_log;
use yew::prelude::*;

use crate::app::providers::{
    ActionDef,
    root_context_provider::{Actions, AppStateContext},
};

#[function_component]
pub fn LangSelector() -> Html {
    let app_state = use_context::<AppStateContext>().unwrap();
    let languages = app_state.languages.clone();
    let current_lang = app_state.current_lang.clone();
    console_log!("LangSelector {}", current_lang.clone());
    let onchange: Callback<String> = {
        let langs = languages.clone();
        Callback::from(move |value: String| {
            let v: Option<&String> = langs.iter().find(|lang| value == **lang);
            console_log!(v);
            match v {
                Some(lang) => {
                    app_state.dispatch(ActionDef {
                        message: Actions::SetLang,
                        payload: lang.to_string(),
                    });
                }
                None => console_log!("Language not matched"),
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
      <Select {onchange} value={current_lang} name="lang_change" color="gray-200">
        { lang_options }
      </Select>
    }
}
