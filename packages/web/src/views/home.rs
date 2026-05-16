use dioxus::prelude::*;
use a2ui::protocol::ExampleFile;
use a2ui::{A2uiSurface, MessageProcessor};

const A2UI_CSS: Asset = asset!("/assets/a2ui.css");

/// Mock A2UI messages — Login Form example from the specification.
const LOGIN_FORM_JSON: &str = r#"{
    "name": "Login Form",
    "description": "Form with input fields and action",
    "messages": [
        {
            "version": "v0.9",
            "createSurface": {
                "surfaceId": "login_form",
                "catalogId": "https://a2ui.org/specification/v0_9/catalogs/minimal/minimal_catalog.json",
                "sendDataModel": true
            }
        },
        {
            "version": "v0.9",
            "updateComponents": {
                "surfaceId": "login_form",
                "components": [
                    {
                        "id": "root",
                        "component": "Column",
                        "children": ["form_title", "username_field", "password_field", "submit_button"],
                        "justify": "start",
                        "align": "stretch"
                    },
                    {
                        "id": "form_title",
                        "component": "Text",
                        "text": "Login",
                        "variant": "h2"
                    },
                    {
                        "id": "username_field",
                        "component": "TextField",
                        "label": "Username",
                        "value": { "path": "/username" },
                        "variant": "shortText"
                    },
                    {
                        "id": "password_field",
                        "component": "TextField",
                        "label": "Password",
                        "value": { "path": "/password" },
                        "variant": "obscured"
                    },
                    {
                        "id": "submit_button",
                        "component": "Button",
                        "child": "submit_label",
                        "variant": "primary",
                        "action": {
                            "event": {
                                "name": "login_submitted",
                                "context": {
                                    "user": { "path": "/username" },
                                    "pass": { "path": "/password" }
                                }
                            }
                        }
                    },
                    {
                        "id": "submit_label",
                        "component": "Text",
                        "text": "Sign In"
                    }
                ]
            }
        }
    ]
}"#;

/// Mock A2UI messages — Restaurant list (incremental/template rendering).
const RESTAURANT_LIST_JSON: &str = r#"{
    "name": "Restaurant List",
    "description": "Template-based collection rendering with data binding",
    "messages": [
        {
            "version": "v0.9",
            "createSurface": {
                "surfaceId": "restaurants",
                "catalogId": "https://a2ui.org/specification/v0_9/catalogs/minimal/minimal_catalog.json"
            }
        },
        {
            "version": "v0.9",
            "updateDataModel": {
                "surfaceId": "restaurants",
                "path": "/",
                "value": {
                    "restaurants": [
                        { "title": "The Golden Fork", "subtitle": "Fine Dining & Spirits" },
                        { "title": "Ocean's Bounty", "subtitle": "Fresh Daily Seafood" },
                        { "title": "Pizzeria Roma", "subtitle": "Authentic Wood-Fired Pizza" }
                    ]
                }
            }
        },
        {
            "version": "v0.9",
            "updateComponents": {
                "surfaceId": "restaurants",
                "components": [
                    {
                        "id": "root",
                        "component": "Column",
                        "children": { "path": "/restaurants", "componentId": "restaurant_card" }
                    }
                ]
            }
        },
        {
            "version": "v0.9",
            "updateComponents": {
                "surfaceId": "restaurants",
                "components": [
                    {
                        "id": "restaurant_card",
                        "component": "Column",
                        "children": ["rc_title", "rc_subtitle"]
                    },
                    {
                        "id": "rc_title",
                        "component": "Text",
                        "text": { "path": "title" },
                        "variant": "h3"
                    },
                    {
                        "id": "rc_subtitle",
                        "component": "Text",
                        "text": { "path": "subtitle" },
                        "variant": "caption"
                    }
                ]
            }
        }
    ]
}"#;

/// Renders a demo page with two mock A2UI surfaces.
#[component]
pub fn Home() -> Element {
    // Initialize surfaces ONCE — use_hook runs only on first render.
    let (login_surface, restaurant_surface) = use_hook(|| {
        let login_example: ExampleFile = serde_json::from_str(LOGIN_FORM_JSON).unwrap();
        let restaurant_example: ExampleFile = serde_json::from_str(RESTAURANT_LIST_JSON).unwrap();

        let mut login_proc = MessageProcessor::new();
        login_proc.process_all(&login_example.messages);

        let mut restaurant_proc = MessageProcessor::new();
        restaurant_proc.process_all(&restaurant_example.messages);

        let login = login_proc.surfaces.get_surface("login_form");
        let restaurant = restaurant_proc.surfaces.get_surface("restaurants");
        (login, restaurant)
    });

    // Live data model view — reads the signal, so it updates on every keystroke
    let login_data = login_surface
        .map(|s| {
            let snap = s.data_model.snapshot();
            serde_json::to_string_pretty(&snap).unwrap_or_default()
        })
        .unwrap_or_default();

    rsx! {
        document::Link { rel: "stylesheet", href: A2UI_CSS }

        div { class: "a2ui-demo-page",
            div { class: "a2ui-demo-header",
                h1 { class: "a2ui-demo-title", "A2UI Renderer Demo" }
                p { class: "a2ui-demo-subtitle", "Dioxus 0.7 × A2UI v0.9 — Minimal Catalog" }
            }

            div { class: "a2ui-demo-grid",
                // Login Form
                div { class: "a2ui-demo-card",
                    div { class: "a2ui-demo-card-title", "Login Form — two-way binding" }
                    if let Some(surface) = login_surface {
                        A2uiSurface { surface: surface }
                    }

                    div { class: "a2ui-demo-data",
                        div { class: "a2ui-demo-data-title", "Live Data Model" }
                        pre { "{login_data}" }
                    }
                }

                // Restaurant List
                div { class: "a2ui-demo-card",
                    div { class: "a2ui-demo-card-title", "Restaurant List — template rendering" }
                    if let Some(surface) = restaurant_surface {
                        A2uiSurface { surface: surface }
                    }
                }
            }
        }
    }
}
