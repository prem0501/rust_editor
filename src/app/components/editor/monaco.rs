use crate::app::providers::{
    ActionDef,
    file_editor_provider::{Actions, EditorFileDetails, FileEditorStateContext},
    root_context_provider::AppStateContext,
};
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::{
        KeyCode, KeyMod,
        editor::{BuiltinTheme, IStandaloneCodeEditor},
    },
    yew::{CodeEditor, CodeEditorLink},
};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

fn get_options(lang: &String) -> CodeEditorOptions {
    CodeEditorOptions::default()
        .with_language(lang.to_owned())
        // .with_value(val.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .with_automatic_layout(true)
}
#[component]
pub fn MonacoWrapper() -> Html {
    let root_context = use_context::<AppStateContext>().unwrap();
    let editor_file_context = use_context::<FileEditorStateContext>().unwrap();
    let current_lang = &root_context.current_lang;
    let current_file_value = editor_file_context
        .current_file
        .clone()
        // .as_ref()
        .unwrap_or_else(|| EditorFileDetails::new());
    let content = &current_file_value.content;

    let text_model = use_state_eq(|| TextModel::create(content, Some(current_lang), None).unwrap());

    let on_editor_created = {
        let text_model = text_model.clone();

        let js_closure = {
            let text_model = text_model.clone();
            Closure::<dyn Fn()>::new(move || {
                let mut file = current_file_value.clone();
                file.content = text_model.get_value().clone();
                editor_file_context.dispatch(ActionDef {
                    message: Actions::SaveFileContent,
                    payload: Some(file),
                });
            })
        };
        use_callback(
            text_model,
            move |editor_link: CodeEditorLink, _text_model| {
                editor_link.with_editor(|editor| {
                    let keycode = KeyCode::KeyS.to_value() | (KeyMod::ctrl_cmd() as u32);
                    let raw_editor: &IStandaloneCodeEditor = editor.as_ref();
                    raw_editor.add_command(
                        keycode.into(),
                        js_closure.as_ref().unchecked_ref(),
                        None,
                    );
                });
            },
        )
    };

    html! {
      <div class="code-wrapper">
            <div class="code-editor">
                <CustomEditor {on_editor_created} text_model={(*text_model).clone()} lang={(*current_lang).to_owned()} />
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CustomEditorProps {
    on_editor_created: Callback<CodeEditorLink>,
    text_model: TextModel,
    lang: String,
}
#[component]
pub fn CustomEditor(props: &CustomEditorProps) -> Html {
    let CustomEditorProps {
        on_editor_created,
        text_model,
        lang,
    } = props;

    html! {
        <CodeEditor classes={"full-height"} options={ get_options(lang).to_sys_options() } {on_editor_created} model={text_model.clone()} />
    }
}
