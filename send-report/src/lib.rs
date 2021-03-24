use suborbital::runnable::*;
use suborbital::http;
use suborbital::req;
use suborbital::util;
use suborbital::log;
use suborbital::file;
use std::collections::BTreeMap;
use serde::{Serialize};

struct SendReport{}

#[derive(Serialize)]
struct WebhookContents {
    content: String
}

impl Runnable for SendReport {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let mut repo = req::url_param("repo");

        let method = req::method();
        if method == "SCHED" {
            repo = req::state("repo").unwrap_or_default();
        }

        let url = file::get_static("./webhook").unwrap_or_default();
        let url_str = util::to_string(url);

        let stargazers = req::state("stargazers").unwrap_or_default();

        let content = WebhookContents{
            content: format!("{}: {} stargazers", repo, stargazers)
        };

        let mut headers = BTreeMap::new();
        headers.insert("Content-Type", "application/json");

        let body = serde_json::to_vec(&content).unwrap_or_default();
        let resp = http::post(url_str.as_str(), Some(body), Some(headers))?;

        log::info(util::to_string(resp).as_str());

        Ok("ok".as_bytes().to_vec())
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &SendReport = &SendReport{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
