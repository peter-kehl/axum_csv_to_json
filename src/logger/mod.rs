use logdna_client::body::{IngestBody, KeyValueMap, Line};
use logdna_client::client::Client;
use logdna_client::params::{Params, Tags};
use logdna_client::request::RequestTemplate;
use logdna_client::response::Response;
use std::env;

//@TODO - if Client becomes `Clone`: #[derive(Clone)]
pub struct Logger {
    client: Client,
    labels: KeyValueMap,
    annotations: KeyValueMap,
}

impl Logger {
    pub fn new() -> Self {
        //env_logger::init();
        let params = Params::builder()
            .hostname("rust-client-test")
            .ip("127.0.0.1")
            .tags(Tags::parse("this,is,a,test"))
            .build()
            .expect("Params::builder()");

        let request_template = RequestTemplate::builder()
            .host(env::var("LOGDNA_HOST").unwrap_or_else(|_| "logs.logdna.com".into()))
            .params(params)
            .api_key(env::var("API_KEY").expect("api key missing"))
            .build()
            .expect("RequestTemplate::builder()");

        let client = Client::new(request_template, Some(true));

        let labels = KeyValueMap::new()
            .add("app", "test")
            .add("workload", "test");

        let annotations = KeyValueMap::new()
            .add("app", "test")
            .add("workload", "test");
        Self {
            client,
            labels,
            annotations,
        }
    }

    async fn send_for_level(
        &self,
        level: impl Into<String>,
        line: impl Into<String>,
    ) -> Result<(), ()> {
        let line = Line::builder()
            .line(line.into())
            .app("rust-client")
            .level(level.into())
            .labels(self.labels.clone())
            .annotations(self.annotations.clone())
            .build()
            .expect("Line::builder()");

        return match self.client.send(&IngestBody::new(vec![line])).await {
            Ok(handed_over) => match handed_over {
                Response::Sent => Ok(()),
                _ => Err(()),
            },
            _ => Err(()),
        };
    }

    pub async fn info(&self, line: impl Into<String>) -> Result<(), ()> {
        self.send_for_level("INFO", line).await
    }
}
