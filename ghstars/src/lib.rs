use serde::{Deserialize, Serialize};
use serde_json;
use suborbital::http;
use suborbital::log;
use suborbital::req;
use suborbital::runnable::*;
use suborbital::util;

struct Ghstars{}

#[derive(Serialize, Deserialize)]
struct Repo {
    stargazers_count: u32,
}

impl Runnable for Ghstars {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let method = req::method();
        let repo = if method == "SCHED" {
             match req::state("repo") {
                Some(val) =>  val,
                None => return Err(RunErr::new(403, "no repo provided"))
            }
        } else {
            let repo_param = req::url_param("repo");
            repo_param.trim_start_matches("/").to_string()
        };

        if !repo.starts_with("suborbital") {
            return Err(RunErr::new(403, "invalid repo org"))
        }

        log::info(format!("fetching stars for {}", repo).as_str());
    
        let repo_details = match http::get(format!("https://api.github.com/repos/{}", repo).as_str(), None) {
            Ok(val) => val,
            Err(e) => { return Err(RunErr::new(500, e.message.as_str())) }
        };

        let repo: Repo = match serde_json::from_slice(&repo_details) {
            Ok(r) => r,
            Err(_) => return Err(RunErr::new(500, "failed to parse repo details"))
        };

        Ok(util::to_vec(format!("{}", repo.stargazers_count)))
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &Ghstars = &Ghstars{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
