use std::iter::once;

use yew::prelude::*;

use crate::data::{Exercise, EXERCISES};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub options: Vec<(&'static str, &'static str)>,
    pub on_change: Callback<String>,
}

use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let on_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            on_change.emit(input.value());
            input.set_selected_index(0);
        })
    };

    html! {
        <>
            <select onchange={on_change}>
                {
                    once(html!(<option disabled={true} selected={true}>{ "--" }</option>))
                        .chain(props.options
                            .iter()
                            .map(|o| html! {
                                <option value={EXERCISES[o.0].id.to_string()}>{ o.1 }</option>
                            }))
                        .collect::<Html>()
                }
            </select>
        </>
    }
}
