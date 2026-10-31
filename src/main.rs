mod app;
use app::App;
// mod test;
// use test::Test;

// #[tokio::main(flavor = "current_thread")]
fn main() {
    yew::Renderer::<App>::new().render();
}
