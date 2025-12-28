use std::path::Path;
/// Example demonstrating how to convert text from a file to audio
///
/// Run with: `cargo run --example from_file`
use text2audio::{Model, Text2Audio, Voice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Path to the text file
    let input_path = "assets/text.md";
    let output_path = "output.wav";

    // Check if file exists
    if !Path::new(input_path).exists() {
        eprintln!("Error: File '{}' not found!", input_path);
        eprintln!("Please ensure the file exists before running this example.");
        std::process::exit(1);
    }

    // Read text from file
    println!("Reading text from: {}", input_path);
    let text = std::fs::read_to_string(input_path)?;

    let text = text.trim();
    if text.is_empty() {
        eprintln!("Error: File is empty!");
        std::process::exit(1);
    }

    println!("Text length: {} characters", text.chars().count());
    println!();

    // Create converter with optimized settings for long-form content
    let converter = Text2Audio::new(&api_key)
        .with_model(Model::GLM4_7)
        .with_coding_plan(true)
        .with_voice(Voice::Tongtong) // Use a voice suitable for narration
        .with_speed(1.0) // Normal speed
        .with_volume(1.0) // Normal volume
        .with_max_segment_length(800) // Longer segments for better flow
        .with_parallel(5); // Enable parallel processing

    println!("Converting text to audio...");
    println!("Voice: {}", Voice::Tongtong);
    println!("Max segment length: 800 characters");
    println!("Parallel processing: enabled (max 5 concurrent requests)");
    println!();

    // Convert and save
    let start = std::time::Instant::now();
    converter.convert(text, output_path).await?;
    let duration = start.elapsed();

    println!("✓ Audio saved to: {}", output_path);
    println!("✓ Conversion time: {:.2}s", duration.as_secs_f64());

    Ok(())
}
