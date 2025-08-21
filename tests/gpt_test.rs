use chatgpt::{
    ChatGpt, ChatGptChatFormat, ChatGptRequest, ChatGptRequestAudioTranslations,
    ChatGptRequestChatCompletions, ChatGptRequestCompletionsCreate, ChatGptRequestEdits,
    ChatGptRequestEmbeddingsGenerations, ChatGptRequestImagesVariation, ChatGptResponse,
};
use serde_json::json;

fn get_client() -> Option<ChatGpt> {
    // Use Ollama local API - no API key needed for local instance
    let api_key = std::env::var("OLLAMA_API_KEY").unwrap_or_else(|_| "ollama".to_string());
    let base_url =
        std::env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
    Some(ChatGpt::new_with_base_url(&api_key, &base_url))
}

#[tokio::test]
async fn test_model() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    let a = gpt.models_list().await.unwrap();
    a.to_value();
    // let a = gpt.completions_create("text-davinci-003").await;
    a.to_value();
    dbg!(a);
    // let a = gpt.chat_completions("Hello, my name is", 10, "").await;
    // println!("{:?}",a);
    assert!(true);
}

#[tokio::test]
#[ignore] // Ollama only supports chat completions, not text completions
async fn test_completions() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    let request =
        ChatGptRequestCompletionsCreate::new("llama3.2:latest", 100, "Say this is a test");
    let res = gpt.completions_create(&request).await;
    assert!(res.is_ok());
    let request = ChatGptRequestCompletionsCreate::from_value(json!({
            "model": "llama3.2:latest",
        "prompt": ["Say this is a test"],
        "max_tokens": 7,
        "temperature": 0.0

    }))
    .unwrap();
    let res = gpt.completions_create(&request).await;
    assert!(res.is_ok());
    let _ = dbg!(res);
}

#[tokio::test]
async fn chat() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    let request = ChatGptRequestChatCompletions::new(
        "llama3.2:latest",
        vec![
            ChatGptChatFormat::new_system("Rust OSS開発者"),
            ChatGptChatFormat::new_user("ChatGPT API のRustライブラリを作ったのでエンジニアが好みそうなReadmeを作って欲しい。言語は英語で"),
        ],
    );
    dbg!(request.clone().to_value());

    let res = gpt.chat_completions(&request).await;
    dbg!(res.unwrap().to_value());
}

#[tokio::test]
#[ignore] // Ollama doesn't support edit endpoint
async fn edit() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    let request = ChatGptRequestEdits::new(
        "llama2",
        "Fix the spelling mistakes",
        " day of the wek is it?",
    );
    dbg!(request.clone().to_value());

    let res = gpt.edits(&request).await;
    let _ = dbg!(res);
}

#[tokio::test]
#[ignore] // Ollama doesn't support image generation
async fn image() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    // let mut request = ChatGptRequestImagesGenerations::new("Japan Home. Wood Picture.", 1);
    // dbg!(request.clone().to_value());
    // let res = gpt.images_generations(&request).await;
    // assert!(res.is_ok(), "error: {:?}", res);
    // let res = res.unwrap();
    // println!("{:?}", res);
    // let urls = res.get_urls();
    // for url in urls {
    //     println!("{}", url);
    // }
    // let mut request = ChatGptRequestImagesGenerations::new("Japan Home. Wood Picture.", 1);
    // request.set_response_format("b64_json");
    // let res = gpt.images_generations(&request).await;
    // assert!(res.is_ok(), "error: {:?}", res);
    // let res = res.unwrap();
    // let rows = res.b64_jsons();
    // assert_ne!(rows.len(), 0);

    let request = ChatGptRequestImagesVariation::new(test_png().as_str(), 1);
    let res = gpt.images_variations(&request).await;
    assert!(res.is_ok(), "error: {:?}", res);
    println!("{:?}", res);
}

#[tokio::test]
async fn embeddings() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    let request = ChatGptRequestEmbeddingsGenerations::new(
        "llama3.2:latest",
        "The food was delicious and the waiter...",
    );
    dbg!(request.clone().to_value());

    let res = gpt.embeddings(&request).await;
    let _ = dbg!(res);
}

#[tokio::test]
#[ignore] // Ollama doesn't support audio transcription
async fn audio() {
    let Some(gpt) = get_client() else {
        println!("Skipping test: Ollama client not available");
        return;
    };
    // let request = ChatGptRequestAudioTranscriptions::new(
    //     "whisper-1",
    //     test_mp3().as_str());
    // let res = gpt.audio_transcriptions(&request).await;
    let request = ChatGptRequestAudioTranslations::new("llama2", test_mp3().as_str());
    let res = gpt.audio_translations(&request).await;
    let _ = dbg!(res);
}

fn test_mp3() -> String {
    "./001-sibutomo.mp3".to_string()
}

fn test_png() -> String {
    "./img-aefyVvkbJ2nVn7V9nl5OVm4m.png".to_string()
}
