//! Surface models — the top-level containers for A2UI rendering.

use std::collections::HashMap;

use dioxus::prelude::*;
use serde_json::Value;

use crate::component::ComponentModel;
use crate::data_model::DataModel;

/// A single A2UI surface — one rendered UI tree.
#[derive(Clone, Copy)]
pub struct SurfaceModel {
    pub id: Signal<String>,
    pub catalog_id: Signal<String>,
    pub theme: Signal<Option<Value>>,
    pub send_data_model: Signal<bool>,
    pub data_model: DataModel,
    pub components: Signal<HashMap<String, ComponentModel>>,
}

impl SurfaceModel {
    /// Create a new surface with the given ID and catalog.
    pub fn new(surface_id: &str, catalog_id: &str) -> Self {
        Self {
            id: Signal::new(surface_id.to_string()),
            catalog_id: Signal::new(catalog_id.to_string()),
            theme: Signal::new(None),
            send_data_model: Signal::new(false),
            data_model: DataModel::new(),
            components: Signal::new(HashMap::new()),
        }
    }

    /// Upsert components into this surface.
    ///
    /// Each component JSON value is parsed into a `ComponentModel` and inserted
    /// (or replaced) by its ID.
    pub fn upsert_components(&mut self, raw_components: &[Value]) {
        let mut comps = self.components.write();
        for raw in raw_components {
            if let Some(model) = ComponentModel::from_value(raw) {
                comps.insert(model.id.clone(), model);
            }
        }
    }

    /// Get a component by ID (cloned snapshot).
    pub fn get_component(&self, id: &str) -> Option<ComponentModel> {
        self.components.read().get(id).cloned()
    }

    /// Update the data model at a given path.
    pub fn update_data(&mut self, path: Option<&str>, value: Option<Value>) {
        let mut dm = self.data_model;
        let path = path.unwrap_or("/");
        match value {
            Some(v) => dm.set(path, v),
            None => {
                dm.remove(path);
            }
        }
    }
}

/// A collection of all active surfaces.
#[derive(Clone, Copy)]
pub struct SurfaceGroupModel {
    pub surfaces: Signal<HashMap<String, SurfaceModel>>,
}

impl SurfaceGroupModel {
    /// Create an empty surface group.
    pub fn new() -> Self {
        Self {
            surfaces: Signal::new(HashMap::new()),
        }
    }

    /// Create a new surface and add it to the group.
    pub fn create_surface(
        &mut self,
        surface_id: &str,
        catalog_id: &str,
        theme: Option<Value>,
        send_data_model: bool,
    ) -> SurfaceModel {
        let mut surface = SurfaceModel::new(surface_id, catalog_id);
        surface.theme.set(theme);
        surface.send_data_model.set(send_data_model);
        self.surfaces
            .write()
            .insert(surface_id.to_string(), surface);
        surface
    }

    /// Get a surface by ID.
    pub fn get_surface(&self, surface_id: &str) -> Option<SurfaceModel> {
        self.surfaces.read().get(surface_id).copied()
    }

    /// Delete a surface by ID.
    pub fn delete_surface(&mut self, surface_id: &str) -> bool {
        self.surfaces.write().remove(surface_id).is_some()
    }
}

impl Default for SurfaceGroupModel {
    fn default() -> Self {
        Self::new()
    }
}
