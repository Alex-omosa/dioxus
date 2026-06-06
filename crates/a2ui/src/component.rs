//! Component model for A2UI surfaces.

use serde_json::Value;

/// A single A2UI component's parsed state.
///
/// Component properties are kept as raw `serde_json::Value` because they are
/// catalog-specific and resolved lazily during rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentModel {
    /// The unique ID of this component within the surface.
    pub id: String,
    /// The component type name (e.g. "Text", "Button", "Row").
    pub component_type: String,
    /// All properties as raw JSON (everything except `id` and `component`).
    pub properties: Value,
}

impl ComponentModel {
    /// Parse a component from its raw JSON value.
    ///
    /// Expects the JSON to have `"id"` and `"component"` string fields.
    /// All other fields are preserved in `properties`.
    pub fn from_value(value: &Value) -> Option<Self> {
        let obj = value.as_object()?;
        let id = obj.get("id")?.as_str()?.to_string();
        let component_type = obj.get("component")?.as_str()?.to_string();

        // Build properties: everything except "id" and "component"
        let mut properties = serde_json::Map::new();
        for (key, val) in obj {
            if key != "id" && key != "component" {
                properties.insert(key.clone(), val.clone());
            }
        }

        Some(Self {
            id,
            component_type,
            properties: Value::Object(properties),
        })
    }

    /// Get a property value by key.
    pub fn get_prop(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    /// Get a string property, returning empty string as fallback.
    pub fn get_str_prop(&self, key: &str) -> &str {
        self.properties
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_text_component() {
        let raw = json!({
            "id": "title",
            "component": "Text",
            "text": "Hello World",
            "variant": "h1"
        });
        let model = ComponentModel::from_value(&raw).unwrap();
        assert_eq!(model.id, "title");
        assert_eq!(model.component_type, "Text");
        assert_eq!(model.get_str_prop("variant"), "h1");
        assert_eq!(model.get_prop("text"), Some(&json!("Hello World")));
    }

    #[test]
    fn parse_button_with_action() {
        let raw = json!({
            "id": "btn",
            "component": "Button",
            "child": "label",
            "action": {"event": {"name": "click"}}
        });
        let model = ComponentModel::from_value(&raw).unwrap();
        assert_eq!(model.component_type, "Button");
        assert_eq!(model.get_str_prop("child"), "label");
        assert!(model.get_prop("action").is_some());
    }

    #[test]
    fn parse_missing_id() {
        let raw = json!({"component": "Text", "text": "no id"});
        assert!(ComponentModel::from_value(&raw).is_none());
    }

    #[test]
    fn parse_missing_component() {
        let raw = json!({"id": "x", "text": "no type"});
        assert!(ComponentModel::from_value(&raw).is_none());
    }
}
