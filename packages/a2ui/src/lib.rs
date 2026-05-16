//! # A2UI — Dioxus Renderer
//!
//! A renderer for the A2UI v0.9 protocol built on Dioxus 0.7.
//!
//! ## Architecture
//!
//! - **Data Layer**: `protocol`, `pointer`, `data_model`, `component`, `surface`,
//!   `context`, `catalog`, `functions`, `processor`
//! - **Rendering Layer**: `hooks`, `surface_view`, `renderer`, `components`
//!
//! The data layer uses Dioxus signals directly for reactivity. The rendering
//! layer provides Dioxus components that consume the data layer to paint the UI.

// Data layer
pub mod protocol;
pub mod pointer;
pub mod data_model;
pub mod component;
pub mod surface;
pub mod context;
pub mod catalog;
pub mod functions;
pub mod processor;

// Re-exports for convenience
pub use data_model::DataModel;
pub use component::ComponentModel;
pub use surface::{SurfaceModel, SurfaceGroupModel};
pub use processor::MessageProcessor;
pub use catalog::Catalog;
pub use protocol::{A2uiMessage, ClientAction, ClientMessage};
