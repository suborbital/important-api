use suborbital::runnable;
use suborbital::http;
use suborbital::req;
use suborbital::util;
use suborbital::log;
use suborbital::file;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

struct SendReport{}

#[derive(Serialize, Deserialize)]
struct WebhookContents {
    content: String
}

impl runnable::Runnable for SendReport {
    fn run(&self, _: Vec<u8>) -> Option<Vec<u8>> {
        let repo = req::url_param("repo");
        let url = file::get_static("./webhook").unwrap_or_default();
        let url_str = util::to_string(url);

        let stargazers = req::state("stargazers");

        let content = WebhookContents{
            content: format!("github.com{} {}", repo, stargazers)
        };

        let mut headers = BTreeMap::new();
        headers.insert("Content-Type", "application/json");

        let body = serde_json::to_vec(&content).unwrap_or_default();
        let resp = http::post(url_str.as_str(), Some(body), Some(headers));

        log::info(util::to_string(resp).as_str());

        Some("ok".as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &SendReport = &SendReport{};

#[no_mangle]
pub extern fn init() {
    runnable::set(RUNNABLE);
}
