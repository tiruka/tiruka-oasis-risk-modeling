pub mod processes;
pub mod structs;

use structs::VulnerabilityFootPrint;

use processes::{merge_footprint_with_vulnerabilities, read_vulnerabilities};

use crate::footprint::structs::FootPrint;

pub fn merge_vulnerabilities_with_footprint(
    footprints: Vec<FootPrint>,
    mut base_path: String,
) -> Vec<VulnerabilityFootPrint> {
    let vulnerabilities = read_vulnerabilities(base_path).unwrap();
    merge_footprint_with_vulnerabilities(vulnerabilities, footprints)
}
