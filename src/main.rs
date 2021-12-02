use crate::reddit_objects::RepliesUnion;
use chrono::{Local, Utc};
use reddit_objects::{NewestJson, PostData, RepliesClass};
use roux::Reddit;
use rust_bert::{
    bart::{BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources},
    pipelines::summarization::{SummarizationConfig, SummarizationModel},
    resources::{RemoteResource, Resource},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, thread::spawn, time::Instant};
use substring::Substring;
use tch::Device;

mod reddit_objects;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (Linux x86_64)"
);

#[derive(Serialize, Deserialize, Debug)]
struct LogData {
    comments: Vec<String>,
    title: String,
    description: String,
    unix_date: String,
    timestamp: String,
    timings: HashMap<String, String>,
}

struct LogDataBuilder {
    comments: Option<Vec<String>>,
    title: Option<String>,
    description: Option<String>,
    unix_date: String,
    timestamp: String,
    timings: HashMap<String, String>,
}

impl LogDataBuilder {
    pub fn new() -> Self {
        let now = Utc::now();
        let ts: String = now.timestamp().to_string();
        LogDataBuilder {
            comments: None,
            title: None,
            description: None,
            unix_date: ts,
            timestamp: chrono::offset::Local::now().to_rfc3339(),
            timings: HashMap::new(),
        }
    }
    pub fn add_timing<T: Into<String>, V: Into<String>>(&mut self, name: T, value: V) -> &mut Self {
        self.timings.insert(name.into(), value.into());
        self
    }
    pub fn set_comments(&mut self, comments: Vec<String>) -> &mut Self {
        self.comments = Some(comments);
        self
    }
    pub fn set_data<T: Into<String>, V: Into<String>>(&mut self, title: T, desc: V) -> &mut Self {
        self.title = Some(title.into());
        self.description = Some(desc.into());
        self
    }
    pub fn log(self) {
        let title = self.title.unwrap();
        let date = Local::now().format("%F-%H");
        let file_name = format!(
            "{}-{}.json",
            date.to_string(),
            &title.replace(" ", "_").replace(".", "").replace(",", "")
        );

        let log = LogData {
            comments: self.comments.unwrap(),
            title: title,
            description: self.description.unwrap(),
            unix_date: self.unix_date,
            timings: self.timings,
            timestamp: self.timestamp,
        };
        dbg!(&log);
        let data = serde_json::to_string(&log).unwrap();
        let log_dir = format!("{}/.bottalks/logs/", home::home_dir().unwrap().display());
        std::fs::create_dir_all(&log_dir).unwrap();
        std::fs::write(format!("{}/{}", log_dir, file_name), data).unwrap();
    }
}

pub fn gen_config(min: i64, max: i64) -> SummarizationConfig {
    let config_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartConfigResources::DISTILBART_CNN_6_6,
    ));
    let vocab_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartVocabResources::DISTILBART_CNN_6_6,
    ));
    let merges_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartMergesResources::DISTILBART_CNN_6_6,
    ));
    let model_resource = Resource::Remote(RemoteResource::from_pretrained(
        BartModelResources::DISTILBART_CNN_6_6,
    ));

    SummarizationConfig {
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 1,
        length_penalty: 1.0,
        min_length: min,
        max_length: max,
        device: Device::Cpu,
        ..Default::default()
    }
}

pub fn sum(text: String, min: i64, max: i64) -> anyhow::Result<Vec<String>> {
    let summarization_config = gen_config(min, max);
    let summarization_model = SummarizationModel::new(summarization_config)?;
    let input = [text.as_str()];
    Ok(summarization_model.summarize(&input))
}

pub fn summarize(
    text: (String, String),
    length: ((i64, i64), (i64, i64)),
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let _output_1 = sum(text.0, length.0 .0, length.0 .1)?;
    let _output_2 = sum(text.1, length.1 .0, length.1 .1)?;

    Ok((_output_1, _output_2))
}

