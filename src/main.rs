mod polydex;
use std::{fs::File, io::Write};

use polydex::*;

fn main() -> Result<(), std::io::Error> {
    let csv_file = File::open("polydex.csv")?;
    let mut reader = csv::Reader::from_reader(csv_file);

    let mut polydex = vec![];

    for result in reader.deserialize() {
        let record: Entry = result?;
        polydex.push(record);
    }

    let ron_str = ron::ser::to_string_pretty(&polydex, ron::ser::PrettyConfig::default()).unwrap();

    let mut ron_file = File::create("polydex.ron")?;
    ron_file.write_all(ron_str.as_bytes())?;

    Ok(())
}
