/// Example demonstrating long text processing with automatic segmentation
///
/// Run with: `cargo run --example long_text`
use text2audio::{Text2Audio, Voice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Create converter with custom segment length and voice
    let converter = Text2Audio::new(&api_key)
        .with_max_segment_length(500)
        .with_voice(Voice::Tongtong)
        .with_speed(1.0)
        .with_volume(1.0);

    // Long text that will be split into segments
    let long_text = r#"
        这是一段很长的文本，用于演示文本转音频的自动分段功能。
        当文本长度超过 API 的限制时，系统会自动将其分成多个较小的段落。
        每个段落都会单独进行语音合成，然后再合并成一个完整的音频文件。

        第一段：文本分段是处理长文本的重要技术。
        按照句子的自然边界进行分割，可以确保每个段落的语义完整性。
        这样的分段方式能够产生更加自然的语音效果。

        第二段：我们使用了多种标点符号作为分段标记。
        包括中文的句号、感叹号、问号，以及英文的句点、感叹号和问号。
        这样可以适应中英文混合的文本内容。

        第三段：音频合并功能确保了多段音频的无缝拼接。
        每段音频都使用相同的采样率和格式，保证音质的一致性。
        最终生成的音频文件听起来就像是一段完整的语音。

        第四段：在实际应用中，这种技术可以用于将长篇文章转换为音频。
        比如将博客文章、新闻内容或小说章节转换为有声读物。
        这大大扩展了文本内容的传播方式和适用场景。

        第五段：我们的库支持多种音色选择，包括通通、吹吹、小辰等。
        还可以调整语速和音量，满足不同的使用需求。
        比如快节奏的内容可以使用较快的语速。

        第六段：错误处理机制确保了系统的稳定性。
        如果某一段转换失败，系统会自动重试，最多重试三次。
        这样可以应对网络波动等临时性问题。

        第七段：整个转换过程是异步进行的，不会阻塞主线程。
        这使得程序可以在等待网络请求的同时处理其他任务。
        提高了程序的响应性和资源利用率。
    "#;

    println!("Converting long text to audio...");
    println!(
        "Text length: {} characters",
        long_text.trim().chars().count()
    );
    println!("Segment length: 500 characters");
    println!("Voice: {}", Voice::Tongtong);

    converter
        .convert(long_text.trim(), "output_long.wav")
        .await?;

    println!("✓ Audio saved to output_long.wav");

    Ok(())
}
