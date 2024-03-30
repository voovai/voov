// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use llama_cpp::standard_sampler::StandardSampler;
use llama_cpp::SessionParams;
use llama_cpp::LlamaParams;
use llama_cpp::LlamaModel;
use llama_cpp::LlamaSession;
use std::io;
use std::io::Write;
// Function to list files in a given directory
use tauri::{Manager, Window};
use std::sync::Mutex;


struct ChatModelSession {
    model: LlamaModel,
    session: Mutex<LlamaSession>, // Adjust this to match the actual type of your session
}

impl ChatModelSession {
    fn new(model_path: &str) -> Self {
        let model = LlamaModel::load_from_file(model_path, LlamaParams::default())
            .expect("Could not load model");
        let session = model.create_session(SessionParams::default())
            .expect("Failed to create session");
        ChatModelSession { model, session: Mutex::new(session) }
    }
}


// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello, {}! You've been greeted from Rust! this is cool", name);
    list_files("/Users/gavi/Downloads")
}

#[tauri::command]
fn chat(message: &str, window: Window, model: tauri::State<'_, ChatModelSession>) {
    let mut session = model.session.lock().expect("Failed to lock session");

    session.advance_context("[INST]hello, who built you?[/INST]").unwrap();

    // LLMs are typically used to predict the next word in a sequence. Let's generate some tokens!
    let max_tokens = 1024;
    let mut decoded_tokens = 0;

    // `ctx.start_completing_with` creates a worker thread that generates tokens. When the completion
    // handle is dropped, tokens stop generating!
    let mut completions = session.start_completing_with(StandardSampler::default(), 1024).into_strings();

    for completion in completions {
        print!("{completion}");
        window.emit("event-name", Payload { message: completion.into() }).unwrap();

        let _ = io::stdout().flush();

        decoded_tokens += 1;

        if decoded_tokens > max_tokens {
            break;
        }
    }
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_files(dir_name: &str) -> String {
    let mut file_names = Vec::new();
    let path = Path::new(dir_name);

    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(e) => {
                            let path = e.path();
                            if path.is_file() {
                                // Add the file name to the vector
                                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                    file_names.push(name.to_owned());
                                }
                            }
                        }
                        Err(err) => println!("Error reading directory: {}", err),
                    }
                }
            }
            Err(err) => println!("Error reading directory: {}", err),
        }
    } else {
        println!("{} is not a directory!", dir_name);
    }

    // Serialize the list of file names into a JSON string
    serde_json::to_string(&file_names).unwrap_or_else(|_| "[]".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(ChatModelSession::new("/Users/gavi/.cache/lm-studio/models/TheBloke/Mixtral-8x7B-Instruct-v0.1-GGUF/mixtral-8x7b-instruct-v0.1.Q5_K_M.gguf"))
        .invoke_handler(tauri::generate_handler![greet,list_files,chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
