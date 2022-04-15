
use std::collections::BTreeMap;

fn main() -> Result<(), serde_yaml::Error> {
    let f = std::fs::File::open("tst.yml").unwrap();
    let d: serde_yaml::Value = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {:#?}", d);
    Ok(())
}

