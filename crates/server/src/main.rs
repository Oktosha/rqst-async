use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Chat {
    messages: Vec<String>,
}

use miniserve::{
    http::{self},
    Content, Request, Response,
};

fn index(_req: Request) -> Response {
    let content = include_str!("../index.html").to_string();
    Ok(Content::Html(content))
}

fn chat(req: Request) -> Response {
    if let Request::Post(s) = req {
        let mut parse_result: Result<Chat, _> = serde_json::from_str(&s);
        match &mut parse_result {
            Ok(chat) => {
                chat.messages
                    .push(String::from("And how does it make you feel?"));
                let serialize_result = serde_json::to_string(chat);
                match &serialize_result {
                    Ok(s) => Ok(Content::Json(s.to_string())),
                    Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            Err(_) => Err(http::StatusCode::BAD_REQUEST),
        }
    } else {
        Err(http::StatusCode::BAD_REQUEST)
    }
}

fn main() {
    miniserve::Server::new()
        .route("/", index)
        .route("/chat", chat)
        .run()
}
