//! Message processor — parses A2UI messages and mutates the surface models.

use crate::protocol::{A2uiMessage, A2uiPayload};
use crate::surface::SurfaceGroupModel;

/// The central controller that accepts parsed A2UI messages and orchestrates
/// updates to the surface group model.
#[derive(Clone, Copy)]
pub struct MessageProcessor {
    pub surfaces: SurfaceGroupModel,
}

impl MessageProcessor {
    /// Create a new processor with an empty surface group.
    pub fn new() -> Self {
        Self {
            surfaces: SurfaceGroupModel::new(),
        }
    }

    /// Create a processor wrapping an existing surface group.
    pub fn with_surfaces(surfaces: SurfaceGroupModel) -> Self {
        Self { surfaces }
    }

    /// Process a single A2UI message.
    pub fn process(&mut self, msg: &A2uiMessage) {
        match &msg.payload {
            A2uiPayload::CreateSurface(cs) => {
                self.surfaces.create_surface(
                    &cs.surface_id,
                    &cs.catalog_id,
                    cs.theme.clone(),
                    cs.send_data_model.unwrap_or(false),
                );
            }
            A2uiPayload::UpdateComponents(uc) => {
                if let Some(mut surface) = self.surfaces.get_surface(&uc.surface_id) {
                    surface.upsert_components(&uc.components);
                }
            }
            A2uiPayload::UpdateDataModel(udm) => {
                if let Some(mut surface) = self.surfaces.get_surface(&udm.surface_id) {
                    surface.update_data(udm.path.as_deref(), udm.value.clone());
                }
            }
            A2uiPayload::DeleteSurface(ds) => {
                self.surfaces.delete_surface(&ds.surface_id);
            }
        }
    }

    /// Process a list of messages in order (e.g., from an example file).
    pub fn process_all(&mut self, messages: &[A2uiMessage]) {
        for msg in messages {
            self.process(msg);
        }
    }
}

impl Default for MessageProcessor {
    fn default() -> Self {
        Self::new()
    }
}
