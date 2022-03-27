//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use crate::logger::{EditScreening, LogEntry, Logger, NewScreening};
use crate::{Data, DATABASE};
use regex::Regex;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn new_screening(
    ctx: Context<'_>,
    #[description = "Email Address"] email: String,
    #[description = "Name (first and last seperated by space)"] name: String,
    #[description = "School Name (or code)"] s_name: String,
) -> Result<(), Error> {
    if crate::DATABASE
        .get()
        .unwrap()
        .get_user(ctx.author().id.0)
        .await
        .unwrap()
        .is_some()
    {
        ctx.send(|f| {
            f.content("You have already registered a screening, use /editscreening to edit it")
        })
        .await?;
        return Ok(());
    }
    let email_regex = Regex::new("(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$)").unwrap();
    if !email_regex.is_match(&email) {
        ctx.send(|f| f.content("Invalid email! DM the bot if you think this is a bug"))
            .await?;
        log::warn!("Email considered invalid: `{}`", email);
    }
    let name_tuple = match name.split_once(" ") {
        None => {
            ctx.send(|f| f.content("Failed to parse name, try entering it in again as \"first last name\", any spaces in the last name will still be included.")).await?;
            log::warn!("Name considered invalid: `{}`", name);
            return Ok(());
        }
        Some(i) => (i.0.to_string(), i.1.to_string()),
    };
    crate::DATABASE
        .get().unwrap().new_user(
            ctx.author().id.0,
            name_tuple.0,
            name_tuple.1,
            email.clone(),
            match crate::school_code::get_school_code(s_name.as_str()).await {
                Ok(i) => i,
                Err(_) => {
                    ctx.send(|f| f.content("Failed to find school! You can try looking for your school code on https://schoolsearch.schools.nyc/ and entering that in the `name` field instead")).await?;
                    log::warn!("Failed to find school: `{}`", s_name);
                    return Ok(())

                }
            },
        )
        .await?;
    ctx.send(|f| {
        f.content("New screening registered successfully!")
            .ephemeral(true)
    })
    .await?;
    let logger = Logger::new(&ctx.discord()).await;
    logger
        .push(
            Box::new(NewScreening {
                user: ctx.author().id.0,
                email,
                name,
                school: s_name.to_string(),
            }),
            ctx.discord().clone(),
        )
        .await;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn edit_screening(
    ctx: Context<'_>,
    #[description = "Email Address"] email: Option<String>,
    #[description = "Name (first and last seperated by space)"] name: Option<String>,
    #[description = "School Name (or code)"] s_name: Option<String>,
) -> Result<(), Error> {
    let old_user = DATABASE.get().unwrap().get_user(ctx.author().id.0).await?;
    if old_user.is_none() {
        ctx.send(|f| {
            f.content("Please register a screening (/newscreening) before trying to edit it!")
        })
        .await?;
        return Ok(());
    }
    let old_user = old_user.unwrap();
    if email.is_some() {
        let email_regex =
            Regex::new("(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$)").unwrap();
        if !email_regex.is_match(&email.clone().unwrap()) {
            ctx.send(|f| f.content("Invalid email! DM the bot if you think this is a bug"))
                .await?;
            log::warn!("Email considered invalid: `{}`", email.unwrap());
            return Ok(());
        }
    }

    let mut first = None;
    let mut last = None;
    if name.is_some() {
        let name_tuple = match name.clone().unwrap().split_once(" ") {
            None => {
                ctx.send(|f| f.content("Failed to parse name, try entering it in again as \"first last name\", any spaces in the last name will still be included.")).await?;
                log::warn!("Name considered invalid: `{}`", name.unwrap());
                return Ok(());
            }
            Some(i) => (i.0.to_string(), i.1.to_string()),
        };
        first = Some(name_tuple.0);
        last = Some(name_tuple.1)
    }
    let school_code = match s_name {
        None => None,
        Some(_) => Some(
            match crate::school_code::get_school_code(s_name.clone().unwrap().as_str()).await {
                Ok(i) => i,
                Err(_) => {
                    ctx.send(|f| f.content("Failed to find school! You can try looking for your school code on https://schoolsearch.schools.nyc/ and entering that in the `name` field instead")).await?;
                    log::warn!("Failed to find school: `{}`", s_name.unwrap());
                    return Ok(());
                }
            },
        ),
    };

    crate::DATABASE
        .get()
        .unwrap()
        .modify_user(
            ctx.author().id.0,
            first,
            last,
            email.clone(),
            school_code.clone(),
        )
        .await?;
    ctx.send(|f| f.content("Screening edited successfully!"))
        .await?;
    let logger = Logger::new(&ctx.discord()).await;
    let entry = Box::new(EditScreening {
        user: ctx.author().id.0,
        email: match email {
            None => None,
            Some(i) => Some((old_user.email.clone(), i.clone())),
        },
        name: match name {
            None => None,
            Some(i) => Some((
                format!(
                    "{} {}",
                    old_user.first_name.clone(),
                    old_user.last_name.clone()
                ),
                i.clone(),
            )),
        },
        school: match s_name {
            None => None,
            Some(_) => Some((old_user.school_code.clone(), school_code.unwrap().clone())),
        },
    }) as Box<dyn LogEntry + Send>;
    logger.push(entry, ctx.discord().clone()).await;

    Ok(())
}
