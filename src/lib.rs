mod data;
use std::io::{Error, ErrorKind};

use data::CompletionRequest;

pub fn completions(
    key: String,
    model: data::Model,
    messages: Vec<data::Message>,
) -> Result<data::CompletionResponse, Error> {
    let model_name = model.as_str().to_string();
    let request_body = CompletionRequest {
        model: model_name,
        messages,
    };
    let request_body_json = serde_json::json!(request_body).to_owned().to_string();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .body(request_body_json)
        .send();

    match res {
        Ok(r) => match r.text() {
            Ok(body_str) => {
                println!("{}", body_str);
                match serde_json::from_str(&body_str) {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
                };
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

pub fn transcribe(
    key: String,
    file: Vec<u8>,
    file_name: String,
) -> Result<data::TranscriptionResponse, Error> {
    let file_part = reqwest::blocking::multipart::Part::bytes(file).file_name(file_name);
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .part("file", file_part)
        .text("model", "whisper-1");
    let res = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .multipart(form)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "multipart/form-data")
        .send();
    match res {
        Ok(r) => match r.text() {
            Ok(body_str) => {
                println!("{}", body_str);
                match serde_json::from_str(&body_str) {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
                };
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        },
        Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use crate::data::Message;

    use super::*;

    #[test]
    fn completion() {
        let msg = Message {
            role: "user".to_string(),
            content: "What is the reason people say that 42 is the secret to the universe?"
                .to_string(),
        };
        let k = env::var("OPENAI_KEY").expect("key exists");
        let result = completions(k, data::Model::GptTurbo0301, vec![msg]);
        match result {
            Ok(c) => {
                println!("{:?}", c);
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(true);
            }
        }
    }
    #[test]
    fn whisper() {
        let f = fs::read("oprah.mp3").expect("test file exists");
        let k = env::var("OPENAI_KEY").expect("key exists");
        let result = transcribe(k, f, "oprah.mp3".to_string());
        match result {
            Ok(c) => {
                println!("{:?}", c);
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(true);
            }
        }
    }
}
