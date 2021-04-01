use super::state::TODOS;
use crate::recoil::use_atom_family;
use dioxus_core::prelude::*;
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct TodoEntryProps {
    id: Uuid,
}

pub fn TodoEntry(ctx: Context, props: &TodoEntryProps) -> DomTree {
    let (is_editing, set_is_editing) = use_state(&ctx, || false);
    let todo = use_atom_family(&ctx, &TODOS, props.id);

    ctx.render(rsx! (
        li {
            "{todo.id}"
            input {
                class: "toggle"
                type: "checkbox"
                "{todo.checked}"
            }
            {is_editing.then(|| {
                rsx!(input {
                    value: "{contents}"
                })
            })}
        }
    ))
}

pub fn Example(ctx: Context, id: Uuid, name: String) -> DomTree {
    ctx.render(rsx! {
        div {}
    })
}
