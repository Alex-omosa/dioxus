//! # A2UI — Dioxus Renderer
//!
//! A renderer for the A2UI v0.9 protocol built on Dioxus 0.7.
//!
//! ## Architecture
//!
//! - **Data Layer**: `protocol`, `pointer`, `data_model`, `component`, `surface`,
//!   `context`, `catalog`, `functions`, `processor`
//! - **Rendering Layer**: `hooks`, `surface_view`, `renderer`, `components`

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

// Rendering layer
pub mod hooks;
pub mod surface_view;
pub mod renderer;
pub mod components;

// Re-exports for convenience
pub use data_model::DataModel;
pub use component::ComponentModel;
pub use surface::{SurfaceModel, SurfaceGroupModel};
pub use processor::MessageProcessor;
pub use catalog::Catalog;
pub use protocol::{A2uiMessage, ClientAction, ClientMessage};
pub use surface_view::A2uiSurface;
