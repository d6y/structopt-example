use git_version::git_version;
use structopt::StructOpt;

use std::collections::HashMap;

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

    let examples = vec!["a", "b", "b", "c", "c"];

    // Deterministic hashmap:
    use rustc_hash::FxHasher;
    use std::hash::BuildHasherDefault;
    let mut class_count = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());

    // Default hash map:
    // let mut class_count = HashMap::new();

    for example in examples {
        *class_count.entry(example).or_insert(0) += 1;
    }

    println!("Counts: {:?}", class_count);

    let most_frequent_class: Option<&str> = class_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(class, _)| class);

    println!("Most frequent: {:?}", most_frequent_class);
}
