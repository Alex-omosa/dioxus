//! A2UI v0.9 protocol message types.
//!
//! Strongly-typed Rust structs mirroring the JSON schemas from the A2UI specification.
//! Uses serde for (de)serialization from the A2UI JSON stream.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Server → Client messages
// ---------------------------------------------------------------------------

/// A payload variant for a server-to-client A2UI message.
///
/// Exactly one variant is present per message. Serde's `untagged` + `flatten`
/// combination ensures the JSON wire format remains a flat object with
/// `"version"` plus the variant name as a key (e.g., `"createSurface"`).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum A2uiPayload {
    CreateSurface(CreateSurface),
    UpdateComponents(UpdateComponents),
    UpdateDataModel(UpdateDataModel),
    DeleteSurface(DeleteSurface),
}

/// A single A2UI server-to-client message envelope.
///
/// The JSON wire format uses a flat object with `"version"` plus exactly one
/// of the four payload keys. The payload field uses `#[serde(flatten)]` to merge
/// enum variants into the top-level message object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct A2uiMessage {
    pub version: String,
    #[serde(flatten)]
    pub payload: A2uiPayload,
}

// ---------------------------------------------------------------------------
// Payload structs
// ---------------------------------------------------------------------------

/// `createSurface` — signals the client to create a new surface.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSurface {
    pub surface_id: String,
    pub catalog_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_data_model: Option<bool>,
}

/// `updateComponents` — adds or replaces components in a surface.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateComponents {
    pub surface_id: String,
    pub components: Vec<Value>,
}

/// `updateDataModel` — patches the data model for a surface.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDataModel {
    pub surface_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

/// `deleteSurface` — removes a surface.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSurface {
    pub surface_id: String,
}

// ---------------------------------------------------------------------------
// Client → Server messages
// ---------------------------------------------------------------------------

/// A client-to-server action event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientAction {
    pub name: String,
    pub surface_id: String,
    pub source_component_id: String,
    pub timestamp: String,
    pub context: Value,
}

/// A client-to-server error report.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientError {
    pub code: String,
    pub surface_id: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Client-to-server message envelope.
#[derive(Debug, Clone, Serialize)]
pub struct ClientMessage {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ClientAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ClientError>,
}

impl ClientMessage {
    pub fn action(action: ClientAction) -> Self {
        Self {
            version: "v0.9".into(),
            action: Some(action),
            error: None,
        }
    }

    pub fn error(error: ClientError) -> Self {
        Self {
            version: "v0.9".into(),
            action: None,
            error: Some(error),
        }
    }
}

// ---------------------------------------------------------------------------
// Example message list wrapper (used in test examples)
// ---------------------------------------------------------------------------

/// Wrapper for example files that contain a list of messages.
#[derive(Debug, Clone, Deserialize)]
pub struct ExampleFile {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub messages: Vec<A2uiMessage>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_create_surface() {
        let json = r#"{
            "version": "v0.9",
            "createSurface": {
                "surfaceId": "test_1",
                "catalogId": "https://example.com/catalog.json",
                "sendDataModel": true
            }
        }"#;
        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.version, "v0.9");
        match &msg.payload {
            A2uiPayload::CreateSurface(cs) => {
                assert_eq!(cs.surface_id, "test_1");
                assert_eq!(cs.catalog_id, "https://example.com/catalog.json");
                assert_eq!(cs.send_data_model, Some(true));
            }
            _ => panic!("Expected CreateSurface variant"),
        }
    }

    #[test]
    fn parse_update_components() {
        let json = r#"{
            "version": "v0.9",
            "updateComponents": {
                "surfaceId": "test_1",
                "components": [
                    {"id": "root", "component": "Column", "children": ["title"]},
                    {"id": "title", "component": "Text", "text": "Hello"}
                ]
            }
        }"#;
        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match &msg.payload {
            A2uiPayload::UpdateComponents(uc) => {
                assert_eq!(uc.surface_id, "test_1");
                assert_eq!(uc.components.len(), 2);
            }
            _ => panic!("Expected UpdateComponents variant"),
        }
    }

    #[test]
    fn parse_update_data_model() {
        let json = r#"{
            "version": "v0.9",
            "updateDataModel": {
                "surfaceId": "test_1",
                "path": "/username",
                "value": "alice"
            }
        }"#;
        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match &msg.payload {
            A2uiPayload::UpdateDataModel(udm) => {
                assert_eq!(udm.surface_id, "test_1");
                assert_eq!(udm.path.as_deref(), Some("/username"));
            }
            _ => panic!("Expected UpdateDataModel variant"),
        }
    }

    #[test]
    fn parse_delete_surface() {
        let json = r#"{
            "version": "v0.9",
            "deleteSurface": {
                "surfaceId": "test_1"
            }
        }"#;
        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match &msg.payload {
            A2uiPayload::DeleteSurface(ds) => {
                assert_eq!(ds.surface_id, "test_1");
            }
            _ => panic!("Expected DeleteSurface variant"),
        }
    }

    #[test]
    fn parse_example_file() {
        let json = r#"{
            "name": "Test",
            "description": "A test",
            "messages": [
                {"version": "v0.9", "createSurface": {"surfaceId": "s1", "catalogId": "cat"}}
            ]
        }"#;
        let example: ExampleFile = serde_json::from_str(json).unwrap();
        assert_eq!(example.messages.len(), 1);
    }

    #[test]
    fn serialize_client_action() {
        let msg = ClientMessage::action(ClientAction {
            name: "submit".into(),
            surface_id: "s1".into(),
            source_component_id: "btn1".into(),
            timestamp: "2026-01-01T00:00:00Z".into(),
            context: serde_json::json!({"key": "value"}),
        });
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"action\""));
        assert!(json.contains("\"submit\""));
    }
}
