use std::{fs::File, io::Read};

use crate::api::types;

// temp local json usage so we don't unnecessarily call API
// probably want to implement caching at some point
pub fn load_timechimp_data() -> Result<Vec<types::Registration>, Box<dyn std::error::Error>> {
    let mut local_json = File::open("uren.json")?;
    let mut contents = String::new();
    local_json.read_to_string(&mut contents)?;

    let mut regs: Vec<types::Registration> =
        serde_json::from_str(&contents).expect("Failed to deserialize JSON");

    regs.sort_by(|a, b| a.date.cmp(&b.date));

    // TODO: this works but still want to find a way to do this without extra fn call
    for reg in regs.iter_mut() {
        reg.set_multiplier();
    }

    Ok(regs)
}
