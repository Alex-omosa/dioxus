use a2ui::{A2uiMessage, MessageProcessor};
use dioxus::{html::KeyCode::A, prelude::*};
use futures::StreamExt;
use nats::{provider::NatsContext, try_use_nats, try_use_nats_signal, NatsProvider};
use serde_json::Deserializer;

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
            A2uiProvider { Home {} }
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
                                    let payload_str =
                                        String::from_utf8_lossy(&msg.payload).to_string();

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

#[component]
pub fn A2uiProvider(children: Element) -> Element {
    // 1. Provide the MessageProcessor to the Dioxus tree.
    // Because MessageProcessor is `Copy` and contains `Signal`s,
    // Dioxus can track mutations to those inner signals perfectly!
    let mut processor = use_context_provider(|| MessageProcessor::new());

    // 2. Get the NATS client from the parent NatsProvider
    let nats_signal = try_use_nats_signal();
    let mut has_subscribed = use_signal(|| false);

    // 3. React to the NATS connection state
    use_effect(move || {
        if let Some(signal) = nats_signal {
            // Read the signal to subscribe to connection changes
            if let Some(client) = signal.read().clone() {
                if !*has_subscribed.read() {
                    has_subscribed.set(true);

                    // 4. Spawn the async listener in the background
                    spawn(async move {
                        match client.subscribe("a2ui.ui").await {
                            Ok(mut subscriber) => {
                                tracing::info!("✅ A2UI Provider subscribed to 'a2ui.ui'");

                                // 5. The Message Entry Point
                                let mut seq: u64 = 0;
                                while let Some(msg) = subscriber.next().await {
                                    // Trim leading/trailing whitespace/newlines from the payload
                                    // so the streaming deserializer doesn't attempt to parse an
                                    // extra empty value and produce spurious errors.
                                    let payload_str = String::from_utf8_lossy(&msg.payload);
                                    let trimmed = payload_str.trim();
                                    seq += 1;
                                    tracing::info!("📨 NATS message #{} ({} bytes)", seq, msg.payload.len());

                                    // Parse NDJSON safely: split on lines and skip blanks.
                                    for line in trimmed.lines() {
                                        let line = line.trim();
                                        if line.is_empty() {
                                            continue;
                                        }

                                        match serde_json::from_str::<a2ui::A2uiMessage>(line) {
                                            Ok(a2ui_msg) => {
                                                tracing::info!(
                                                    "✅ Processing A2UI message: {:?}",
                                                    a2ui_msg.payload
                                                );
                                                processor.process(&a2ui_msg);
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "❌ Failed to parse A2UI message: {:?}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("❌ Failed to subscribe to NATS: {:?}", e);
                                // Optional: reset has_subscribed to allow retry logic later
                                has_subscribed.set(false);
                            }
                        }
                    });
                }
            }
        }
    });

    rsx! {
        {children}

    }
}
