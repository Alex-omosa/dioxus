//! Fallback component for unknown/unimplemented A2UI component types.

use dioxus::prelude::*;

use crate::component::ComponentModel;

#[component]
pub fn A2uiFallback(model: ComponentModel) -> Element {
    rsx! {
        div {
            class: "a2ui-fallback",
            style: "padding: 4px 8px; border: 1px dashed #666; border-radius: 4px; font-size: 12px; color: #999;",
            "Unknown: {model.component_type} (id: {model.id})"
        }
    }
}
