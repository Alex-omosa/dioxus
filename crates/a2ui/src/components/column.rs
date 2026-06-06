//! A2UI Column component — vertical flex container.

use dioxus::prelude::*;
use serde_json::Value;

use crate::component::ComponentModel;
use crate::data_model::DataModel;
use crate::renderer::A2uiComponent;

#[component]
pub fn A2uiColumn(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let justify = match model.get_str_prop("justify") {
        "center" => "center",
        "end" => "flex-end",
        "spaceBetween" => "space-between",
        "spaceAround" => "space-around",
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

    let children_prop = model.get_prop("children").cloned().unwrap_or(Value::Null);

    match &children_prop {
        // Static array of child IDs
        Value::Array(arr) => {
            let ids: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();

            rsx! {
                div {
                    class: "a2ui-column",
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "{justify}",
                    align_items: "{align}",
                    gap: "8px",

                    for child_id in ids {
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
        // Template: {"componentId": "...", "path": "/..."}
        Value::Object(tmpl) => {
            let template_id = tmpl
                .get("componentId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let data_path = tmpl
                .get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Read the array from the data model
            let item_count = {
                let abs_path = crate::pointer::make_absolute(&data_path, &base_path);
                data_model
                    .get(&abs_path)
                    .and_then(|v| v.as_array().map(|a| a.len()))
                    .unwrap_or(0)
            };

            rsx! {
                div {
                    class: "a2ui-column",
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "{justify}",
                    align_items: "{align}",
                    gap: "8px",

                    for i in 0..item_count {
                        A2uiComponent {
                            key: "{template_id}-{i}",
                            component_id: template_id.clone(),
                            data_model: data_model,
                            base_path: format!("{}/{i}", crate::pointer::make_absolute(&data_path, &base_path)),
                        }
                    }
                }
            }
        }
        _ => rsx! { div { class: "a2ui-column" } },
    }
}
