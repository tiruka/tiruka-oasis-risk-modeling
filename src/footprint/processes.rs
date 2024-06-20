use crate::footprint::structs::FootPrint;
use csv;
use std::error::Error;
use std::fs::File;

pub fn read_footprint(mut base_path: String) -> Result<Vec<FootPrint>, Box<dyn Error>> {
    base_path.push_str("/footprint.csv");
    let file = File::open(base_path.as_str())?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut buffer = vec![];
    for result in rdr.deserialize() {
        let record: FootPrint = result?;
        buffer.push(record);
    }
    Ok(buffer)
}

pub fn merge_footprint_with_events(
    event_ids: Vec<i32>,
    footprints: Vec<FootPrint>,
) -> Vec<FootPrint> {
    let mut buffer = vec![];
    for event_id in event_ids {
        for footprint in &footprints {
            if footprint.event_id == event_id {
                buffer.push(footprint.clone());
            }
        }
    }
    buffer
}
