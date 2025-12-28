use text2audio::Model;
/// Example demonstrating parallel processing of audio segments
///
/// Run with: `cargo run --example parallel`
use text2audio::Text2Audio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Very long text to demonstrate parallel processing benefits
    let long_text = "这是一段很长的文本。".repeat(200);

    println!("Converting with parallel processing...");
    println!("Text length: {} characters", long_text.chars().count());
    println!("Max parallel: 5\n");

    // Enable parallel processing for faster conversion
    let converter = Text2Audio::new(&api_key)
        .with_coding_plan(true)
        .with_model(Model::GLM4_7)
        .with_max_segment_length(500)
        .with_parallel(5);

    let start = std::time::Instant::now();
    converter.convert(&long_text, "output_parallel.wav").await?;
    let duration = start.elapsed();

    println!("✓ Audio saved to output_parallel.wav");
    println!("Time taken: {:?}", duration);
    println!("\nNote: Parallel processing can significantly speed up long text conversion.");

    Ok(())
}
