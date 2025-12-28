/// Example demonstrating AI-powered text splitting
///
/// Run with: `cargo run --example ai_splitter`
use text2audio::{Model, Text2Audio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("ZHIPU_API_KEY").expect("Please set ZHIPU_API_KEY environment variable");

    // Long text that benefits from AI semantic splitting
    let long_text = r#"
        人工智能技术的发展日新月异。机器学习算法在各个领域都展现出了强大的能力，从图像识别到自然语言处理，从自动驾驶到医疗诊断。深度学习作为机器学习的一个重要分支，通过构建多层神经网络来模拟人脑的学习过程，已经在许多任务上取得了超越人类的表现。

        自然语言处理是人工智能领域最具挑战性的任务之一。语言是人类交流的基础，包含了丰富的语义、语法和上下文信息。近年来，基于Transformer架构的大型语言模型如GPT、BERT等，在文本生成、机器翻译、问答系统等任务上取得了突破性进展。这些模型通过在海量文本数据上进行预训练，学会了语言的统计规律和世界知识，能够生成连贯、有逻辑的文本。

        计算机视觉是另一个人工智能的重要分支。它致力于让计算机能够"看懂"图像和视频。卷积神经网络（CNN）是计算机视觉的核心技术，通过局部感知、权值共享和池化等机制，有效地提取图像的特征。从图像分类到目标检测，从人脸识别到图像生成，计算机视觉技术已经广泛应用于安防监控、自动驾驶、医疗影像分析等领域。

        强化学习是一种通过与环境交互来学习最优策略的机器学习方法。智能体通过观察环境状态、执行动作并接收奖励或惩罚，不断调整自己的行为策略以最大化累积奖励。AlphaGo是强化学习的典型应用，它通过自我对弈学习了围棋的策略，最终战胜了世界冠军。强化学习在机器人控制、游戏博弈、推荐系统等领域都有广泛应用。

        人工智能的未来充满机遇和挑战。一方面，随着计算能力的提升和算法的改进，AI系统将在更多领域发挥重要作用；另一方面，我们也需要关注AI的伦理问题、隐私保护、可解释性等议题。只有负责任地发展人工智能，才能让这项技术更好地造福人类社会。
    "#;

    println!("Converting with AI-powered splitting...");
    println!(
        "Text length: {} characters",
        long_text.trim().chars().count()
    );
    println!("Model: GLM-4.7 (best for semantic analysis)\n");

    // Use AI splitter for semantic segmentation
    let converter = Text2Audio::new(&api_key)
        .with_model(Model::GLM4_7)
        .with_max_segment_length(300);

    converter
        .convert(long_text.trim(), "output_ai_split.wav")
        .await?;

    println!("✓ Audio saved to output_ai_split.wav");
    println!("\nNote: AI splitting ensures semantic coherence by splitting at natural boundaries.");

    Ok(())
}
