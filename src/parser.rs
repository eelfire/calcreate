use std::{error::Error, fs::File};

use csv::ReaderBuilder;

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
pub struct Course {
    #[serde(rename = "Course Code")]
    pub course_code: Option<String>,
    #[serde(rename = "Course Name")]
    pub course_name: Option<String>,
    #[serde(rename = "L")]
    pub l: Option<f32>,
    #[serde(rename = "T")]
    pub t: Option<f32>,
    #[serde(rename = "P")]
    pub p: Option<f32>,
    #[serde(rename = "C")]
    pub c: Option<f32>,
    #[serde(rename = "Name of the Instructors and Tutors")]
    pub instructors: Option<String>,
    #[serde(rename = "Capacity")]
    pub capacity: Option<i32>,
    #[serde(rename = "Number of Students")]
    pub num_students: Option<i32>,
    #[serde(rename = "Sections")]
    pub sections: Option<String>,
    #[serde(rename = "Lecture")]
    pub lecture_rooms: Option<String>,
    #[serde(rename = "Tutorial")]
    pub tutorial_rooms: Option<String>,
    #[serde(rename = "Lab")]
    pub lab_rooms: Option<String>,
    #[serde(rename = "Link to Course Plan")]
    pub course_plan_link: Option<String>,
    #[serde(rename = "Preferred Knowledge Equivalent to")]
    pub knowledge_equivalent: Option<String>,
    #[serde(rename = "Remark")]
    pub remark: Option<String>,
    #[serde(rename = "Minor in")]
    pub minor_in: Option<String>,
    #[serde(rename = "HSS/BS elective")]
    pub hss_bs_elective: Option<String>,
}

pub fn run() -> Result<Vec<Course>, Box<dyn Error>> {
    // let file = File::open("csv-ics/timetable.csv")?;
    // let file = File::open("csv-ics/5th-sem.csv")?;
    let file = File::open("csv-ics/jivi.csv")?;
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = vec![];

    for result in rdr.deserialize() {
        let record: Course = result?;
        records.push(record);
        // println!("{:?}", record);
    }
    // Ok(())
    Ok(records)
}
