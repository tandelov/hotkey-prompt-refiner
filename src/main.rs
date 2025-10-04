mod anthropic;
mod clipboard;
mod config;
mod hotkey;
mod paste;
mod workflow;

use config::Config;
use hotkey::HotkeyManager;
use workflow::execute_workflow_with_logging;
use std::sync::Arc;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

fn main() {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Hotkey Prompt Refiner v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting up...\n");

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => {
            println!("✓ Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            eprintln!("✗ Configuration error: {}", e);
            eprintln!("\nPlease ensure:");
            eprintln!("  1. ANTHROPIC_API_KEY is set in environment or .env file");
            eprintln!("  2. Get your API key from: https://console.anthropic.com/settings/keys");
            std::process::exit(1);
        }
    };

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("✗ Configuration validation failed: {}", e);
        std::process::exit(1);
    }
    println!("✓ Configuration validated");

    // Create API client
    let api_client = Arc::new(anthropic::ApiClient::new(config.api_key.clone(), None));
    let config = Arc::new(config);

    // Initialize hotkey manager
    let hotkey_manager = match HotkeyManager::new() {
        Ok(manager) => {
            println!("✓ Hotkey manager initialized");
            manager
        }
        Err(e) => {
            eprintln!("✗ Failed to create hotkey manager: {}", e);
            std::process::exit(1);
        }
    };

    // Register hotkey
    if let Err(e) = hotkey_manager.register() {
        eprintln!("✗ Failed to register hotkey: {}", e);
        eprintln!("  The hotkey might be in use by another application.");
        std::process::exit(1);
    }
    println!("✓ Hotkey registered: {}", hotkey_manager.hotkey_description());

    #[cfg(target_os = "macos")]
    println!("\n✅ System ready! Press Cmd+Shift+] to process clipboard text.\n");

    #[cfg(not(target_os = "macos"))]
    println!("\n✅ System ready! Press Ctrl+Shift+] to process clipboard text.\n");

    // Create shared tokio runtime in a separate thread
    let (workflow_tx, workflow_rx) = std::sync::mpsc::channel::<()>();

    let config_clone = Arc::clone(&config);
    let api_client_clone = Arc::clone(&api_client);

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

        while workflow_rx.recv().is_ok() {
            let config = Arc::clone(&config_clone);
            let api_client = Arc::clone(&api_client_clone);

            rt.block_on(async {
                match execute_workflow_with_logging(&config, &api_client).await {
                    Ok(()) => {
                        println!("✅ Workflow completed successfully!\n");
                    }
                    Err(e) => {
                        eprintln!("❌ Workflow failed: {}\n", e);
                    }
                }
                println!("Ready for next hotkey press...");
            });
        }
    });

    // Create tao event loop (required for macOS global hotkeys)
    let event_loop = EventLoopBuilder::new().build();

    // Get global hotkey event receiver
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    // Run tao event loop on main thread
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            // Only trigger on key press, not release
            if event.state == HotKeyState::Pressed {
                println!("\n⌨️  Hotkey pressed! Executing workflow...");

                // Send signal to workflow thread
                let _ = workflow_tx.send(());
            }
        }
    });
}
