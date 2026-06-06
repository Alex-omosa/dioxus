use a2ui::MessageProcessor;
use dioxus::prelude::*;
use futures::StreamExt;
use nats::{provider::NatsContext, try_use_nats, try_use_nats_signal, NatsProvider};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        NatsProvider {
            url: "ws://localhost:8081".to_string(),
            user: Some("auth".to_string()),
            pass: Some("auth".to_string()),
            Home {}
        }
    }
}

#[component]
fn Home() -> Element {
    let nats_signal = try_use_nats_signal();
    
    // 1. Create a signal to store incoming messages so the UI can react to them
    let mut messages = use_signal(|| Vec::<String>::new());
    
    // 2. Track if we have already initiated a subscription to prevent duplicate background tasks
    let mut has_subscribed = use_signal(|| false);

    // 3. use_effect runs synchronously, but allows us to react to signal changes
    use_effect(move || {
        // Check if the context exists
        if let Some(signal) = nats_signal {
            // Read the signal. This subscribes the effect to re-run when the client connects!
            let client_opt = signal.read().clone();
            
            // If the client is connected and we haven't subscribed yet...
            if let Some(client) = client_opt {
                if !*has_subscribed.read() {
                    has_subscribed.set(true);
                    
                    // 4. Spawn an async task to handle the subscription in the background
                    spawn(async move {
                        match client.subscribe("a2ui.ui").await {
                            Ok(mut subscriber) => {
                                tracing::info!("Successfully subscribed to a2ui.ui");
                                
                                // 5. Continuously read from the stream
                                while let Some(msg) = subscriber.next().await {
                                    // Convert payload to string. 
                                    // This works for both Native (bytes::Bytes) and Wasm (Vec<u8>) 
                                    // because both dereference to &[u8].
                                    let payload_str = String::from_utf8_lossy(&msg.payload).to_string();
                                    
                                    // Push to signal, triggering a UI re-render
                                    messages.write().push(payload_str);
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to subscribe to a2ui.ui: {:?}", e);
                                // Optional: reset has_subscribed if you want it to retry on failure
                                // has_subscribed.set(false);
                            }
                        }
                    });
                }
            }
        }
    });

    rsx! {
        div { "Welcome to the home page!" }

        // Handle the async connection state safely
        match nats_signal.map(|s| s.read().clone()) {
            Some(Some(_client)) => rsx! {
                div { "✅ NATS is connected and listening to 'a2ui.ui'!" }
            },
            Some(None) => rsx! {
                div { "⏳ NATS is still connecting..." }
            },
            None => rsx! {
                div { "❌ NATS context not found!" }
            },
        }

        // Render the incoming messages
        div {
            h3 { "Received Messages:" }
            ul {
                for msg in messages.read().iter() {
                    li { "{msg}" }
                }
            }
        }
    }
}
// let a2ui_messages_json = fs::read_to_string("assets/a2ui_messages.json")
//     .expect("Failed to read assets/a2ui_messages.json");

// let a2ui_messages: Vec<serde_json::Value> = serde_json::from_str(&a2ui_messages_json).unwrap();

// let mut processor = MessageProcessor::new();
// for msg in &a2ui_messages {
//     let parsed_msg = serde_json::from_value::<a2ui::A2uiMessage>(msg.clone())
//         .expect("Failed to parse A2UI message");

//     processor.process(&parsed_msg);
// }
