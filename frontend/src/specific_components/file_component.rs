use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, MouseEvent};
use yew::prelude::*;
use yew_hooks::use_toggle;
use yew_router::prelude::{use_navigator, use_route};
use yewdux::prelude::*;

use shared::{
    id::Id,
    log,
    schema::{FileDirectory, FileNodeDelete},
};

use crate::{backend, components::PopOverMenu, router::Route};

#[derive(PartialEq, Properties)]
pub struct FileComponentProps {
    pub onclick: Callback<MouseEvent>,
    pub onclickfile: Callback<MouseEvent>,
    pub name: String,
    pub class: String,
    pub id: Id,
}

#[function_component(FileComponent)]
pub fn file_component(props: &FileComponentProps) -> Html {
    // HasMap
    // {
    // type:"dropover", // or dropunder or dropbellow,
    // dragged_id: uuid,
    // target_id: uuid
    // }
    //let drop_data = use_state(|| "".to_string());
    //let is_drag_over = use_state(|| "".to_string());
    let dispatch_file_directory = Dispatch::<FileDirectory>::new();
    let is_drag_under = use_state(|| "".to_string());
    let position: UseStateHandle<Option<MouseEvent>> = use_state(|| None);
    let is_dragged = use_state(|| "".to_string());
    let is_enter = use_state(|| "".to_string());
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>().unwrap_or_default();

    let caret = use_toggle("", "caret-down");
    let id = props.id.clone();

    let toggle_caret = {
        let caret = caret.clone();
        move |_e: MouseEvent| {
            caret.toggle();
        }
    };

    let ondragstart: Callback<DragEvent> = {
        let is_dragged = is_dragged.clone();
        let id = id.clone();

        Callback::from(move |e: DragEvent| {
            e.data_transfer()
                .unwrap()
                .set_data("dragged_item", &id.to_string())
                .unwrap();
            is_dragged.set("dragged".to_string())
        })
    };

    let ondragend: Callback<DragEvent> = {
        let is_dragged = is_dragged.clone();
        let is_enter = is_enter.clone();
        Callback::from(move |_e: DragEvent| {
            is_dragged.set("".to_string());
            is_enter.set("".to_string());
        })
    };

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_enter.set("dragging_over".to_string());
        }
    });

    let _is_enter = is_enter.clone();
    let ondragleave: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_enter.set("".to_string());
    });

    let _is_enter = is_enter.clone();
    let _is_dragged = is_dragged.clone();
    let _is_drag_under = is_drag_under.clone();
    let ondrop: Callback<DragEvent> = {
        let id = id.clone();
        let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            let curr: Element = e.target_unchecked_into();
            let _ = curr.class_list().toggle("dragging_over");
            let dragged = e.data_transfer().unwrap().get_data("dragged_item").unwrap();
            let id = id.clone();
            let mut old_parent_id: Id = Uuid::new_v4().into();
            let dragged_uuid = Uuid::parse_str(dragged.as_str()).map(Id::from).unwrap();
            for (i, value) in &_dispatch_file_directory.get().files.adjacency {
                if value.contains(&dragged_uuid) {
                    old_parent_id = *i;
                    break;
                }
            }
            crate::backend::change_directory(id.to_string(), dragged, old_parent_id.to_string());
        })
    };

    let _is_drag_under = is_drag_under.clone();
    let _is_dragged = is_dragged.clone();
    let ondragenter_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        if (*_is_dragged).len() == 0 {
            _is_drag_under.set("height: 20px; opacity:1;".to_string());
        }
    });

    let ondragover: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _e.prevent_default();
    });

    let _is_drag_under = is_drag_under.clone();
    let _id = id.clone();
    let ondrop_under: Callback<DragEvent> = Callback::from(move |e: DragEvent| {
        e.prevent_default();
        let curr: Element = e.target_unchecked_into();
        let _ = curr.set_attribute("style", "height: 3px; opacity:0;");

        let _dragged = e.data_transfer().unwrap().get_data("dragged_item");
    });

    let _id = id.clone();
    // log!(id.to_string());
    // let rename_file: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
    //     spawn_local(async move {
    //         let id = serde_json::json!(_id).to_string();
    //         log!(&id);
    //         backend::rename_file(id, "new_name".to_string()).await;
    //     });
    // });

    let rename_file: Callback<MouseEvent> =
        dispatch_file_directory.reduce_mut_future_callback(move |state| {
            Box::pin(async move {
                let clone_id = _id.clone();
                let id = serde_json::json!(_id).to_string();
                log!(&clone_id);
                let x = backend::rename_file(id, "new_name".to_string()).await;
                if x.is_ok() {
                    log!("success".to_string());
                    state
                        .files
                        .vertices
                        .get_mut(&clone_id)
                        .unwrap()
                        .name = "new_name".to_string();
                }
            })
        });

    let ondelete = {
        let id = id.clone();
        let _dispatch_file_directory = Dispatch::<FileDirectory>::new();
        let mut parent_id = Id::default();
        for (parent, child_id) in &_dispatch_file_directory.get().files.adjacency {
            if child_id.contains(&id) {
                parent_id = *parent;
            }
        }
        let delete_file_node = FileNodeDelete {
            id,
            parent_id,
            tree_id: _dispatch_file_directory.get().id,
        };
        let file_id = id.clone();
        _dispatch_file_directory.reduce_mut_future_callback(move |state| {
            match route {
                // the current file is in use navigate to home!
                Route::File { id } => {
                    if file_id == id {
                        navigator.push(&Route::Home);
                    }
                }
                _ => {}
            }
            Box::pin(async move {
                let result = crate::backend::delete_file(delete_file_node).await;
                log!(result);
                state.files.remove(&file_id);
            })
        })
    };

    let _is_drag_under = is_drag_under.clone();
    let ondragleave_under: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
        _is_drag_under.set("".to_string());
    });
    let oncontextmenu = {
        let position = position.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            position.set(Some(e));
        })
    };

    // let ondragenter_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("height: 20px; opacity:1;".to_string());
    // });

    // let ondragleave_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _is_drag_above.set("".to_string());
    // });

    // let ondrop_above: Callback<DragEvent> = Callback::from(move |_e: DragEvent| {
    //     _e.prevent_default();
    //     _is_drag_above.set("".to_string());
    // });

    html! {
        <div>
        // TODO
        //  {if is_first_file {
        //         html!{
        //         <div
        //            ondrop={ondrop_above}
        //            ondragenter={ondragenter_above}
        //            ondragleave={ondragleave_above}
        //            class="drag_under"/>
        //         }
        //  }}

        <div {oncontextmenu} style="position: relative; width:100%; display: block;">
           if props.class.contains("caret"){
               <button class={format!("{} crate_button",(*caret))}
               onmouseup={toggle_caret}
               onclick = { props.onclick.clone() }><i class="fa-solid fa-caret-right"></i></button>
           }
           <li
           ondragover={ondragover.clone()}
           {ondrop}
           {ondragenter}
           {ondragleave}
           {ondragstart}
           {ondragend}
           id = { id.to_string()}
           onclick={props.onclickfile.clone()}
           draggable="true"
           class={format!("right_clickable file_component hovering active {} {} {}",(*is_dragged).clone(),(*is_enter).clone(), "")}
           style="margin-left: 30px; min-width: 0px; align-items: center; height: 100%; display: block;"
          >
           {props.name.clone()}
           </li>
           <i style="height:100%" class="btn create_file fa-solid fa-plus"></i>
        </div>

            // <div
            // ondragover={ondragover.clone()}
            // style={format!("{}",(*is_drag_under).clone())}
            // ondrop={ondrop_under}
            // ondragenter={ondragenter_under}
            // ondragleave={ondragleave_under}
            // class="drag_under"/>

           <PopOverMenu
            click_on={Some(true)}
           position = {position.clone()}
           items={vec![
           html! {<a><input
                onclick={rename_file}
                // TODO fix this later not matter now
                autofocus=true placeholder="rename.."/></a>},
           html! {<a><i class="fa-solid fa-upload"/>{"Share"}</a>},
           html! {<a><i class="fa-solid fa-eye"/>{"Permissions"}</a>},
           html! {<a onclick = {ondelete}><i class="fa-solid fa-trash"/>{"Delete"}</a>},
           html! {<a><i class="fa-brands fa-medium"></i>{"Category"}</a>},
           ]}
          />

        </div>

    }
}
