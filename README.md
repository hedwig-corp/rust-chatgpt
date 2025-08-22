# ChatGPT API Rust Library

![Crates.io](https://img.shields.io/crates/v/rust-chatgpt?style=flat-square)
![GitHub](https://img.shields.io/github/license/hedwig-corp/rust-chatgpt?style=flat-square)


## Overview

This Rust library provides a simple and efficient way to interact with the ChatGPT API, which is a state-of-the-art NLP
platform that can generate human-like responses to text queries. The library provides a convenient interface for sending
requests and receiving responses from the ChatGPT API, allowing developers to easily integrate the API into their
Rust-based projects.

## Features

- Easy-to-use API for sending requests and receiving responses.
- Provides responses in multiple formats, including text and JSON.
- Supports multiple endpoints and response languages.

  |API|Support|
  |---|---|
  |Models|✔️|
  |Completions|✔️|
  |Chat|✔️|
  |Edits|✔️|
  |Images|✔️|
  |Embeddings|✔️|
  |Audio|✔️|
___


## Getting Started
To get started, you will need an API key from OpenAI. You can obtain an API key by visiting the [OpenAI API page](https://platform.openai.com/docs/api-reference/authentication) and following the instructions there.

Once you have an API key, you can install the library using Cargo:



## Installation

To use this library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-chatgpt = "0.1"
```

Then, add the following to your Rust code:

```rust
use rust_chatgpt::*;
let chatgpt = ChatGpt::new("YOUR_API_KEY_HERE");
let request = ChatGptRequestChatCompletions::new(
    "gpt-3.5-turbo",
    vec![
        ChatGptChatFormat::new_system("Rust OSS開発者"),
        ChatGptChatFormat::new_user("ChatGPT API のRustライブラリを作ったのでエンジニアが好みそうなReadmeを作って欲しい。"),
    ]
);

let res = chatgpt.chat_completions(&request).await.unwrap();
println!("{:?}", response);
```

You can replace `"YOUR_API_KEY_HERE"` with your actual API key, which can be obtained from the ChatGPT API website.

## Usage

### Creating a New ChatGPT Object

To use the ChatGPT API Rust library, you first need to create a new `ChatGPT` object. You can do this using the
following code:

```rust
use rust_chatgpt::*;
let chatgpt = ChatGpt::new("YOUR_API_KEY_HERE");
```

Replace `"YOUR_API_KEY_HERE"` with your actual API key.

#### Using a Custom Base URL

If you need to use a different OpenAI-compatible endpoint (e.g., Azure OpenAI Service, custom proxy, or local deployment), you can specify a custom base URL:

```rust
use rust_chatgpt::*;

// With custom base URL
let chatgpt = ChatGpt::new_with_base_url("YOUR_API_KEY_HERE", "https://your-custom-endpoint.com");

// With organization ID and custom base URL
let chatgpt = ChatGpt::new_org_with_base_url(
    "YOUR_API_KEY_HERE".to_string(),
    "YOUR_ORG_ID".to_string(),
    "https://your-custom-endpoint.com".to_string()
);
```

The library will automatically append the appropriate OpenAI API paths (e.g., `/v1/chat/completions`) to your base URL.

### Models List
Here is an example of how to use the models_list method to retrieve a list of all available models:

```rust
let models = chatgpt.models_list().await.unwrap();
let value = models.to_value();
```

### Models Retrieve

```rust
let model = .models_retrieve("text-davinci-003").await;
let value = model.to_value();
```

### Chatting
Here is an example of how to use the library to chat with the ChatGPT API:

```rust
let request = ChatGptRequestChatCompletions::new(
    "gpt-3.5-turbo",
    vec![
        ChatGptChatFormat::new_system("Rust OSS開発者"),
        ChatGptChatFormat::new_user("ChatGPT API のRustライブラリを作ったのでエンジニアが好みそうなReadmeを作って欲しい。"),
    ]
);

let res = chatgpt.chat_completions(&request).await.unwrap();
println!("{:?}", response.to_value());
```



## Development

### Building and Testing

This project includes a comprehensive Makefile for development tasks:

```bash
# Build the project
make build

# Run tests (requires Ollama for full test suite)
make test

# Run code formatting and linting
make fmt
make clippy

# Run full CI pipeline
make ci

# Install and setup Ollama for local testing
make install-ollama

# Run tests specifically with Ollama
make test-ollama
```

### Ollama Support

This library supports local testing with [Ollama](https://ollama.ai), which allows you to run LLM models locally without requiring OpenAI API keys.

To test with Ollama:

1. Install Ollama: `make install-ollama` or visit [https://ollama.ai](https://ollama.ai)
2. Start Ollama: `ollama serve`
3. Pull a model: `ollama pull llama3.2:latest`
4. Run tests: `make test-ollama`

The library will automatically detect Ollama running on `http://localhost:11434` and use it for testing.

## Contributing

Pull requests are welcome! If you have any questions or issues, please open an issue on the [GitHub repository](https://github.com/hedwig-corp/rust-chatgpt).

## License

This library is licensed under the Apache-2.0 License. See the LICENSE file for details.