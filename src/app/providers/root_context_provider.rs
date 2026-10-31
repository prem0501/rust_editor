use std::{rc::Rc, sync::LazyLock};

use yew::prelude::*;

static LANGUAGES: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "plain".to_string(),
        "json".to_string(),
        "javascript".to_string(),
    ]
});

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AppState {
    pub current_lang: String,
    pub languages: Vec<String>,
}

pub struct ActionDef {
    pub message: Actions,
    pub payload: String,
}

pub enum Actions {
    SetLang,
}

impl Reducible for AppState {
    type Action = ActionDef;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.message {
            Actions::SetLang => Rc::new(AppState {
                current_lang: action.payload,
                languages: self.languages.clone(),
            }),
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

#[derive(Debug, Properties, PartialEq)]
pub struct ProviderProp {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn AppContextProvider(props: &ProviderProp) -> Html {
    let state = use_reducer(|| AppState {
        current_lang: LANGUAGES
            .get(0)
            .unwrap_or(&String::from("plain").to_string())
            .to_string(),
        languages: LANGUAGES.to_vec(),
    });
    html! {
      <ContextProvider<AppStateContext> context={state}>
        {props.children.clone()}
      </ContextProvider<AppStateContext>>
    }
}
