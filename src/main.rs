mod anthropic;
mod hotkey_manager;

use hotkey_manager::HotkeyManager;
use std::sync::mpsc;

fn main() {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Hotkey Prompt Refiner v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting up...");

    // Verify API key is configured
    match std::env::var("ANTHROPIC_API_KEY") {
        Ok(_) => println!("✓ API key configured"),
        Err(_) => println!("⚠ Warning: ANTHROPIC_API_KEY not set"),
    }

    // Register global hotkey
    let hotkey_manager = match HotkeyManager::register() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Create channel for communication between hotkey callback and async runtime
    let (tx, rx) = mpsc::channel();

    // Spawn async runtime on separate thread
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            println!("Async runtime ready");

            // Listen for hotkey events
            while let Ok(()) = rx.recv() {
                println!("Processing hotkey event...");
                // TODO: Integrate clipboard → API → paste workflow
                // This will be implemented in subsequent tasks
            }
        });
    });

    println!("Ready! Waiting for hotkey...");

    // Run event loop on main thread (required for macOS)
    if let Err(e) = hotkey_manager.run_event_loop(tx) {
        eprintln!("Event loop error: {}", e);
        std::process::exit(1);
    }
}