async fn do_req<T: DeserializeOwned>(client: &reqwest::Client, url: String) -> anyhow::Result<T> {
    let r = client.get(url).send().await.unwrap();
    if r.status() == 200 {
        let text = r.text().await.unwrap();
        let r = serde_json::from_str::<T>(text.as_str());
        if let Ok(json) = r {
            return Ok(json);
        } else {
            println!("ERROR::: {}", r.err().unwrap())
        }
    } else {
        let text = r.text().await.unwrap();
        println!("TEXT: {}", text);
    }

    Err(anyhow::anyhow!("FUCK"))
}

fn read_things(reply: &RepliesClass) -> Vec<String> {
    let mut reply_contents: Vec<String> = vec![];
    for data in reply.data.children.iter() {
        let body = &data.data.body;
        reply_contents.push(body.to_string())
    }
    reply_contents
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let log_dir = format!("{}/.bottalks/", home::home_dir().unwrap().display());
    std::fs::create_dir_all(&log_dir).unwrap();
    let path = format!("{}/.env", log_dir);
    dotenv::from_path(path).unwrap();

    let mut log = LogDataBuilder::new();
    let start_time = Instant::now();
    //make it once for performance improvements
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    //Fetches the nwest posts
    let r = do_req::<NewestJson>(
        &client,
        "https://www.reddit.com/r/bottalks/new.json".to_owned(),
    )
    .await?;
    //Grab the newest post
    let url = &r.data.children[0].data.url;
    // Get the json data from the post this includes all commentsx
    let post = do_req::<PostData>(&client, format!("{}.json", url)).await?;

    let mut contents: Vec<String> = Vec::new();
    //Parse some comments, honestly not sure what im doing here
    for child in post[1].data.children.iter() {
        if let Some(body) = &child.data.body {
            contents.push(body.to_string());
        }
        if let Some(replies) = &child.data.replies {
            match replies {
                RepliesUnion::RepliesClass(data) => {
                    let mut r = read_things(data);
                    contents.append(&mut r);
                }
                RepliesUnion::String(data) => contents.push(data.to_string()),
            }
        }
    }
    log.set_comments(contents.clone());
    log.add_timing(
        "reddit_fetching_time",
        start_time.elapsed().as_millis().to_string(),
    );
    let parsed_posts_time = Instant::now();

    let joined = contents.join("\n");
    //This is necessary due to bert_ml making a tokio runtime for some odd reason
    //Uses bert-ml to generate a summarization of the post and a title,
    let (short, long) =
        spawn(|| summarize((joined.clone(), joined), ((10, 20), (400, 3000))).unwrap())
            .join()
            .unwrap();

    let title = &short[0];
    let desc = &long[0];
    // let (title, desc) = (short[0], long[0]);
    log.set_data(title.clone(), desc.clone());
    log.add_timing(
        "summarizing_posts_time",
        parsed_posts_time.elapsed().as_millis().to_string(),
    );
    let ai_time = Instant::now();

    //Creates a post on /r/bottalks
    //Using the ai generated material
    submit_post(title.to_owned(), desc.to_owned())
        .await
        .unwrap();

    log.add_timing("submitting_post", ai_time.elapsed().as_millis().to_string());
    log.add_timing("total_time", start_time.elapsed().as_millis().to_string());

    //Write to ~/.bottalks/logs/<log>
    log.log();
    let data: Value = json!({
        "embeds": [
            {
                "description": &desc.substring(0,1999),
                "title": &title.substring(0,50),
                "url": "https://reddit.com/r/bottalks",
                "color": 0x1345c3,
                "footer": {
                    "text": "r/bottalks is a project a project that uses ai to generate random text daily",
                },
            },
        ],
    });
    client
        .post(dotenv::var("WEBHOOK").unwrap())
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .await?;

    Ok(())
}

pub async fn submit_post(title: String, text: String) -> Result<(), roux::util::error::RouxError> {
    let client = Reddit::new(
        "linux:ai:v0.3.0 (by /u/bottalks)",
        &dotenv::var("CLIENT_ID").unwrap(),
        &dotenv::var("CLIENT_SECRET").unwrap(),
    )
    .username(&dotenv::var("REDDIT_USERNAME").unwrap())
    .password(&dotenv::var("REDDIT_PASSWORD").unwrap())
    .login()
    .await;

    let me = client.unwrap();
    me.submit_text(&title, &text, "bottalks").await?;
    Ok(())
}
