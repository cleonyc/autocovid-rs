//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use crate::database::User;
use isahc::{AsyncReadResponseExt, Error, HttpClient, Response};
use std::time::Duration;

use serde_json::json;
use time::UtcOffset;

pub struct Runner {
    users: Vec<User>,
    client: isahc::HttpClient,
}
impl Runner {
    pub async fn new() -> anyhow::Result<Self> {
        let users = crate::DATABASE.get().unwrap().get_all_users().await?;
        let client = HttpClient::new()?;
        Ok(Self { users, client })
    }
    pub async fn screen(&self) -> anyhow::Result<()> {
        for i in self.users.clone() {
            let json = json!({
                "Type": "G",
                "IsOther": "False",
                "IsStudent": "1",
                "FirstName": i.first_name,
                "LastName": i.last_name,
                "Email": i.email,
                "State": "NY",
                "Location": i.school_code,
                "Floor": "",
                "Answer1": "0",
                "Answer2": "0",
                "Answer3": "3",
                "ConsentType": "",
            });
            match self
                .client
                .post_async(
                    "https://healthscreening.schools.nyc/home/submit",
                    json.as_str(),
                )
                .await
            {
                Ok(mut r) => {
                    log::debug!("Successfully screened, resp: {}", r.text().await?);
                }
                Err(e) => return Err(anyhow::Error::from(e)),
            }
        }
        Ok(())
    }
    // runs from tokio::spawn in main, otherwise it gets dropped and weird shit happens
    pub async fn event_loop(&self) {
        loop {
            //IMPORTANT: WILL NOT ADJUST FOR DAYLIGHT SAVINGS TIME (IF IT STILL EXISTS)
            if time::OffsetDateTime::now_utc()
                .to_offset(UtcOffset::from_hms(-4, 0, 0).unwrap())
                .hour()
                == 6
            {
                self.screen().await.unwrap();
                tokio::time::sleep(Duration::from_secs(86400)).await;
            }
            // in case my genius code decides to break itself, would rather not have it run on every cycle
            tokio::time::sleep(Duration::new(10, 0)).await;
        }
    }
}
