use crate::{Data, Runner, BOT_CONFIG, DATABASE};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, hide_in_help)]
pub(crate) async fn force_screen(ctx: Context<'_>) -> Result<(), Error> {
    if !BOT_CONFIG.owner.ids.contains(&ctx.author().id.0) {
        return Ok(());
    }
    Runner::new().await?.screen().await?;
    ctx.send(|f| f.content("sent screening to all users").ephemeral(true))
        .await?;
    Ok(())
}
