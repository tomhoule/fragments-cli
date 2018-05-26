use failure::Error;
pub use fragmentary::text::*;
use uuid::Uuid;

impl ::Command for CreateText {
    fn handle(self, context: &::Context) -> Result<::CommandResponse, Error> {
        unimplemented!();
    }
}

pub fn handle_create_text(command: CreateText, user_id: Option<Uuid>) -> Result<TextCreated, ()> {
    if command.text.is_none() {
        return Err(());
    }

    Ok(TextCreated {
        text: command.text,
        user_id: user_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| String::new()),
    })
}
