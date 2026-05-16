//! Custom Dioxus hooks for A2UI data binding and validation.

use dioxus::prelude::*;
use serde_json::Value;

use crate::context::OwnedDataContext;
use crate::data_model::DataModel;
use crate::pointer;

/// Resolve a dynamic A2UI value reactively.
///
/// Returns a `Memo<String>` that auto-updates when the underlying data changes.
///
/// Handles:
/// - Literal string → returned as-is
/// - Data binding `{"path": "..."}` → resolved from the data model
/// - Other literals (number, bool) → converted to string
pub fn use_dynamic_string(
    value: Value,
    data_model: DataModel,
    base_path: String,
) -> Memo<String> {
    use_memo(move || {
        let ctx = OwnedDataContext::with_base(data_model, base_path.clone());
        ctx.resolve_string(&value)
    })
}

/// Resolve a dynamic A2UI value to a raw JSON Value reactively.
pub fn use_dynamic_value(
    value: Value,
    data_model: DataModel,
    base_path: String,
) -> Memo<Value> {
    use_memo(move || {
        let ctx = OwnedDataContext::with_base(data_model, base_path.clone());
        ctx.resolve_value(&value)
    })
}

/// Two-way binding hook for input components (TextField, Slider, etc.).
///
/// Returns a reactive read value and a callback to write back to the data model.
pub fn use_two_way_binding(
    value_prop: &Value,
    data_model: DataModel,
    base_path: &str,
) -> (Memo<String>, Option<String>) {
    let binding_path = value_prop
        .as_object()
        .and_then(|obj| obj.get("path"))
        .and_then(|p| p.as_str())
        .map(|p| pointer::make_absolute(p, base_path));

    let read_value = {
        let value_prop = value_prop.clone();
        let base_path = base_path.to_string();
        use_memo(move || {
            let ctx = OwnedDataContext::with_base(data_model, base_path.clone());
            ctx.resolve_string(&value_prop)
        })
    };

    (read_value, binding_path)
}

/// Validation hook for components with `checks` (Checkable trait).
///
/// Returns a `Memo<Vec<String>>` containing failed check messages.
/// Empty vec = all checks pass.
pub fn use_validation(
    checks: Vec<Value>,
    data_model: DataModel,
    base_path: String,
) -> Memo<Vec<String>> {
    use_memo(move || {
        let ctx = OwnedDataContext::with_base(data_model, base_path.clone());
        checks
            .iter()
            .filter_map(|check| {
                let condition = check.get("condition")?;
                let resolved = ctx.resolve_value(condition);
                let passes = match &resolved {
                    Value::Bool(b) => *b,
                    Value::Null => false,
                    _ => true,
                };
                if !passes {
                    let msg = check
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Validation failed")
                        .to_string();
                    Some(msg)
                } else {
                    None
                }
            })
            .collect()
    })
}
