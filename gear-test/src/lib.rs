pub mod common;
pub mod log;
pub mod runner;
pub mod sample;

use common::Result;
use sample::test::Test;

use std::{fs, path::Path};

pub fn read_test_from_file<P: AsRef<Path>>(path: P) -> Result<Test> {
    let file = fs::File::open(path.as_ref())
        .map_err(|e| format!("Error loading '{}': {}", path.as_ref().display(), e))?;

    Ok(serde_yaml::from_reader(file)
        .map_err(|e| format!("Error decoding '{}': {}", path.as_ref().display(), e))?)
}

#[test]
fn name() {
    let test = read_test_from_file("/Users/breathx/Work/gear/gear-test/spec/sample.yaml").unwrap();

    let test = common::create_test(test);

    println!("{:?}", test.unwrap().fixtures);
}
