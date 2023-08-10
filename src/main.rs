use chrono::{DateTime, Utc};
use ics::properties::{DtEnd, DtStart, Location, RRule, Summary};
use ics::{parameters, Event, ICalendar, Standard, TimeZone};

use uuid::Uuid;

mod parser;
mod timeslots;
use parser::run;
use timeslots::maap;

fn main() -> std::io::Result<()> {
    let gibme = run().unwrap();

    let mut cal = ICalendar::new("2.0", "-//eelfire//Calendar//EN");
    let tz = TimeZone::standard(
        "Asia/Kolkata",
        Standard::new("19700101T000000", "+0530", "+0530"),
    );
    cal.add_timezone(tz);

    let maap = maap();

    let mut num_loops = 10;

    for course in gibme {
        if num_loops == 0 {
            break;
        } else {
            num_loops -= 1;
        }

        let mut l_slots = Vec::new();
        let mut l_location = String::new();
        if let Some(l_times) = course.lecture_rooms {
            let slot_code = l_times.lines().next();
            let slot_code = slot_code.unwrap().trim().split(',');
            for slot in slot_code {
                l_slots.push(slot.trim().to_string());
            }

            if let Some(loc) = l_times.lines().nth(1) {
                l_location.push_str(loc.trim());
            }
        }

        let mut t_slots = Vec::new();
        let mut t_location = String::new();
        if let Some(t_times) = course.tutorial_rooms {
            let slot_code = t_times.lines().next();
            let slot_code = slot_code.unwrap().trim().split(',');
            for slot in slot_code {
                t_slots.push(slot.trim().to_string());
            }

            if let Some(loc) = t_times.lines().nth(1) {
                t_location.push_str(loc.trim());
            }
        }

        let mut p_slots = Vec::new();
        let mut p_location = String::new();
        if let Some(p_times) = course.lab_rooms {
            let slot_code = p_times.lines().next();
            let slot_code = slot_code.unwrap().trim().split(',');
            for slot in slot_code {
                p_slots.push(slot.trim().to_string());
            }

            if let Some(loc) = p_times.lines().nth(1) {
                p_location.push_str(loc.trim());
            }
        }

        let mut summary = String::new();
        if let Some(course_code) = course.course_code {
            summary.push_str(&course_code);
        }
        if let Some(course_name) = course.course_name {
            summary.push_str(&format!(" - {}", course_name));
        }

        for slot in l_slots {
            let current_time: DateTime<Utc> = Utc::now();
            let now = current_time.format("%Y%m%dT%H%M%SZ").to_string();
            let id = Uuid::new_v4().to_string();

            let mut event = Event::new(id, now);

            let l_summary = format!("{} - Lecture", summary);
            event.push(Summary::new(l_summary.clone()));
            event.push(Location::new(l_location.clone()));
            let mut rrule = String::from("FREQ=WEEKLY;WKST=MO;UNTIL=20231124T182959Z");
            let mut dtstart = String::from("20230802T");
            let mut dtend = String::from("20230802T");

            if let Some(get_slot) = maap.get(&slot) {
                let (time, day) = get_slot;
                // println!("{} {} {}", slot, time, day);

                rrule.push_str(format!(";BYDAY={}", day).as_str());
                event.push(RRule::new(rrule));

                match time {
                    0 => {
                        dtstart.push_str("083000");
                        dtend.push_str("095000");
                    }
                    1 => {
                        dtstart.push_str("100000");
                        dtend.push_str("112000");
                    }
                    2 => {
                        dtstart.push_str("113000");
                        dtend.push_str("125000");
                    }
                    3 => {
                        dtstart.push_str("130000");
                        dtend.push_str("140000");
                    }
                    4 => {
                        dtstart.push_str("140000");
                        dtend.push_str("152000");
                    }
                    5 => {
                        dtstart.push_str("153000");
                        dtend.push_str("165000");
                    }
                    6 => {
                        dtstart.push_str("170000");
                        dtend.push_str("182000");
                    }
                    _ => {
                        dtstart.push_str("200000");
                        dtend.push_str("212000");
                    }
                }
                let mut dtstart = DtStart::new(dtstart);
                let mut dtend = DtEnd::new(dtend);
                dtstart.append(parameters!("TZID" => "Asia/Kolkata"));
                dtend.append(parameters!("TZID" => "Asia/Kolkata"));
                event.push(dtstart);
                event.push(dtend);
            }

            cal.add_event(event);
        }

        for slot in t_slots {
            let current_time: DateTime<Utc> = Utc::now();
            let now = current_time.format("%Y%m%dT%H%M%SZ").to_string();
            let id = Uuid::new_v4().to_string();

            let mut event = Event::new(id, now);

            let t_summary = format!("{} - Tutorial", summary);
            event.push(Summary::new(t_summary.clone()));
            event.push(Location::new(t_location.clone()));
            let mut rrule = String::from("FREQ=WEEKLY;WKST=MO;UNTIL=20231124T182959Z");
            let mut dtstart = String::from("20230802T");
            let mut dtend = String::from("20230802T");

            if let Some(get_slot) = maap.get(&slot) {
                let (time, day) = get_slot;
                // println!("{} {} {}", slot, time, day);

                rrule.push_str(format!(";BYDAY={}", day).as_str());
                event.push(RRule::new(rrule));

                match time {
                    0 => {
                        dtstart.push_str("083000");
                        dtend.push_str("095000");
                    }
                    1 => {
                        dtstart.push_str("100000");
                        dtend.push_str("112000");
                    }
                    2 => {
                        dtstart.push_str("113000");
                        dtend.push_str("125000");
                    }
                    3 => {
                        dtstart.push_str("130000");
                        dtend.push_str("140000");
                    }
                    4 => {
                        dtstart.push_str("140000");
                        dtend.push_str("152000");
                    }
                    5 => {
                        dtstart.push_str("153000");
                        dtend.push_str("165000");
                    }
                    6 => {
                        dtstart.push_str("170000");
                        dtend.push_str("182000");
                    }
                    _ => {
                        dtstart.push_str("200000");
                        dtend.push_str("212000");
                    }
                }
                let mut dtstart = DtStart::new(dtstart);
                let mut dtend = DtEnd::new(dtend);
                dtstart.append(parameters!("TZID" => "Asia/Kolkata"));
                dtend.append(parameters!("TZID" => "Asia/Kolkata"));
                event.push(dtstart);
                event.push(dtend);
            }

            cal.add_event(event);
        }

        for slot in p_slots {
            let current_time: DateTime<Utc> = Utc::now();
            let now = current_time.format("%Y%m%dT%H%M%SZ").to_string();
            let id = Uuid::new_v4().to_string();

            let mut event = Event::new(id, now);

            let p_summary = format!("{} - Lab", summary);
            event.push(Summary::new(p_summary.clone()));
            event.push(Location::new(p_location.clone()));
            let mut rrule = String::from("FREQ=WEEKLY;WKST=MO;UNTIL=20231124T182959Z");
            let mut dtstart = String::from("20230802T");
            let mut dtend = String::from("20230802T");

            if let Some(get_slot) = maap.get(&slot) {
                let (time, day) = get_slot;
                // println!("{} {} {}", slot, time, day);

                rrule.push_str(format!(";BYDAY={}", day).as_str());
                event.push(RRule::new(rrule));

                match time {
                    0 => {
                        dtstart.push_str("083000");
                        dtend.push_str("095000");
                    }
                    1 => {
                        dtstart.push_str("100000");
                        dtend.push_str("112000");
                    }
                    2 => {
                        dtstart.push_str("113000");
                        dtend.push_str("125000");
                    }
                    3 => {
                        dtstart.push_str("130000");
                        dtend.push_str("140000");
                    }
                    4 => {
                        dtstart.push_str("140000");
                        dtend.push_str("152000");
                    }
                    5 => {
                        dtstart.push_str("153000");
                        dtend.push_str("165000");
                    }
                    6 => {
                        dtstart.push_str("170000");
                        dtend.push_str("182000");
                    }
                    _ => {
                        dtstart.push_str("200000");
                        dtend.push_str("212000");
                    }
                }
                let mut dtstart = DtStart::new(dtstart);
                let mut dtend = DtEnd::new(dtend);
                dtstart.append(parameters!("TZID" => "Asia/Kolkata"));
                dtend.append(parameters!("TZID" => "Asia/Kolkata"));
                event.push(dtstart);
                event.push(dtend);
            }

            cal.add_event(event);
        }
    }

    // cal.save_file("csv-ics/bigbrain.ics");
    // cal.save_file("csv-ics/bigbrain-5.ics");
    cal.save_file("csv-ics/jivi.ics");

    Ok(())
}
