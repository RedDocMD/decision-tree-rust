use decision_tree;
use decision_tree::InputData;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data_filename = "/home/deep/work/rust/decision_tree/data/mushroom.csv";
    let data = InputData::from_file(data_filename, 0)?;
    println!("{}", data);
    let tree = decision_tree::ida3(&data);
    println!("{}", tree);
    Ok(())
}
