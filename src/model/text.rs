pub use proto::text::*;

pub fn handle_create_text(command: CreateText) -> Result<TextCreated, ()> {
    if command.text.is_none() {
        return Err(());
    }

    Ok(TextCreated { text: command.text })
}
