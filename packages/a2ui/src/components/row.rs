//! A2UI Row component — horizontal flex container.

use dioxus::prelude::*;

use crate::component::ComponentModel;
use crate::data_model::DataModel;
use crate::renderer::A2uiComponent;

#[component]
pub fn A2uiRow(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let justify = match model.get_str_prop("justify") {
        "center" => "center",
        "end" => "flex-end",
        "spaceAround" => "space-around",
        "spaceBetween" => "space-between",
        "spaceEvenly" => "space-evenly",
        "stretch" => "stretch",
        _ => "flex-start",
    };

    let align = match model.get_str_prop("align") {
        "center" => "center",
        "end" => "flex-end",
        "stretch" => "stretch",
        _ => "flex-start",
    };

    let children_ids = get_static_children(&model);

    rsx! {
        div {
            class: "a2ui-row",
            display: "flex",
            flex_direction: "row",
            justify_content: "{justify}",
            align_items: "{align}",
            gap: "8px",

            for child_id in children_ids {
                A2uiComponent {
                    key: "{child_id}",
                    component_id: child_id.clone(),
                    data_model: data_model,
                    base_path: base_path.clone(),
                }
            }
        }
    }
}

/// Extract static child IDs from a component's `children` property.
pub fn get_static_children(model: &ComponentModel) -> Vec<String> {
    model
        .get_prop("children")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}
