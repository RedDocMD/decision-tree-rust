use decision_tree;
use decision_tree::InputData;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <csv_file_path> <result_column_number>", args[0]);
    } else {
        let data_filename = &args[1];
        let result_column: usize = args[2].parse()?;
        let data = InputData::from_file(data_filename, result_column)?;
        let tree = decision_tree::ida3(&data);
        println!("{}", tree);
    }
    Ok(())
}
