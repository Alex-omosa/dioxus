//! A2UI Button component.

use dioxus::prelude::*;

use crate::component::ComponentModel;
use crate::context::A2uiActionContext;
use crate::data_model::DataModel;
use crate::renderer::A2uiComponent;
use crate::surface::SurfaceModel;

#[component]
pub fn A2uiButton(model: ComponentModel, data_model: DataModel, base_path: String) -> Element {
    let variant = model.get_str_prop("variant").to_string();
    let child_id = model.get_str_prop("child").to_string();
    let action = model.get_prop("action").cloned();
    let surface_ctx = use_context::<Signal<SurfaceModel>>();
    let surface_id_clone = surface_ctx.read().id.read().clone();
    let component_id = model.id.clone();

    // Check if there is an action handler provided
    let action_ctx = try_use_context::<A2uiActionContext>();

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
                        let name = event.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let context = event.get("context").cloned().unwrap_or_default();
                        
                        // Emit the action if a handler is configured
                        if let Some(handler) = &action_ctx {
                            let action_payload = crate::protocol::ClientAction {
                                name: name.to_string(),
                                surface_id: surface_id_clone.clone(),
                                source_component_id: component_id.clone(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                                context,
                            };
                            handler.0.call(action_payload);
                        } else {
                            // No handler configured
                        }
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
