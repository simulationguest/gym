use chrono::{Duration, NaiveDateTime};
use serde::Serialize;
use serde_with::DurationSeconds;

#[derive(Debug, Clone)]
pub enum Data {
	Day(Day),
	Exercise(Exercise),
	Set(Set),
	Unknown,
	Nothing,
}

#[derive(Debug, Clone, Serialize)]
pub struct Day {
	pub info: Option<DayInfo>,
	pub exercises: Vec<Exercise>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct DayInfo {
	pub name: String,
	pub date: Result<NaiveDateTime, String>,
	#[serde_as(as = "Option<DurationSeconds<i64>>")]
	pub duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Exercise {
	pub sets: Vec<Set>,
	pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Set {
	pub number: Option<f32>,
	pub kilos: Option<f32>,
	pub reps: Option<f32>,
}
