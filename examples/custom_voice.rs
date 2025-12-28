/// Example demonstrating custom voice and configuration options
///
/// Run with: `cargo run --example custom_voice`
use text2audio::{Text2Audio, Voice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    let text = "这是自定义配置的语音合成示例。您可以调整语速、音量和音色。";

    // Test different voices
    let voices = vec![
        Voice::Tongtong,
        Voice::Chuichui,
        Voice::Xiaochen,
        Voice::Jam,
        Voice::Kazi,
        Voice::Douji,
        Voice::Luodo,
    ];

    println!("Testing different voices...\n");

    for voice in voices {
        println!("Voice: {}", voice);

        let converter = Text2Audio::new(&api_key)
            .with_voice(voice)
            .with_speed(1.0)
            .with_volume(1.0);

        let output_file = format!("output_voice_{}.wav", voice);
        match converter.convert(text, &output_file).await {
            Ok(_) => println!("  ✓ Saved to {}", output_file),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }

    // Test different speeds
    println!("\nTesting different speeds...\n");

    let speeds = vec![0.5, 1.0, 1.5, 2.0];

    for speed in speeds {
        println!("Speed: {}", speed);

        let converter = Text2Audio::new(&api_key)
            .with_voice(Voice::Tongtong)
            .with_speed(speed)
            .with_volume(1.0);

        let output_file = format!("output_speed_{}.wav", speed);
        match converter.convert(text, &output_file).await {
            Ok(_) => println!("  ✓ Saved to {}", output_file),
            Err(e) => println!("  ✗ Error: {}", e),
        }
    }

    println!("\n✓ All examples completed!");

    Ok(())
}
