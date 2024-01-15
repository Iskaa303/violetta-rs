// start: -- Modules

use futures::StreamExt;
use tokio::io::AsyncWriteExt;

use violetta_rs::{
    Result,
    consts::{
        MODEL,
        DEFAULT_SYSTEM_MOCK,
    },
};

use ollama_rs::{
    Ollama,
    generation::chat::{
        ChatMessage,
        ChatMessageResponseStream,
        request::ChatMessageRequest,
    }
};

// end: -- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // By Default localhost::11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();

    let mut stdout = tokio::io::stdout();
    let stdin = std::io::stdin();

    let mut messages: Vec<ChatMessage> = vec![];

    let system_message =
        ChatMessage::assistant(DEFAULT_SYSTEM_MOCK.to_string());
    messages.push(system_message);

    loop {
        stdout.write_all(b"\n\nUser:\n").await?;
        stdout.flush().await?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let user_message = ChatMessage::user(input.to_string());
        messages.push(user_message);

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("bye bye") {
            let mut stream: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                model.to_owned(),
                messages.clone(),
            ))
            .await?;
        
            stdout.write_all(b"\nVioletta:").await?;
            stdout.flush().await?;

            let mut response = String::new();
            while let Some(Ok(res)) = stream.next().await {
                if let Some(assistant_message) = res.message {
                    stdout
                        .write_all(assistant_message.content.as_bytes())
                        .await?;
                    stdout.flush().await?;
                    response += assistant_message.content.as_str();
                }
            }
            messages.push(ChatMessage::assistant(response));

            stdout.write_all(b"\n").await?;
            stdout.flush().await?;

            break;
        }

        let mut stream: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                model.to_owned(),
                messages.clone(),
            ))
            .await?;
        
        
        stdout.write_all(b"\nVioletta:").await?;
        stdout.flush().await?;

        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                stdout
                    .write_all(assistant_message.content.as_bytes())
                    .await?;
                stdout.flush().await?;
                response += assistant_message.content.as_str();
            }
        }
        messages.push(ChatMessage::assistant(response));
    }

    Ok(())
}