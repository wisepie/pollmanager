use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Dinkdonk {
    title: String,
    options: String,
    #[serde(rename = "type")]
    poll_type: String,
    duration: u8,
    randomize: bool,
    broadcast: bool,
    #[serde(rename = "apiKey")]
    api_key: String,
}

impl Dinkdonk {
    pub async fn create_rating_poll(
        title: String,
        api_key: String,
        randomize: bool,
        broadcast: bool,
    ) -> Result<String, reqwest::Error> {
        let mut poll_options: Vec<String> = Vec::new();
        for i in 1..11 {
            let val: f32 = (i as f32) / 2.0;
            poll_options.push(format!("{}/5", val))
        }
        poll_options.push("Didn't Watch".to_owned());
        let payload = Dinkdonk {
            title,
            options: serde_json::to_string(&poll_options).unwrap(),
            poll_type: String::from("single_choice"),
            duration: 8,
            randomize,
            broadcast,
            api_key,
        };
        let client = Client::new();
        let res = client
            .post("https://dinkdonk.mov/api/create")
            .form(&payload)
            .send()
            .await?;

        println!("{}", res.status());
        let data_json: Value = res.json().await?;

        match data_json
            .get("data")
            .and_then(|data| data.get("id"))
            .and_then(|id| id.as_str())
        {
            Some(id) => Ok(format!("https://dinkdonk.mov/poll/{}", id)),
            None => Ok("Error Fetching Poll ID".to_owned()),
        }
    }
}
