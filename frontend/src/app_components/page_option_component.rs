use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::Closure;
use web_sys::{DragEvent, Element, MouseEvent, window};
use web_sys::console::log_1;
use yew::{html, Html};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::{Menu};
use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct PageOptionsProps {
    // pub id: u64,
}


#[function_component(PageOptions)]
pub fn page_options(props: &PageOptionsProps) -> Html {
    let position: UseStateHandle<String> = use_state(|| "".to_string());
    let _position = position.clone();
    let onmouseup: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        _position.set(
            format!("top:{}px; right:{}px;", _e.offset_y(), _e.offset_x()).into()
        );
    });
    
    let items: Vec<Html> = vec![
        html! {<><i class="fa-solid fa-comment"></i>{"Comments"}</>},
        html! {<><i class="fa-solid fa-rectangle-history"></i>{"History"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"Share"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"Export google docs"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"Export dox"}</>},
        html! {<><i class="fa-solid fa-file-pdf"></i>{"Export pdf"}</>},
        html! {<><i class="fa-solid fa-right-from-bracket"></i>{"Delete"}</>},
    ];

    html! {
    <>
        <span {onmouseup} class="btn">
            <i class="fa-solid fa-ellipsis-vertical"></i>
        </span>


        <Menu
        position={position.clone()}
        {items}
      />
    </>
    }
}
