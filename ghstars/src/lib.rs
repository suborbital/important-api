use suborbital::runnable::*;
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

impl Runnable for Ghstars {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let repo_param = req::url_param("repo");
        let mut repo = String::from(repo_param.trim_start_matches("/"));

        let method = req::method();
        if method == "SCHED" {
            repo = match req::state("repo") {
                Some(val) =>  val,
                None => return Err(RunErr::new(403, "no repo provided"))
            }
        }

        if !repo.starts_with("suborbital") {
            return Err(RunErr::new(403, "invalid repo org"))
        }

        log::info(format!("fetching stars for {}", repo).as_str());
    
        let repo_details = http::get(format!("https://api.github.com/repos/{}", repo).as_str(), None)?;

        let repo: Repo = match serde_json::from_slice(repo_details.as_slice()) {
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
