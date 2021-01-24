use suborbital::runnable;
use suborbital::req;
use suborbital::http;
use suborbital::log;
use suborbital::util;
use serde::{Serialize, Deserialize};
use serde_json;

struct Ghstars{}

#[derive(Serialize, Deserialize)]
struct Repo {
    stargazers_count: i32,
}

impl runnable::Runnable for Ghstars {
    fn run(&self, _: Vec<u8>) -> Option<Vec<u8>> {
        let repo_param = req::url_param("repo");
        let repo = repo_param.trim_start_matches("/");

        log::info(format!("fetching stars for {}", repo).as_str());
    
        let repo_details = http::get(format!("https://api.github.com/repos/{}", repo).as_str());

        let repo: Repo = match serde_json::from_slice(repo_details.as_slice()) {
            Ok(r) => r,
            Err(_) => return None,
        };

        Some(util::to_vec(format!("stargazers: {}", repo.stargazers_count)))
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &Ghstars = &Ghstars{};

#[no_mangle]
pub extern fn init() {
    runnable::set(RUNNABLE);
}
