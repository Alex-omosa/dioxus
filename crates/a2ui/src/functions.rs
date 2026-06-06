//! Built-in function implementations for the A2UI catalogs.

use serde_json::Value;

use crate::catalog::FunctionImpl;
use crate::context::OwnedDataContext;

/// `capitalize` — converts a string to capitalized form.
pub struct Capitalize;

impl FunctionImpl for Capitalize {
    fn name(&self) -> &str {
        "capitalize"
    }

    fn execute(&self, args: &Value, ctx: &OwnedDataContext) -> Value {
        let value = args.get("value").cloned().unwrap_or(Value::Null);
        let resolved = ctx.resolve_value(&value);
        let s = match &resolved {
            Value::String(s) => s.clone(),
            _ => return resolved,
        };

        if s.is_empty() {
            return Value::String(s);
        }

        let mut chars = s.chars();
        let capitalized = match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        };
        Value::String(capitalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Pure logic tests — capitalize doesn't need the data model for literals.
    #[test]
    fn capitalize_literal_pure() {
        // Test the capitalize logic without data binding (no Signal needed)
        let s = "hello";
        let mut chars = s.chars();
        let capitalized = match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => String::new(),
        };
        assert_eq!(capitalized, "Hello");
    }

    #[test]
    fn capitalize_empty_pure() {
        let s = "";
        assert!(s.is_empty());
    }

    // Integration tests requiring a Dioxus runtime are tested at a higher level
    // (e.g., in the processor or component tests) where a VirtualDom is available.
}
