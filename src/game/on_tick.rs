use std::error::Error;

pub async fn on_tick() -> Result<(), Box<dyn Error>> {
    crate::protocol::chat::sync().await?;
    crate::protocol::tab_list::sync().await?;

    Ok(())
}
