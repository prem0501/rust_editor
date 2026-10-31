use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}
#[component(UseState)]
pub fn client_only(props: &Props) -> Html {
    let csr = use_state(|| false);
    html! {
        if *csr == true {
            <>{
                props.children.clone()
            }</>
        } else {
            <div>{"prem"}</div>
        }
    }
}
