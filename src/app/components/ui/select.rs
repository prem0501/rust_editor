use gloo::events::{self, EventListener};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Style {
    BORDERED,
    GHOST,
    TEXT,
}
#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or(Style::TEXT)]
    pub style: Style,
    #[prop_or(String::from("black"))]
    pub color: String,
    pub onchange: Callback<String>,
    pub value: AttrValue,
}
#[function_component]
pub fn Select(props: &Props) -> Html {
    let onchange = {
        let emitter = props.onchange.clone();
        Callback::from(move |e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            emitter.emit(target.value().clone());
        })
    };

    let class_to_apply = match props.style {
        Style::BORDERED => {
            String::from("border-2 border-black bg-white shadow-[4px_4px_0_0] shadow-black")
        }
        Style::GHOST => String::from("border-r-4 bg-gray-50 dark:bg-gray-900"),
        Style::TEXT => String::from("bg-transparent white"),
    };
    let classes_default = vec![
        "mt-0.5",
        "w-full",
        "placeholder-black",
        "focus:ring-2",
        "focus:ring-yellow-300",
        "sm:text-sm",
        "capitalize",
    ];

    html! {
      <label for={ props.name.clone() } class={format!("text-{}", props.color.clone())}>
        <span class="text-sm font-semibold"> { props.label.clone() } </span>

        <select
          name={ props.name.clone() }
          id={ props.name.clone() }
          {onchange}
          class={classes!(classes_default, class_to_apply.clone())}
        >
          { props.children.clone() }
        </select>
      </label>
    }
}
