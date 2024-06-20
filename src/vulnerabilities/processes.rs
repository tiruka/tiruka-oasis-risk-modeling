use crate::footprint::{self, structs::FootPrint};
use csv;
use std::error::Error;
use std::fs::File;

use crate::vulnerabilities::structs::{Vulnerability, VulnerabilityFootPrint};

pub fn read_vulnerabilities(mut base_path: String) -> Result<Vec<Vulnerability>, Box<dyn Error>> {
    base_path.push_str("/vulnerability.csv");
    let file = File::open(base_path.as_str())?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut buffer = vec![];
    for result in rdr.deserialize() {
        let record: Vulnerability = result?;
        buffer.push(record);
    }
    Ok(buffer)
}

pub fn merge_footprint_with_vulnerabilities(
    vulnerabilities: Vec<Vulnerability>,
    footprints: Vec<FootPrint>,
) -> Vec<VulnerabilityFootPrint> {
    let mut buffer = vec![];
    for vul in &vulnerabilities {
        for footprint in &footprints {
            if footprint.intensity_bin_id == vul.intensity_bin_id {
                buffer.push(VulnerabilityFootPrint {
                    vulnerability_id: vul.vulnerability_id,
                    intensity_bin_id: vul.intensity_bin_id,
                    damage_bin_id: vul.damage_bin_id,
                    damage_probability: vul.probability,
                    event_id: footprint.event_id,
                    areaperil_id: footprint.areaperil_id,
                    footprint_probability: footprint.probability,
                    total_probability: footprint.probability * vul.probability,
                })
            }
        }
    }
    buffer
}
