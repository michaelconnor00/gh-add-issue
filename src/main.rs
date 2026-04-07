mod auth;
mod repos;
mod select;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    log::debug!("gh-add-issue starting");

    if let Err(e) = auth::check_auth() {
        log::error!("{e}");
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let repo_list = match repos::fetch_repos() {
        Ok(r) => r,
        Err(e) => {
            log::error!("{e}");
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    log::info!("Fetched {} repositories.", repo_list.len());

    let selected = match select::select_repo(&repo_list) {
        Ok(r) => r,
        Err(e) => {
            log::error!("{e}");
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    log::info!("Selected repository: {selected}");

    // Placeholder — issue input prompts (section 4) come next.
    println!("Selected: {selected}");
}

