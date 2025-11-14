use tauri_plugin_sql;
use tauri_plugin_opener;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Clone)]
struct ChatMessagePayload {
    role: String,
    content: String,
}

#[derive(Deserialize, Clone)]
struct ImageAttachment {
    id: String,
    name: String,
    size: u64,
    url: String,
}

#[derive(Deserialize, Clone)]
struct FileAttachment {
    id: String,
    name: String,
    size: u64,
    r#type: String,
}

#[derive(Deserialize, Clone)]
struct Attachments {
    images: Vec<ImageAttachment>,
    files: Vec<FileAttachment>,
    hasAudio: bool,
}

#[derive(Deserialize, Clone)]
struct ChatPayload {
    provider: String,
    model: String,
    apiKey: String,
    systemPrompt: String,
    message: String,
    messages: Option<Vec<ChatMessagePayload>>,
    attachments: Attachments,
}

#[derive(Serialize)]
struct ChatResponse {
    text: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn chat_completion(payload: ChatPayload) -> Result<ChatResponse, String> {
    match payload.provider.as_str() {
        "openai" => call_openai(payload).await,
        _ => Err("Provider not supported".into()),
    }
}

// ----------- OPENAI -----------

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: Value,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
}

#[derive(Deserialize)]
struct OpenAIMessageResp {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessageResp,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

async fn call_openai(payload: ChatPayload) -> Result<ChatResponse, String> {
    let mut messages_vec: Vec<OpenAIMessage> = Vec::new();

    // 1) System prompt (se tiver)
    let system_prompt = payload.systemPrompt.clone();
    if !system_prompt.trim().is_empty() {
        messages_vec.push(OpenAIMessage {
            role: "system".to_string(),
            content: Value::String(system_prompt),
        });
    }

    // helper para montar a mensagem do usuário atual (com ou sem imagem)
    let push_current_user_message = |messages_vec: &mut Vec<OpenAIMessage>| {
        if payload.attachments.images.is_empty() {
            messages_vec.push(OpenAIMessage {
                role: "user".to_string(),
                content: Value::String(payload.message.clone()),
            });
        } else {
            let mut parts = vec![json!({
                "type": "text",
                "text": payload.message.clone()
            })];

            for img in &payload.attachments.images {
                parts.push(json!({
                    "type": "image_url",
                    "image_url": { "url": img.url }
                }));
            }

            messages_vec.push(OpenAIMessage {
                role: "user".to_string(),
                content: Value::Array(parts),
            });
        }
    };

    // 2) Histórico + mensagem atual
    if let Some(history) = payload.messages.clone() {
        if history.is_empty() {
            // histórico existe, mas está vazio -> trata como "sem histórico"
            push_current_user_message(&mut messages_vec);
        } else {
            let last_index = history.len() - 1;
            for (i, m) in history.into_iter().enumerate() {
                // última mensagem do usuário + imagens -> multimodal
                if i == last_index && m.role == "user" && !payload.attachments.images.is_empty() {
                    let mut parts = vec![json!({
                        "type": "text",
                        "text": payload.message.clone()
                    })];

                    for img in &payload.attachments.images {
                        parts.push(json!({
                            "type": "image_url",
                            "image_url": { "url": img.url }
                        }));
                    }

                    messages_vec.push(OpenAIMessage {
                        role: "user".to_string(),
                        content: Value::Array(parts),
                    });
                } else {
                    // demais mensagens do histórico vão como texto simples
                    messages_vec.push(OpenAIMessage {
                        role: m.role,
                        content: Value::String(m.content),
                    });
                }
            }
        }
    } else {
        // nenhum histórico enviado -> só a mensagem atual
        push_current_user_message(&mut messages_vec);
    }

    // 3) Garantia extra: nunca enviar array vazio
    if messages_vec.is_empty() {
        push_current_user_message(&mut messages_vec);
    }

    let body = OpenAIRequest {
        model: payload.model.clone(),
        messages: messages_vec,
    };

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&payload.apiKey)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let raw = res.text().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        // aqui você vê o JSON de erro inteiro no console do front, como já estava
        return Err(format!("OpenAI error {}: {}", status, raw));
    }

    let data: OpenAIResponse = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let first = data.choices.get(0).ok_or("Empty OpenAI response")?;

    Ok(ChatResponse {
        text: first.message.content.clone(),
    })
}

// ----------- TAURI BOOTSTRAP -----------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .build()
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            chat_completion
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
