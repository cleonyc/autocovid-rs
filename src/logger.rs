//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use async_trait::async_trait;
use poise::serenity_prelude::{ChannelId, Context, GuildChannel, User, UserId};

#[async_trait]
pub trait LogEntry {
    async fn send_log(
        &self,
        channel: GuildChannel,
        ctx: &poise::serenity_prelude::Context,
    ) -> anyhow::Result<()>;
}
pub struct Logger {
    log_channels: Vec<GuildChannel>,
}
impl Logger {
    pub async fn new(ctx: &poise::serenity_prelude::Context) -> Logger {
        let logcs = crate::BOT_CONFIG.clone().owner.logs;
        let mut log_channels = vec![];
        for gc_string in logcs {
            let channel_id =
                ChannelId(gc_string.split_once(":").unwrap().1.parse::<u64>().unwrap());
            let guild_channel = channel_id
                .to_channel(&ctx.http)
                .await
                .unwrap()
                .guild()
                .unwrap();
            // GuildChannel::convert(&ctx, Some(guild_id), Some(channel_id), "")
            // .await
            // .unwrap();
            log_channels.push(guild_channel)
        }
        Logger { log_channels }
    }
    pub async fn push(&self, entry: Box<dyn LogEntry + Send>, ctx: Context) {
        for gc in self.log_channels.clone() {
            let fut = entry.send_log(gc, &ctx);
            fut.await.unwrap();
        }
    }
}

pub struct EditScreening {
    pub user: u64,
    pub email: Option<(String, String)>,
    pub name: Option<(String, String)>,
    pub school: Option<(String, String)>,
}
#[async_trait]
impl LogEntry for EditScreening {
    async fn send_log(
        &self,
        channel: GuildChannel,
        ctx: &poise::serenity_prelude::Context,
    ) -> anyhow::Result<()> {
        let user = UserId::from(self.user).to_user(&ctx).await.unwrap();
        let mut fields = vec![];
        match &self.email {
            Some(t) => fields.push((
                "Email",
                format!("`{} -> {}`", t.0.clone(), t.1.clone()),
                true,
            )),
            _ => {}
        };
        match &self.name {
            Some(t) => fields.push((
                "Name",
                format!("`{} -> {}`", t.0.clone(), t.1.clone()),
                true,
            )),
            _ => {}
        };
        match &self.school {
            Some(t) => {
                fields.push((
                    "School Code",
                    format!("`{} -> {}`", t.0.clone(), t.1.clone()),
                    true,
                ));
            }
            _ => {}
        };
        channel
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Screening Edited!")
                        .footer(|f| {f
                            .text(format!(
                                "{} - {}",
                                self.user,
                                format!("{}#{}", user.name.clone(), user.discriminator)
                            ))
                            // probably not legal but is a neat feature ig
                            .icon_url(user.avatar_url().unwrap_or("https://pngimg.com/uploads/question_mark/question_mark_PNG142.png".to_string()))
                        })
                        .fields(fields)
                })
            })
            .await?;
        Ok(())
    }
}
pub struct NewScreening {
    pub user: u64,
    pub email: String,
    pub name: String,
    pub school: String,
}
#[async_trait]
impl LogEntry for NewScreening {
    async fn send_log(
        &self,
        channel: GuildChannel,
        ctx: &poise::serenity_prelude::Context,
    ) -> anyhow::Result<()> {
        let user = UserId::from(self.user).to_user(&ctx.http).await.unwrap();

        channel
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("New Screening Registered!")
                        .footer(|f| {f
                            .text(format!(
                                "{} - {}",
                                self.user,
                                format!("{}#{}", user.name.clone(), user.discriminator)
                            ))
                            // probably not legal but is a neat feature ig
                            .icon_url(user.avatar_url().unwrap_or("https://pngimg.com/uploads/question_mark/question_mark_PNG142.png".to_string()))
                        })
                        .field("Email", format!("`{}`", &self.email), true)
                        .field("Name", format!("`{}`", &self.name), true)
                        .field("School Code", format!("`{}`", &self.school), true)
                })
            })
            .await?;
        Ok(())
    }
}
