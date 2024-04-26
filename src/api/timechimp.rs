// use reqwest::header;
// use std::fs;
use std::fs::File;
use std::io::Read;

use crate::api::types;

// temp local json usage so we don't unnecessarily call API
// probably want to implement caching at some point
pub fn load_timechimp_data() -> Result<Vec<types::Registration>, Box<dyn std::error::Error>> {
    let mut local_json = File::open("res-2023.json")?;
    let mut contents = String::new();
    local_json.read_to_string(&mut contents)?;

    let json: types::Root = serde_json::from_str(&contents).expect("Failed to deserialize JSON");

    let mut regs = json.data;
    regs.sort_by(|a, b| a.date.cmp(&b.date));

    // TODO: this works but still want to find a way to do this without extra fn call
    for reg in regs.iter_mut() {
        reg.set_multiplier();
    }

    Ok(regs)
}

// pub fn load_timechimp_data() -> Result<Vec<types::Registration>, Box<dyn std::error::Error>> {
//     // TODO: moved this to a file cause I got sick of it.
//     // Find a way to clean up & have user enter the date period
//     // let query_date_range = "01%2F01%2F2023~12%2F31%2F2023";

//     let cookie = fs::read_to_string("cookie.txt").unwrap();
//     let cookie = cookie.trim();
//     let body_form = fs::read_to_string("timechimp-form-body.txt").unwrap();
//     // let body = body_form.trim(); // TODO: this is not allowed for some reason?

//     let mut headers = header::HeaderMap::new();
//     headers.insert(
//         "Accept",
//         "application/json, text/javascript, */*; q=0.01"
//             .parse()
//             .unwrap(),
//     );
//     headers.insert("Accept-Language", "en-GB,en;q=0.5".parse().unwrap());
//     headers.insert(
//         "Content-Type",
//         "application/x-www-form-urlencoded; charset=UTF-8"
//             .parse()
//             .unwrap(),
//     );
//     headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
//     headers.insert("Origin", "https://app.timechimp.com".parse().unwrap());
//     headers.insert(header::COOKIE, cookie.parse().unwrap());

//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let req = client
//         .post("https://app.timechimp.com/api/time/datatables")
//         .headers(headers)
//         .body(body_form)
//         .build()
//         .unwrap();

//     let res = client.execute(req).unwrap();

//     let json: types::Root = serde_json::from_str(&res.text().expect("Failed to parse response"))
//         .expect("Failed to deserialize JSON");

//     let mut regs = json.data;

//     // TODO: this works but still want to find a way to do this without extra fn call
//     for reg in regs.iter_mut() {
//         reg.set_multiplier();
//     }

//     Ok(regs)
// }
