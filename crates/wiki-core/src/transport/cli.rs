use crate::transport::dto::{CoreCommand, CoreResponse};

pub fn dispatch(command: CoreCommand) -> CoreResponse {
    CoreResponse::error(format!("unsupported_action:{}", command.action))
}
