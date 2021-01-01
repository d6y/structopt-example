use structopt::StructOpt;
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

#[derive(Debug, StructOpt)]
struct Settings {
    /// Seed for the random number generator.
    #[structopt(short, long, default_value = "1")]
    seed: u64,
}


fn main() {
    let settings = Settings::from_args();
    println!("{:#?}", settings);
    println!("{}", GIT_VERSION);
}