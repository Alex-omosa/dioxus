//! A2UI Surface view — the top-level Dioxus component that renders a surface.

use dioxus::prelude::*;

use crate::renderer::A2uiComponent;
use crate::surface::SurfaceModel;

/// Renders a single A2UI surface.
///
/// This component:
/// 1. Provides the `SurfaceModel` as context so child components can access it
/// 2. Starts recursive rendering from the "root" component
#[component]
pub fn A2uiSurface(surface: SurfaceModel) -> Element {
    // Provide the surface as context for child components (Button needs it for actions)
    use_context_provider(|| surface);

    let data_model = surface.data_model;
    let root_id = surface.get_root_component_id();

    rsx! {
        div { class: "a2ui-surface",
            A2uiComponent {
                component_id: root_id,
                data_model,
                base_path: String::new(),
            }
        }
    }
}
