// start: -- Modules

use tokio::io::AsyncWriteExt;

use violetta_rs::{
    Result,
    consts::{
        MODEL,
        DEFAULT_SYSTEM_MOCK,
    },
    generation::run_chat_request,
};

use ollama_rs::{
    Ollama,
    generation::chat::{
            ChatMessage,
            MessageRole,
            request::ChatMessageRequest,
        },
};

// end: -- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // By Default localhost::11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();

    let mut stdout = tokio::io::stdout();
    let stdin = std::io::stdin();

    let mut thread_messages: Vec<ChatMessage> = vec![];

    loop {
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;   

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("bye bye") {
            break;
        }

        let system_message =
            ChatMessage::new(MessageRole::System, DEFAULT_SYSTEM_MOCK.to_string());

        thread_messages.push(system_message);

        let input_message = ChatMessage::new(MessageRole::User, input.to_string());

        thread_messages.push(input_message);

        let chat_request =
            ChatMessageRequest::new(model.to_owned(), thread_messages.clone());

        let message_content = run_chat_request(&ollama, chat_request).await?;

        if let Some(content) = message_content {
            let assistant_message = ChatMessage::new(MessageRole::Assistant, content);
            thread_messages.push(assistant_message);
        }
    }

    Ok(())
}