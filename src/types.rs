use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)] // TODO: impl Serialize for Root
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Registration>,
    // pub draw: i64,
    // pub records_total: i64,
    // pub records_filtered: i64,
    // pub error: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)] // TODO: impl Serialize for Registration
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub id: i64,
    pub week: i64,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    pub customer: String,
    pub project: String,
    pub task: String,
    pub time: f64,
    pub status: i64,
    #[serde(
        deserialize_with = "deserialize_hours_multiplier",
        default = "default_hours_multiplier"
    )]
    pub hours_multiplier: f32,
    // pub billable: bool,
    // pub company: Value,
    // pub company_id: i64,
    // pub customer_id: i64,
    // pub customer_relation_id: Value,
    // pub project_code: Value,
    // pub project_name: String,
    // pub project_id: i64,
    // pub user: String,
    // pub user_id: i64,
    // pub user_name: String,
    // pub user_tags: String,
    // pub customer_tags: Value,
    // pub task_tags: String,
    // pub notes: Value,
    // pub start_end: Option<String>,
    // pub start: Option<String>,
    // pub end: Option<String>,
    // pub pause: Value,
    // pub amount: Value,
    // pub project_manager_user_ids: Value,
    // pub tags: Value,
    // pub client_status: i64,
    // pub approved_by: Value,
    // pub approved_date: Value,
    // pub client_approved_by: Value,
    // pub client_approved_date: Value,
    // pub external_url: Value,
    // pub external_name: Value,
    // pub invoice_id: i64,
    // pub clockin_photo: Value,
    // pub clockout_photo: Value,
    // pub clockin_location: Value,
    // pub clockout_location: Value,
    // pub self_billing_status: i64,
    // pub timezone: Option<String>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
        .map(|dt| dt.date())
        .map_err(serde::de::Error::custom)
}

fn deserialize_hours_multiplier<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    let task: &str = value
        .get("task")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| serde::de::Error::custom("missing task"))?;

    let hours_multiplier = match task {
        "Werkuren" => 1.0,
        "Standby VZ ma-za 3,2%" => 0.032,
        "Standby VZ zon 6,3%" => 0.063,
        "Inzet Standby  213%" => 2.13,
        "Verlof" => 0.0,
        "Feestdag" => 0.0,
        _ => 1.0,
    };

    Ok(hours_multiplier)
}

fn default_hours_multiplier() -> f32 {
    1.0
}
