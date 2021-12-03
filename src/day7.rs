mod common;

use common::InputConversions;

#[common::aoc(day = 5, name = "The Treachery of Whales")]
fn main() {
  common::read_split_file(7, ",").to::<i128>()
}

#[common::task(part = 1, name = "Fuel consumed")]
fn fuel_consumed(input: &mut [i128]) {
  input.sort_unstable();

  let median = input.get(input.len() / 2).unwrap();

  input.iter().cloned().map(|position| (median - position).abs()).sum::<i128>()
}

#[common::task(part = 2, name = "Adjusted fuel consumed")]
fn adjusted_fuel_consumed(input: &mut [i128]) {
  let mean: f64 = input.iter().sum::<i128>() as f64 / input.len() as f64;
  let (floor, ceil) = (mean.floor() as i128, mean.ceil() as i128);

  let fuels: (Vec<_>, Vec<_>) = input
    .iter()
    .cloned()
    .map(|position| (adjusted_fuel_consumption((floor - position).abs()), adjusted_fuel_consumption((ceil - position).abs())))
    .unzip();

  fuels.0.iter().sum::<i128>().min(fuels.1.iter().sum::<i128>())
}

fn adjusted_fuel_consumption(distance: i128) -> i128 {
  (distance * (1 + distance)) / 2
}

#[cfg(test)]
mod tests {
  fn input() -> Vec<i128> {
    vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
  }

  #[test]
  fn fuel_consumed() {
    let mut input = input();
    let result = super::inner_fuel_consumed(&mut input);

    assert_eq!(result, 37);
  }

  #[test]
  fn adjusted_fuel_consumed() {
    let mut input = input();
    let result = super::inner_adjusted_fuel_consumed(&mut input);

    assert_eq!(result, 168);
  }
}
