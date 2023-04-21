use std::iter::once;

use yew::prelude::*;

use crate::data::{Equipment, Exercise, EXERCISES};
use crate::select::Select;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub exercises: Vec<Exercise>,
    pub on_add: Callback<String>,
}

#[function_component(Set)]
pub fn set(props: &Props) -> Html {
    html! {
        <div class="set">
            <div>
                { format!("Set #{}", props.index + 1) }
            </div>
            <ul>
                { props.exercises.iter().map(|e| html!(<li>{ e.name }</li>)).collect::<Html>() }
            </ul>
            <div>
                <Select
                    options={EXERCISES.iter().map(|(&i, e)| (i, e.name)).collect::<Vec<_>>()}
                    on_change={props.on_add.clone()} />
            </div>
        </div>
    }
}
