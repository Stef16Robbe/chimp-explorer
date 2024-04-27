use std::error;

use chrono::Datelike;

use crate::api::{timechimp::load_timechimp_data, types::Registration};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub registrations: Vec<Registration>,
    pub total_hours: f64,
    pub cumulative_hours: Vec<(f64, f64)>,
    pub hour_target: Vec<(f64, f64)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            registrations: Vec::new(),
            total_hours: 0.0,
            cumulative_hours: Vec::new(),
            hour_target: Vec::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.registrations = load_timechimp_data().unwrap();
        for reg in &self.registrations {
            self.total_hours += reg.time * reg.hours_multiplier;
        }

        for reg in self.registrations.iter() {
            // (month, cumulative hours)
            let cumul_hours = if self.cumulative_hours.len() > 0 {
                (self.cumulative_hours[self.cumulative_hours.len() - 1].1
                    + reg.time * reg.hours_multiplier)
                    .round()
            } else {
                reg.time * reg.hours_multiplier
            };
            self.cumulative_hours
                .push((reg.date.ordinal() as f64, cumul_hours));
        }

        // hard coded 1680 hour target for now
        self.hour_target = (0..365).map(|x| (x as f64, 1680 as f64)).collect();

        Ok(())
    }
}
