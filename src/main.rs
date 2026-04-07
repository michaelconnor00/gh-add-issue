mod auth;

fn main() {
    if let Err(e) = auth::check_auth() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

