use std::io::{self, Read};

fn main() {
    if std::env::args().any(|arg| arg == "--json") {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("stdin should be readable");

        let response = wiki_core::transport::json_rpc::handle_json(&input)
            .unwrap_or_else(|error| wiki_core::transport::dto::CoreResponse::error(error.to_string()));

        println!(
            "{}",
            serde_json::to_string(&response).expect("response should serialize")
        );
        return;
    }

    println!("{}", wiki_core::workspace_name());
}
