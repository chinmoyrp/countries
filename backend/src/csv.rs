use crate::country::Country;
use anyhow::Result;

pub type Countries = Vec<Country>;

pub fn read_from_csv(path: &str) -> Result<Countries> {
    let mut rds = csv::Reader::from_path(path)?;
    let mut countries = vec![];
    for result in rds.records() {
        let record: Country = result?.deserialize(None)?;
        countries.push(record);
    }

    Ok(countries)
}