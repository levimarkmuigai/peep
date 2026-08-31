use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
    stream: bool,
}

struct Credentials {
    base_url: String,
    api_key: String,
}

impl Credentials {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = std::env::var("BASE_URL").map_err(|_| {
            "BASE_URL is not set.\n\
                Add it to your enviroment (e.g in ~/.bashrc)"
        })?;

        let api_key = std::env::var("API_KEY").map_err(|_| {
            "API_KEY is not set.\n\
            Add it to your enviroment (e.g in ~/.bashrc)"
        })?;

        Ok(Self { base_url, api_key })
    }
}

pub fn send_request(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    //let instructions = fs::read_to_string("INSTRUCTIONS.md")?;

    let instructions = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/INSTRUCTIONS.md"));

    let request_body = RequestBody {
        model: "openai/gpt-oss-120b".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: instructions.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.15,
        top_p: 0.9,
        max_tokens: 1000,
        stream: false,
    };

    let client = Client::new();

    let credentials = Credentials::new()?;

    let res = client
        .post(&credentials.base_url)
        .bearer_auth(&credentials.api_key)
        .json(&request_body)
        .send()?
        .error_for_status()?;

    let json: serde_json::Value = res.json()?;

    let response = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("no content found");

    Ok(response.to_string())
}
