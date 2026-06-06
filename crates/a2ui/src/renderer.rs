//! Recursive A2UI component dispatcher.
//!
//! `A2uiComponent` looks up a component by ID from the surface's component map
//! and dispatches to the appropriate catalog component implementation.

use dioxus::prelude::*;

use crate::components::*;
use crate::data_model::DataModel;
use crate::surface::SurfaceModel;

/// The recursive heart of the A2UI renderer.
///
/// Given a component ID, it looks up the component model from the surface,
/// determines its type, and renders the appropriate Dioxus component.
#[component]
pub fn A2uiComponent(
    component_id: String,
    data_model: DataModel,
    base_path: String,
) -> Element {
    let surface = use_context::<Signal<SurfaceModel>>();
    let model = {
        let s = surface.read();
        s.get_component(&component_id)
    };

    match model {
        None => {
            // Component not yet available — progressive rendering placeholder
            rsx! {}
        }
        Some(model) => {
            match model.component_type.as_str() {
                "Text" => rsx! {
                    A2uiText { model: model, data_model: data_model, base_path: base_path }
                },
                "Row" => rsx! {
                    A2uiRow { model: model, data_model: data_model, base_path: base_path }
                },
                "Column" => rsx! {
                    A2uiColumn { model: model, data_model: data_model, base_path: base_path }
                },
                "Button" => rsx! {
                    A2uiButton { model: model, data_model: data_model, base_path: base_path }
                },
                "TextField" => rsx! {
                    A2uiTextField { model: model, data_model: data_model, base_path: base_path }
                },
                _ => rsx! {
                    A2uiFallback { model: model }
                },
            }
        }
    }
}
