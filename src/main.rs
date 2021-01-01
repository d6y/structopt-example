use structopt::StructOpt;
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

#[derive(Debug, StructOpt)]
struct Settings {
    /// Seed for the random number generator.
    #[structopt(short, long, default_value = "1")]
    seed: u64,
}

fn experiment_id(settings: &Settings, version: &str) -> String {
    let settings_text = format!("{:?}", &settings);
    format!("{}_{}", version, hash(settings_text))
}

fn hash(s: String) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut h = DefaultHasher::new();
    h.write(s.as_bytes());
    h.finish()
}


fn main() {
    let settings = Settings::from_args();
    println!("{:#?}", settings);
    println!("{}", GIT_VERSION);

    let id = experiment_id(&settings, GIT_VERSION);
    println!("{}", id);

}