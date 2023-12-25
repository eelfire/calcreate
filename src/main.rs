use chrono::{DateTime, Utc};
use ics::properties::{DtEnd, DtStart, Location, RRule, Summary};
use ics::{parameters, Event, ICalendar, Standard, TimeZone};

use uuid::Uuid;

mod parser;
mod timeslots;
use clap::Parser;
use parser::run;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use timeslots::gen_slots;

fn get_slots_and_locations(course: &parser::Course, slot_type: Slot) -> (Vec<String>, String) {
    let mut slots = Vec::new();
    let mut location = String::new();
    if let Some(times) = match slot_type {
        Slot::L => &course.lecture_rooms,
        Slot::T => &course.tutorial_rooms,
        Slot::P => &course.lab_rooms,
    } {
        let slot_code = times.lines().next();
        let slot_code = slot_code.unwrap().trim().split(',');
        for slot in slot_code {
            slots.push(slot.trim().to_string());
        }

        if let Some(loc) = times.lines().nth(1) {
            location.push_str(loc.trim());
        }
    }
    (slots, location)
}

enum Slot {
    L,
    T,
    P,
}

fn gen_events(
    course_slots: Vec<String>,
    location: String,
    slot_type: Slot,
    slots: &HashMap<String, (usize, String)>,
    course_code_and_name: String,
    cal: &mut ICalendar,
) {
    for slot in course_slots {
        let current_time: DateTime<Utc> = Utc::now();
        let now = current_time.format("%Y%m%dT%H%M%SZ").to_string();
        let id = Uuid::new_v4().to_string();

        let mut event = Event::new(id, now);

        let summary = match slot_type {
            Slot::L => format!("{} - Lecture", course_code_and_name),
            Slot::T => format!("{} - Tutorial", course_code_and_name),
            Slot::P => format!("{} - Lab", course_code_and_name),
        };

        event.push(Summary::new(summary.clone()));
        event.push(Location::new(location.clone()));
        let mut rrule = String::from("FREQ=WEEKLY;WKST=MO;UNTIL=20240424T202959Z");
        let mut dtstart = String::from("20240103T");
        let mut dtend = String::from("20240103T");

        if let Some(get_slot) = slots.get(&slot) {
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

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    #[arg(short, long)]
    input_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let input_file = args.input_file;
    let input_path = Path::new(&input_file);
    let output_directory = input_path.parent().unwrap();
    let output_file = output_directory
        .join(input_path.file_stem().unwrap())
        .with_extension("ics");

    let courses = run(input_file).unwrap();

    let mut cal = ICalendar::new("2.0", "-//eelfire//Calendar//EN");
    let tz = TimeZone::standard(
        "Asia/Kolkata",
        Standard::new("19700101T000000", "+0530", "+0530"),
    );
    cal.add_timezone(tz);

    let slots = gen_slots();

    // let mut num_loops = 10;

    for course in courses {
        // if num_loops == 0 {
        //     break;
        // } else {
        //     num_loops -= 1;
        // }

        let (l_slots, l_location) = get_slots_and_locations(&course, Slot::L);
        let (t_slots, t_location) = get_slots_and_locations(&course, Slot::T);
        let (p_slots, p_location) = get_slots_and_locations(&course, Slot::P);

        let mut course_code_and_name = String::new();
        if let Some(ref course_code) = course.course_code {
            course_code_and_name.push_str(&course_code);
        }
        if let Some(ref course_name) = course.course_name {
            course_code_and_name.push_str(&format!(" - {}", course_name));
        }

        gen_events(
            l_slots,
            l_location,
            Slot::L,
            &slots,
            course_code_and_name.clone(),
            &mut cal,
        );
        gen_events(
            t_slots,
            t_location,
            Slot::T,
            &slots,
            course_code_and_name.clone(),
            &mut cal,
        );
        gen_events(
            p_slots,
            p_location,
            Slot::P,
            &slots,
            course_code_and_name.clone(),
            &mut cal,
        );

        println!("{:?}", course);
    }

    cal.save_file(output_file)?;

    Ok(())
}
