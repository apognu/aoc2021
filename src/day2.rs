mod common;

#[derive(Default)]
struct Position {
  position: i64,
  depth: i64,
  aim: i64,
}

#[common::aoc(day = 2, name = "Dive!")]
fn main() {
  common::read_lined_file(2)
}

#[common::task(part = 1, name = "Submarine position")]
fn position(input: &[String]) {
  let position = input.iter().fold(Position::default(), |acc, op| match op.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
    ["forward", x] => Position {
      position: acc.position + to_i64(x),
      ..acc
    },

    ["up", x] => Position { depth: acc.depth - to_i64(x), ..acc },
    ["down", x] => Position { depth: acc.depth + to_i64(x), ..acc },
    _ => acc,
  });

  position.position * position.depth
}

#[common::task(part = 2, name = "Submarine position and aim")]
fn position_and_aim(input: &[String]) {
  let position = input.iter().fold(Position::default(), |acc, op| match op.splitn(2, ' ').collect::<Vec<&str>>().as_slice() {
    ["forward", x] => Position {
      position: acc.position + to_i64(x),
      depth: acc.depth + (acc.aim * to_i64(x)),
      ..acc
    },

    ["up", x] => Position { aim: acc.aim - to_i64(x), ..acc },
    ["down", x] => Position { aim: acc.aim + to_i64(x), ..acc },
    _ => acc,
  });

  position.position * position.depth
}

fn to_i64(x: &str) -> i64 {
  x.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
  fn input() -> Vec<String> {
    vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
      .into_iter()
      .map(String::from)
      .collect::<Vec<_>>()
  }

  #[test]
  fn position() {
    let mut input = input();
    let result = super::inner_position(&mut input);

    assert_eq!(result, 150);
  }

  #[test]
  fn position_and_aim() {
    let mut input = input();
    let result = super::inner_position_and_aim(&mut input);

    assert_eq!(result, 900);
  }
}
