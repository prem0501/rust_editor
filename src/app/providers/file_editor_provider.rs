use std::rc::Rc;
use yew::prelude::*;

use crate::app::providers::ActionDef;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FileEditorState {
    pub current_file: Option<EditorFileDetails>,
    pub all_files: Vec<Option<EditorFileDetails>>,
    pub all_opened_files: Vec<Option<EditorFileDetails>>,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EditorFileDetails {
    pub name: String,
    pub content: String,
    pub file_uri: Option<String>,
    pub state: FileState,
}

impl EditorFileDetails {
    pub fn new() -> EditorFileDetails {
        EditorFileDetails {
            name: String::from("Untitled"),
            content: String::from(""),
            file_uri: None,
            state: FileState::EMPTY,
        }
    }
    pub fn save_content(&mut self, content: String) {
        self.content = content.clone();
        self.state = FileState::SAVED;
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FileState {
    EMPTY,
    OPENED,
    EDITING,
    SAVED,
    CLOSED,
}
pub enum Actions {
    SetCurrentFile,
    SaveFileContent,
}

impl Reducible for FileEditorState {
    type Action = ActionDef<Option<EditorFileDetails>, Actions>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.message {
            Actions::SetCurrentFile => set_current_file(&self, action.payload),
            Actions::SaveFileContent => save_current_file(&self, action.payload),
        }
    }
}

pub fn set_current_file(
    state: &Rc<FileEditorState>,
    payload: Option<EditorFileDetails>,
) -> Rc<FileEditorState> {
    Rc::new(FileEditorState {
        current_file: payload,
        all_files: state.all_files.clone(),
        all_opened_files: state.all_opened_files.clone(),
    })
}

pub fn save_current_file(
    state: &Rc<FileEditorState>,
    payload: Option<EditorFileDetails>,
) -> Rc<FileEditorState> {
    if let Some(file) = &state.current_file {
        if let Some(pay) = payload {
            let mut f = file.clone();
            f.save_content(pay.content);
            return Rc::new(FileEditorState {
                current_file: Some(f),
                all_files: state.all_files.clone(),
                all_opened_files: state.all_opened_files.clone(),
            });
        }
    }
    state.clone()
}

pub type FileEditorStateContext = UseReducerHandle<FileEditorState>;

#[derive(Debug, Properties, PartialEq)]
pub struct FileEditorStateProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn FileEditorStateProvider(props: &FileEditorStateProps) -> Html {
    let state = use_reducer(|| FileEditorState {
        current_file: Some(EditorFileDetails {
            content: String::from("prem"),
            name: String::from("Untitled"),
            file_uri: None,
            state: FileState::EMPTY,
        }),
        all_files: vec![],
        all_opened_files: vec![],
    });
    html! {
      <ContextProvider<FileEditorStateContext> context={state}>
        { props.children.clone() }
      </ContextProvider<FileEditorStateContext>>
    }
}
