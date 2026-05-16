//! Catalog and function registry.
//!
//! A catalog defines which components and functions a surface supports.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde_json::Value;

use crate::context::OwnedDataContext;

/// A client-side function implementation.
pub trait FunctionImpl: Send + Sync {
    /// The function name (e.g., "capitalize", "formatString").
    fn name(&self) -> &str;

    /// Execute the function with the given arguments and data context.
    fn execute(&self, args: &Value, ctx: &OwnedDataContext) -> Value;
}

/// A catalog of known component types and functions.
#[derive(Clone)]
pub struct Catalog {
    /// The catalog URI identifier.
    pub id: String,
    /// Known component type names.
    pub component_types: HashSet<String>,
    /// Registered function implementations (wrapped in Arc for Clone).
    pub functions: HashMap<String, Arc<dyn FunctionImpl>>,
}

impl Catalog {
    /// Create a new empty catalog.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            component_types: HashSet::new(),
            functions: HashMap::new(),
        }
    }

    /// Register a component type as known.
    pub fn register_component(&mut self, name: impl Into<String>) {
        self.component_types.insert(name.into());
    }

    /// Register a function implementation.
    pub fn register_function(&mut self, func: Arc<dyn FunctionImpl>) {
        self.functions.insert(func.name().to_string(), func);
    }

    /// Check if a component type is known to this catalog.
    pub fn has_component(&self, name: &str) -> bool {
        self.component_types.contains(name)
    }

    /// Execute a function call.
    ///
    /// The `call_value` should be the full function call JSON:
    /// `{"call": "name", "args": {...}, "returnType": "string"}`
    pub fn execute_function(&self, call_value: &Value, ctx: &OwnedDataContext) -> Value {
        let call_name = call_value
            .get("call")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let args = call_value.get("args").cloned().unwrap_or(Value::Null);

        if let Some(func) = self.functions.get(call_name) {
            func.execute(&args, ctx)
        } else {
            Value::Null
        }
    }
}

/// Build the minimal catalog with its component types and functions.
pub fn minimal_catalog() -> Catalog {
    use crate::functions;

    let mut catalog = Catalog::new(
        "https://a2ui.org/specification/v0_9/catalogs/minimal/minimal_catalog.json",
    );
    catalog.register_component("Text");
    catalog.register_component("Row");
    catalog.register_component("Column");
    catalog.register_component("Button");
    catalog.register_component("TextField");

    catalog.register_function(Arc::new(functions::Capitalize));

    catalog
}
