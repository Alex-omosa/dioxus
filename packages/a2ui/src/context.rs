//! Data and surface context for scoped path resolution.
//!
//! `DataContext` enables relative JSON Pointer resolution within collection
//! templates, where each iteration has a different base path.

use serde_json::Value;
use dioxus::prelude::*;

use crate::data_model::DataModel;
use crate::protocol::ClientAction;
use crate::pointer;

/// Context type for passing the action callback down to A2UI components
#[derive(Clone)]
pub struct A2uiActionContext(pub Callback<ClientAction>);

/// Provides scoped access to the data model with a base path for relative
/// pointer resolution.
#[derive(Clone, Copy)]
pub struct DataContext {
    data_model: DataModel,
    base_path: &'static str,
}

impl DataContext {
    /// Create a root-level data context (no base path).
    pub fn root(data_model: DataModel) -> Self {
        Self {
            data_model,
            base_path: "",
        }
    }

    /// Resolve a dynamic value.
    ///
    /// A dynamic value in A2UI can be:
    /// - A literal (string, number, boolean, array) → returned as-is
    /// - A data binding `{"path": "..."}` → resolved against the data model
    /// - A function call `{"call": "...", "args": {...}}` → resolved by the caller
    pub fn resolve_dynamic(&self, value: &Value) -> ResolvedValue {
        match value {
            // Data binding: {"path": "some/pointer"}
            Value::Object(obj) if obj.contains_key("path") => {
                if let Some(path) = obj.get("path").and_then(|p| p.as_str()) {
                    let abs_path = pointer::make_absolute(path, self.base_path);
                    let resolved = self.data_model.get(&abs_path);
                    ResolvedValue::Resolved(resolved.unwrap_or(Value::Null))
                } else {
                    ResolvedValue::Resolved(Value::Null)
                }
            }
            // Function call: {"call": "...", "args": {...}}
            Value::Object(obj) if obj.contains_key("call") => {
                ResolvedValue::FunctionCall(value.clone())
            }
            // Literal value
            _ => ResolvedValue::Resolved(value.clone()),
        }
    }

    /// Resolve a dynamic value to a concrete JSON value.
    ///
    /// Function calls are NOT resolved here — they return Null.
    /// Use `resolve_dynamic` if you need to handle function calls.
    pub fn resolve_value(&self, value: &Value) -> Value {
        match self.resolve_dynamic(value) {
            ResolvedValue::Resolved(v) => v,
            ResolvedValue::FunctionCall(_) => Value::Null,
        }
    }

    /// Resolve a dynamic value to a string.
    pub fn resolve_string(&self, value: &Value) -> String {
        let resolved = self.resolve_value(value);
        match &resolved {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            other => other.to_string(),
        }
    }

    /// Resolve a dynamic value to a bool.
    pub fn resolve_bool(&self, value: &Value) -> bool {
        let resolved = self.resolve_value(value);
        match &resolved {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
            _ => true,
        }
    }

    /// Get the base path of this context.
    pub fn base_path(&self) -> &str {
        self.base_path
    }

    /// Get the underlying data model.
    pub fn data_model(&self) -> DataModel {
        self.data_model
    }

    /// Resolve the binding path for two-way binding.
    ///
    /// Given a dynamic value that is a data binding, returns the absolute path
    /// for writing back to the data model.
    pub fn resolve_binding_path(&self, value: &Value) -> Option<String> {
        let obj = value.as_object()?;
        let path = obj.get("path")?.as_str()?;
        Some(pointer::make_absolute(path, self.base_path))
    }
}

/// The result of resolving a dynamic value.
#[derive(Debug, Clone)]
pub enum ResolvedValue {
    /// A concrete JSON value.
    Resolved(Value),
    /// A function call that needs to be executed by the catalog function registry.
    FunctionCall(Value),
}

/// A runtime data context that owns its base path string.
///
/// Unlike `DataContext` which uses a `&'static str`, this variant can hold
/// dynamic base paths constructed at runtime (e.g., for template iterations).
#[derive(Clone)]
pub struct OwnedDataContext {
    data_model: DataModel,
    base_path: String,
}

impl OwnedDataContext {
    /// Create a root-level context.
    pub fn root(data_model: DataModel) -> Self {
        Self {
            data_model,
            base_path: String::new(),
        }
    }

    /// Create a child context with a new base path.
    pub fn with_base(data_model: DataModel, base_path: String) -> Self {
        Self {
            data_model,
            base_path,
        }
    }

    /// Resolve a dynamic value.
    pub fn resolve_dynamic(&self, value: &Value) -> ResolvedValue {
        match value {
            Value::Object(obj) if obj.contains_key("path") => {
                if let Some(path) = obj.get("path").and_then(|p| p.as_str()) {
                    let abs_path = pointer::make_absolute(path, &self.base_path);
                    let resolved = self.data_model.get(&abs_path);
                    ResolvedValue::Resolved(resolved.unwrap_or(Value::Null))
                } else {
                    ResolvedValue::Resolved(Value::Null)
                }
            }
            Value::Object(obj) if obj.contains_key("call") => {
                ResolvedValue::FunctionCall(value.clone())
            }
            _ => ResolvedValue::Resolved(value.clone()),
        }
    }

    /// Resolve to a concrete value (function calls return Null).
    pub fn resolve_value(&self, value: &Value) -> Value {
        match self.resolve_dynamic(value) {
            ResolvedValue::Resolved(v) => v,
            ResolvedValue::FunctionCall(_) => Value::Null,
        }
    }

    /// Resolve to a string.
    pub fn resolve_string(&self, value: &Value) -> String {
        let resolved = self.resolve_value(value);
        match &resolved {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            other => other.to_string(),
        }
    }

    /// Resolve the binding path for two-way binding.
    pub fn resolve_binding_path(&self, value: &Value) -> Option<String> {
        let obj = value.as_object()?;
        let path = obj.get("path")?.as_str()?;
        Some(pointer::make_absolute(path, &self.base_path))
    }

    /// Get the base path.
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    /// Get the data model.
    pub fn data_model(&self) -> DataModel {
        self.data_model
    }
}
