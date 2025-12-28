/// Simple example demonstrating basic text-to-audio conversion
///
/// Run with: `cargo run --example simple`
use text2audio::Text2Audio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Create converter with default settings
    let converter = Text2Audio::new(&api_key);

    // Short text to convert
    let text = "你好，世界！这是一个简单的文本转音频示例。text2audio 库使用ai分段与ai转音频，可以将任何文本转换为语音。";

    println!("Converting text to audio...");
    println!("Text: {}", text);
    println!("Text length: {} characters", text.chars().count());

    // Convert and save
    converter.convert(text, "output_simple.wav").await?;

    println!("✓ Audio saved to output_simple.wav");

    Ok(())
}
