//! Reactive data model backed by a Dioxus `Signal<serde_json::Value>`.
//!
//! The `DataModel` wraps the entire surface data as a single JSON value in a
//! signal. Any Dioxus component or memo that reads from the signal auto-subscribes
//! to changes — no manual subscribe/unsubscribe needed.

use dioxus::prelude::*;
use serde_json::Value;

use crate::pointer;

/// A reactive JSON data store for a single A2UI surface.
///
/// Internally holds a `Signal<Value>` so that any read in a reactive scope
/// (component render, memo, effect) automatically tracks changes.
#[derive(Clone, Copy)]
pub struct DataModel {
    data: Signal<Value>,
}

impl DataModel {
    /// Create a new empty data model (empty JSON object).
    pub fn new() -> Self {
        Self {
            data: Signal::new(Value::Object(serde_json::Map::new())),
        }
    }

    /// Create a data model with an initial value.
    pub fn with_value(value: Value) -> Self {
        Self {
            data: Signal::new(value),
        }
    }

    /// Get a value at the given JSON Pointer path.
    ///
    /// This call reads from the signal, subscribing the current reactive scope.
    pub fn get(&self, path: &str) -> Option<Value> {
        pointer::resolve(&self.data.read(), path)
    }

    /// Get a value at a path that may be relative to a base path.
    ///
    /// If `path` starts with `/`, it is absolute; otherwise it is joined with
    /// `base_path`.
    pub fn get_relative(&self, path: &str, base_path: &str) -> Option<Value> {
        let abs = pointer::make_absolute(path, base_path);
        self.get(&abs)
    }

    /// Set a value at the given JSON Pointer path.
    ///
    /// Auto-vivifies intermediate objects/arrays. This triggers a signal write,
    /// which will re-render any subscribed components.
    pub fn set(&mut self, path: &str, value: Value) {
        let mut data = self.data.write();
        pointer::set_at_path(&mut data, path, value);
    }

    /// Remove a value at the given JSON Pointer path.
    ///
    /// Returns the removed value, if any.
    pub fn remove(&mut self, path: &str) -> Option<Value> {
        let mut data = self.data.write();
        pointer::remove_at_path(&mut data, path)
    }

    /// Replace the entire data model.
    pub fn replace(&mut self, value: Value) {
        self.data.set(value);
    }

    /// Read the raw signal (for use in memos that need the full root).
    pub fn signal(&self) -> Signal<Value> {
        self.data
    }

    /// Snapshot the current data model value (cloned).
    pub fn snapshot(&self) -> Value {
        self.data.read().clone()
    }
}

impl Default for DataModel {
    fn default() -> Self {
        Self::new()
    }
}
