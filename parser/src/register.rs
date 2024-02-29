use serde::Serialize;

use crate::data::*;

#[derive(Debug, Clone, Serialize)]
pub struct Register {
	pub days: Vec<Day>,
}

impl Default for Register {
	fn default() -> Self {
		Self::new()
	}
}

impl Register {
	pub fn new() -> Self {
		Self { days: Vec::new() }
	}

	fn last_day(&mut self) -> &mut Day {
		if self.days.is_empty() {
			self.days.push(Day {
				info: None,
				exercises: Vec::new(),
			})
		}
		let last = self.days.len() - 1;
		&mut self.days[last]
	}

	pub fn push_day(&mut self, day: Day) {
		self.days.push(day);
	}

	pub fn push_exercise(&mut self, exercise: Exercise) {
		self.last_day().exercises.push(exercise);
	}

	pub fn push_set(&mut self, set: Set) {
		let day = self.last_day();
		if day.exercises.is_empty() {
			day.exercises.push(Exercise {
				sets: Vec::new(),
				name: None,
			})
		}
		let last = day.exercises.len() - 1;
		day.exercises[last].sets.push(set);
	}

	pub fn push_data(&mut self, data: Data) {
		match data {
			Data::Day(day) => self.push_day(day),
			Data::Exercise(exercise) => self.push_exercise(exercise),
			Data::Set(set) => self.push_set(set),
			Data::Unknown => {
				dbg!("unknown line");
			}
			Data::Nothing => {}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::parser::parse_line;

	use super::*;

	#[test]
	fn test_example() {
		let file = r#"
    "Cardio · Tag 4 · Woche 4 · M2 · Deload";"2024-01-18 15:45 Uhr";57 Min.
"1. Laufen auf dem Laufband · Maschine · 20 Min."
#; KM; MIN.
1;2,25;20
"2. Stationäres Radfahren · Maschine · 15 Min."
#; KM; MIN.
1;5;15
"3. Rudergerät · Maschine"
#; KM; MIN.
1;1;4:30
"4. Crosstrainer · Maschine"
#; KM; MIN.
1;2,07;15
"#;

		let lines: Vec<Data> = file.lines().map(parse_line).collect();
		println!("{lines:?}");
	}
}
