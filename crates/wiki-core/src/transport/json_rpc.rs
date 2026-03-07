use crate::transport::dto::{CoreCommand, CoreResponse};

pub fn handle(command: CoreCommand) -> CoreResponse {
    crate::transport::cli::dispatch(command)
}

pub fn handle_json(input: &str) -> Result<CoreResponse, serde_json::Error> {
    let command: CoreCommand = serde_json::from_str(input)?;
    Ok(handle(command))
}
