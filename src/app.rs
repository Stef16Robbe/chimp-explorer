use std::collections::HashMap;
use std::error;

use chrono::Datelike;

use crate::api::{timechimp::load_timechimp_data, types::Registration};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub registrations: Vec<Registration>,
    pub total_hours: f64,
    pub cumulative_hours: Vec<(f64, f64)>,
    pub hour_target: Vec<(f64, f64)>,
    pub customer_hours_division: Vec<(String, u64)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            registrations: Vec::new(),
            total_hours: 0.0,
            cumulative_hours: Vec::new(),
            hour_target: Vec::new(),
            customer_hours_division: Vec::new(),
        }
    }
}

impl App {
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // TODO:
    // don't load all data at once at startup in a function like this lol
    pub fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.registrations = load_timechimp_data().unwrap();
        for reg in &self.registrations {
            self.total_hours += reg.hours * reg.hours_multiplier;
        }

        for reg in self.registrations.iter() {
            // (month, cumulative hours)
            let cumul_hours = if !self.cumulative_hours.is_empty() {
                (self.cumulative_hours[self.cumulative_hours.len() - 1].1
                    + reg.hours * reg.hours_multiplier)
                    .round()
            } else {
                reg.hours * reg.hours_multiplier
            };
            self.cumulative_hours
                .push((reg.date.ordinal() as f64, cumul_hours));
        }

        // hard coded 1680 hour target for now
        self.hour_target = (1..366).map(|x| (x as f64, 1680_f64)).collect();

        // get unique customer with total hours in map
        let mut tmp_map: HashMap<&str, u64> = HashMap::new();
        for reg in &self.registrations {
            *tmp_map.entry(&reg.customer).or_insert(0) += (reg.hours * reg.hours_multiplier) as u64;
        }

        // converting it to a `Vec<(String, f64)>` cause that's what the BarChart wants
        self.customer_hours_division = tmp_map
            .into_iter()
            .map(|(customer, total_hours)| (String::from(customer), total_hours))
            .collect();

        Ok(())
    }
}
