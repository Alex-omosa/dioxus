//! A2UI Button component.

use dioxus::prelude::*;

use crate::component::ComponentModel;
use crate::data_model::DataModel;
use crate::renderer::A2uiComponent;
use crate::surface::SurfaceModel;

#[component]
pub fn A2uiButton(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let variant = model.get_str_prop("variant").to_string();
    let child_id = model.get_str_prop("child").to_string();
    let action = model.get_prop("action").cloned();

    let surface_ctx = use_context::<Signal<SurfaceModel>>();

    let btn_class = match variant.as_str() {
        "primary" => "a2ui-button a2ui-button-primary",
        "borderless" => "a2ui-button a2ui-button-borderless",
        _ => "a2ui-button",
    };

    rsx! {
        button {
            class: "{btn_class}",
            onclick: move |_| {
                if let Some(ref action_val) = action {
                    if let Some(event) = action_val.get("event") {
                        let _name = event.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let _surface = surface_ctx.read();
                        // TODO: dispatch via event channel / transport layer
                    }
                }
            },

            if !child_id.is_empty() {
                A2uiComponent {
                    component_id: child_id.clone(),
                    data_model: data_model,
                    base_path: base_path.clone(),
                }
            }
        }
    }
}
