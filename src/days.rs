use crate::solution::Solution;

pub mod day00;
pub mod day01;

#[must_use]
pub fn get_day(day: usize) -> Option<Box<dyn Solution>> {
  match day {
      0 => Some(Box::new(day00::Day)),
      1 => Some(Box::new(day01::Day)),
      _ => None,
  }
}
