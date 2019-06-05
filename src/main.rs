extern crate reqwest;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: i32,
    name: String,
    biography: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeSlot {
    id: i32,
    guid: String,
    logo: Option<String>,
    date: String,
    start: String,
    duration: String,
    room: String,
    slug: String,
    url: String,
    title: String,
    subtitle: String,
    track: Option<String>,
    r#type: String,
    language: String,
    r#abstract: String,
    description: String,
    recording_license: String,
    do_not_record: bool,
    persons: Vec<Person>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Day {
    index: i32,
    date: String,
    day_start: String,
    day_end: String,
    rooms: HashMap<String, Vec<TimeSlot>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Conference {
    acronym: String,
    title: String,
    #[serde(rename = "daysCount")]
    days_count: i32,
    start: String,
    end: String,
    timeslot_duration: String,
    days: Vec<Day>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Schedule {
    version: String,
    conference: Conference,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ScheduleResp {
    schedule: Schedule
}

fn get_schedule_data<U: reqwest::IntoUrl>(url: U) -> Result<ScheduleResp, Box<Error>> {
    Ok(reqwest::get(url)?.json()?)
}

fn main() {
    match get_schedule_data("https://timetable.flawcon.xyz/2019/schedule/export/schedule.json") {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("Error getting schedule: {:?}", e)
    }
}
