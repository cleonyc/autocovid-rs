//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html
use isahc::AsyncReadResponseExt;
use serde_json::Value;

pub async fn get_school_code(name: &str) -> anyhow::Result<String> {
    let url = format!(
        "https://ws.schools.nyc/schooldata/GetSchools?search={}&borough=&grade=",
        name.replace(" ", "+")
    );
    let mut resp = isahc::get_async(url).await?;
    let js: Value = resp.json().await?;
    let code = js[0]["locationCode"].clone();
    Ok(code.as_str().unwrap().parse()?)
}
