use crate::solution::Solution;

pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day10;

#[must_use]
pub fn get_day(day: usize) -> Option<Box<dyn Solution>> {
  match day {
      0 => Some(Box::new(day00::Day)),
      1 => Some(Box::new(day01::Day)),
      2 => Some(Box::new(day02::Day)),
      3 => Some(Box::new(day03::Day)),
      10 => Some(Box::new(day10::Day)),
      _ => None,
  }
}
