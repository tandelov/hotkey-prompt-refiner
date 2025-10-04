mod anthropic;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Hotkey Prompt Refiner v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting up...");

    // Verify API key is configured
    match std::env::var("ANTHROPIC_API_KEY") {
        Ok(_) => println!("✓ API key configured"),
        Err(_) => println!("⚠ Warning: ANTHROPIC_API_KEY not set"),
    }

    println!("Ready!");
}
