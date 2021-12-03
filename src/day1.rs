mod common;

use common::InputConversions;

#[common::aoc(day = 1, name = "Sonar Sweep")]
fn main() {
  common::read_lined_file(1).to::<u128>()
}

#[common::task(part = 1, name = "Depth increase rate")]
fn depth_increase(input: &[u128]) {
  let (_, count) = input
    .iter()
    .fold((&u128::MAX, 0), |(last, count), number| if number > last { (number, count + 1) } else { (number, count) });

  count
}

#[common::task(part = 2, name = "Adjusted depth increase rate")]
fn depth_increase_adjusted(input: &[u128]) {
  let (_, count) = input.windows(3).fold((u128::MAX, 0), |(last, count), numbers| {
    let numbers = numbers.iter().sum();

    if numbers > last {
      (numbers, count + 1)
    } else {
      (numbers, count)
    }
  });

  count
}

#[cfg(test)]
mod tests {
  fn input() -> Vec<u128> {
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
  }

  #[test]
  fn depth_increase() {
    let mut input = input();
    let result = super::inner_depth_increase(&mut input);

    assert_eq!(result, 7);
  }

  #[test]
  fn depth_increase_adjusted() {
    let mut input = input();
    let result = super::inner_depth_increase_adjusted(&mut input);

    assert_eq!(result, 5);
  }
}
