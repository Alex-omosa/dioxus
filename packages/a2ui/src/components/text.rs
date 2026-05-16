//! A2UI Text component.

use dioxus::prelude::*;
use serde_json::Value;

use crate::component::ComponentModel;
use crate::data_model::DataModel;
use crate::hooks::use_dynamic_string;

#[component]
pub fn A2uiText(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let text_val = model.get_prop("text").cloned().unwrap_or(Value::Null);
    let text = use_dynamic_string(text_val, data_model, base_path);
    let variant = model.get_str_prop("variant");

    match variant {
        "h1" => rsx! { h1 { class: "a2ui-text a2ui-h1", "{text}" } },
        "h2" => rsx! { h2 { class: "a2ui-text a2ui-h2", "{text}" } },
        "h3" => rsx! { h3 { class: "a2ui-text a2ui-h3", "{text}" } },
        "h4" => rsx! { h4 { class: "a2ui-text a2ui-h4", "{text}" } },
        "h5" => rsx! { h5 { class: "a2ui-text a2ui-h5", "{text}" } },
        "caption" => rsx! { span { class: "a2ui-text a2ui-caption", "{text}" } },
        _ => rsx! { p { class: "a2ui-text a2ui-body", "{text}" } },
    }
}
