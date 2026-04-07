mod auth;
mod repos;

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

    // Placeholder — repository selection UI (section 3.2) comes next.
    log::info!("Fetched {} repositories.", repo_list.len());
}

