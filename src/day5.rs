mod common;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Line {
  start: (i128, i128),
  end: (i128, i128),
}

#[common::aoc(day = 5, name = "Hydrothermal Venture")]
fn main() {
  let input = common::read_lined_file(5);

  setup(&input)
}

fn setup(input: &[String]) -> Vec<Line> {
  input
    .iter()
    .map(|line| {
      let line = line
        .split(" -> ")
        .map(|coords| {
          let coords = coords.split(',').collect::<Vec<_>>();

          (coords.get(0).unwrap().parse::<i128>().unwrap(), coords.get(1).unwrap().parse::<i128>().unwrap())
        })
        .collect::<Vec<_>>();

      Line {
        start: *line.get(0).unwrap(),
        end: *line.get(1).unwrap(),
      }
    })
    .collect::<Vec<_>>()
}

#[common::task(part = 1, name = "Dangerous areas")]
fn dangerous_areas(input: &[Line]) {
  input
    .iter()
    .filter_map(trace_line)
    .flatten()
    .map(|coords| (coords, coords))
    .into_group_map()
    .into_iter()
    .filter(|(_, points)| points.len() > 1)
    .count()
}

#[common::task(part = 2, name = "All dangerous areas")]
fn dangerous_areas_with_diagonals(input: &[Line]) {
  input
    .iter()
    .map(trace_line_with_diagonals)
    .flatten()
    .map(|coords| (coords, coords))
    .into_group_map()
    .into_iter()
    .filter(|(_, points)| points.len() > 1)
    .count()
}

fn trace_line(line: &Line) -> Option<Vec<(i128, i128)>> {
  if line.start.0 == line.end.0 {
    let min = line.start.1.min(line.end.1);
    let max = line.start.1.max(line.end.1);

    return Some((min..=max).map(|offset| (line.start.0, offset)).collect::<Vec<_>>());
  }

  if line.start.1 == line.end.1 {
    let min = line.start.0.min(line.end.0);
    let max = line.start.0.max(line.end.0);

    return Some((min..=max).map(|offset| (offset, line.start.1)).collect::<Vec<_>>());
  }

  None
}

fn trace_line_with_diagonals(line: &Line) -> Vec<(i128, i128)> {
  if let Some(points) = trace_line(line) {
    return points;
  }

  let (min_x, max_x, start_y, slope) = if line.start.0 < line.end.0 {
    if line.start.1 < line.end.1 {
      (line.start.0, line.end.0, line.start.1, 1)
    } else {
      (line.start.0, line.end.0, line.start.1, -1)
    }
  } else if line.start.1 > line.end.1 {
    (line.end.0, line.start.0, line.end.1, 1)
  } else {
    (line.end.0, line.start.0, line.end.1, -1)
  };

  (min_x..=max_x).enumerate().map(|(index, offset)| (offset, (start_y + (index as i128 * slope)))).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
  use super::Line;

  fn input() -> Vec<Line> {
    let raw = vec![
      "0,9 -> 5,9",
      "8,0 -> 0,8",
      "9,4 -> 3,4",
      "2,2 -> 2,1",
      "7,0 -> 7,4",
      "6,4 -> 2,0",
      "0,9 -> 2,9",
      "3,4 -> 1,4",
      "0,0 -> 8,8",
      "5,5 -> 8,2",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    super::setup(&raw)
  }

  #[test]
  fn dangerous_areas() {
    let mut input = input();
    let result = super::inner_dangerous_areas(&mut input);

    assert_eq!(result, 5);
  }

  #[test]
  fn dangerous_areas_wit_diagonals() {
    let mut input = input();
    let result = super::inner_dangerous_areas_with_diagonals(&mut input);

    assert_eq!(result, 12);
  }
}
