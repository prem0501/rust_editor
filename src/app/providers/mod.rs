pub mod file_editor_provider;
pub mod root_context_provider;

pub struct ActionDef<T, S> {
    pub message: S,
    pub payload: T,
}
