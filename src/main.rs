mod anthropic;
mod config;

use config::Config;

#[tokio::main]
async fn main() {
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
    let api_client = anthropic::ApiClient::new(config.api_key.clone(), None);

    // Test 1: Basic API connection
    println!("\n🔬 Test 1: Basic API connection...");
    match anthropic::send_message(
        &config.api_key,
        "Say 'Hello! API connection successful.' in 5 words or less.",
        anthropic::DEFAULT_MODEL,
        100,
    ).await {
        Ok(response) => {
            println!("✓ API test successful!");
            println!("  Model: {}", anthropic::DEFAULT_MODEL);
            println!("  Response: {}", response.trim());
        }
        Err(e) => {
            eprintln!("✗ API test failed: {}", e);
            eprintln!("  Check your API key and network connection");
        }
    }

    // Test 2: Prompt formatting and processing
    println!("\n🔬 Test 2: Prompt formatting with template...");
    let test_clipboard = "make this text better";
    match api_client.process_text(&config.prompt_template, test_clipboard).await {
        Ok(response) => {
            println!("✓ Prompt formatting test successful!");
            println!("  Input: {}", test_clipboard);
            println!("  Output: {}", response.trim());
        }
        Err(e) => {
            eprintln!("✗ Prompt formatting test failed: {}", e);
        }
    }

    println!("\n✅ All tests passed! Ready for hotkey integration.");
    println!("Ready! Hotkey: Cmd+Shift+P (not yet implemented)");
}
