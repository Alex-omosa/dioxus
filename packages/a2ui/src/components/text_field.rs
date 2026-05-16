//! A2UI TextField component — two-way data-bound input.

use dioxus::prelude::*;
use serde_json::Value;

use crate::component::ComponentModel;
use crate::data_model::DataModel;
use crate::hooks::use_two_way_binding;

#[component]
pub fn A2uiTextField(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let label_val = model.get_prop("label").cloned().unwrap_or(Value::Null);
    let value_prop = model.get_prop("value").cloned().unwrap_or(Value::Null);
    let variant = model.get_str_prop("variant").to_string();

    let label = crate::hooks::use_dynamic_string(label_val, data_model, base_path.clone());
    let (read_value, binding_path) = use_two_way_binding(&value_prop, data_model, &base_path);

    let is_obscured = variant == "obscured";
    let is_long_text = variant == "longText";
    let input_type = if is_obscured { "password" } else { "text" };

    // Store the binding path for the write-back closure
    let mut dm = data_model;

    rsx! {
        div { class: "a2ui-textfield",
            label { class: "a2ui-textfield-label", "{label}" }

            if is_long_text {
                textarea {
                    class: "a2ui-textfield-input a2ui-textfield-textarea",
                    value: "{read_value}",
                    oninput: {
                        let binding_path = binding_path.clone();
                        move |e: Event<FormData>| {
                            if let Some(ref path) = binding_path {
                                dm.set(path, Value::String(e.value()));
                            }
                        }
                    },
                }
            } else {
                input {
                    class: "a2ui-textfield-input",
                    r#type: "{input_type}",
                    value: "{read_value}",
                    oninput: {
                        let binding_path = binding_path.clone();
                        move |e: Event<FormData>| {
                            if let Some(ref path) = binding_path {
                                dm.set(path, Value::String(e.value()));
                            }
                        }
                    },
                }
            }
        }
    }
}
