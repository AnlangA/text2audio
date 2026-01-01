/// Example demonstrating how to read text from a file, convert it to audio, and play the audio
///
/// This example reads text from assets/text.md, converts it to audio using the text2audio library,
/// and automatically plays the generated audio using the system's audio player.
///
/// Run with: `cargo run --example play_audio`
use std::path::Path;
use std::process::Command;
use text2audio::{Model, Text2Audio, Voice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Path to the text file and output audio
    let input_path = "assets/text.md";
    let output_path = "output_play.wav";

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

    // Create converter with optimized settings
    let converter = Text2Audio::new(&api_key)
        .with_model(Model::GLM4_7)
        .with_coding_plan(true)
        .with_voice(Voice::Tongtong)
        .with_speed(1.0)
        .with_volume(1.0)
        .with_max_segment_length(800)
        .with_parallel(5);

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
    println!();

    // Play the generated audio
    println!("Playing audio...");
    play_audio(output_path)?;

    println!("✓ Playback complete");

    Ok(())
}

/// Play an audio file using the system's audio player
///
/// This function tries multiple audio players in order:
/// Windows:
/// 1. powershell (Windows Media Player via .NET)
/// 2. ffplay (from FFmpeg)
///
/// Linux:
/// 1. aplay (ALSA - common on Linux)
/// 2. paplay (PulseAudio - common on Linux)
/// 3. ffplay (from FFmpeg)
///
/// macOS:
/// 1. afplay
/// 2. ffplay
fn play_audio(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let is_windows = cfg!(windows);
    let is_macos = cfg!(target_os = "macos");

    if is_windows {
        // Windows-specific players
        println!("Using audio player: Windows Media Player (via PowerShell)");

        // Convert path to Windows format if needed
        let windows_path = if file_path.contains('/') {
            file_path.replace('/', "\\")
        } else {
            file_path.to_string()
        };

        // Use PowerShell to play audio via Windows Media Player
        let powershell_script = format!(
            "$player = New-Object -ComObject WMPlayer.OCX;$player.URL = '{}';$player.controls.play();Start-Sleep -Seconds 1;while($player.playState -eq 3){{Start-Sleep -Seconds 1}}",
            windows_path
        );

        let status = Command::new("powershell")
            .args(&["-Command", &powershell_script])
            .status()?;

        if status.success() {
            return Ok(());
        } else {
            eprintln!("Warning: Windows Media Player exited with status {}", status);
        }
    }

    // Cross-platform players
    let players: Vec<(&str, Vec<&str>)> = if is_macos {
        vec![
            ("afplay", vec![file_path]),
            ("ffplay", vec!["-nodisp", "-autoexit", file_path]),
        ]
    } else {
        vec![
            ("aplay", vec!["-q", file_path]),
            ("paplay", vec![file_path]),
            ("ffplay", vec!["-nodisp", "-autoexit", file_path]),
        ]
    };

    for (player, args) in players {
        if which(player).is_ok() {
            println!("Using audio player: {}", player);
            let status = Command::new(player).args(&args).status()?;

            if status.success() {
                return Ok(());
            } else {
                eprintln!("Warning: {} exited with status {}", player, status);
            }
        }
    }

    let error_msg = if is_windows {
        "No audio player found. Windows Media Player failed. Please install FFmpeg for fallback."
    } else {
        "No audio player found. Please install aplay, paplay, or ffplay"
    };

    Err(error_msg.into())
}

/// Check if a command exists in PATH
fn which(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("which")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Command '{}' not found", command).into())
    }
}
