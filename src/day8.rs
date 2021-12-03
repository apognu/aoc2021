mod common;

use std::collections::{HashMap, HashSet};

type Pattern = HashSet<char>;

#[common::aoc(day = 5, name = "Seven Segment Search")]
fn main() {
  let input = common::read_lined_file(8);

  setup(&input)
}

fn setup(input: &[String]) -> Vec<(Vec<Pattern>, Vec<Pattern>)> {
  input
    .into_iter()
    .map(|spec| {
      spec
        .split(" | ")
        .map(|side| side.split_whitespace().map(|pattern| pattern.chars().collect::<Pattern>()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
    })
    .map(|spec| {
      let signals = spec.get(0).unwrap().clone();
      let codes = spec.get(1).unwrap().clone();

      (signals, codes)
    })
    .collect::<Vec<_>>()
}

#[common::task(part = 1, name = "Easy numbers count")]
fn easy_numbers_count(input: &mut [(Vec<Pattern>, Vec<Pattern>)]) {
  let output = input
    .iter()
    .flat_map(|(_, digits)| {
      digits.iter().map(|number| match number.len() {
        2 | 4 | 3 | 7 => 1,
        _ => 0,
      })
    })
    .sum::<i128>();

  output
}

#[common::task(part = 2, name = "Output value")]
fn output_value(input: &mut [(Vec<Pattern>, Vec<Pattern>)]) {
  let digits = input
    .iter()
    .map(|(pattern, spec)| {
      let patterns = solve_patterns(pattern);
      let digits = spec.iter().map(|digit| patterns.iter().find(|p| p.1 == digit).unwrap().0).cloned().collect::<Vec<_>>();

      digits.get(0).unwrap() * 1000 + digits.get(1).unwrap() * 100 + digits.get(2).unwrap() * 10 + digits.get(3).unwrap()
    })
    .collect::<Vec<_>>();

  digits.iter().sum::<usize>()
}

fn solve_patterns(input: &[Pattern]) -> HashMap<usize, Pattern> {
  let solvers: [(usize, Box<dyn Fn(&HashMap<usize, Pattern>, &Pattern) -> bool>); 10] = [
    (6, Box::new(|patterns, p| p != get(patterns, 9) && p.intersection(get(patterns, 1)).count() == 2)),
    (2, Box::new(|_, _| true)),
    (5, Box::new(|patterns, p| p != get(patterns, 3) && p != get(patterns, 5))),
    (5, Box::new(|patterns, p| p.intersection(get(patterns, 1)).count() == 2)),
    (4, Box::new(|_, _| true)),
    (5, Box::new(|patterns, p| p != get(patterns, 3) && p.intersection(get(patterns, 4)).count() == 3)),
    (6, Box::new(|patterns, p| p != get(patterns, 9) && p != get(patterns, 0))),
    (3, Box::new(|_, _| true)),
    (7, Box::new(|_, _| true)),
    (6, Box::new(|patterns, p| p.intersection(get(patterns, 3)).count() == 5)),
  ];

  [1, 7, 4, 8, 3, 5, 2, 9, 0, 6].iter().fold(HashMap::<usize, Pattern>::new(), |mut patterns, digit| {
    let (length, predicate) = &solvers[*digit];
    let pattern = input.iter().find(|p| p.len() == *length && predicate(&patterns, p)).unwrap();

    patterns.insert(*digit, pattern.clone());
    patterns
  })
}

fn get(patterns: &HashMap<usize, Pattern>, key: usize) -> &Pattern {
  patterns.get(&key).unwrap()
}

#[cfg(test)]
mod tests {
  use super::Pattern;

  fn input() -> Vec<(Vec<Pattern>, Vec<Pattern>)> {
    let input = vec![
      "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
      "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
      "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
      "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
      "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
      "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
      "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
      "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
      "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
      "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();

    super::setup(&input)
  }

  #[test]
  fn easy_numbers_count() {
    let mut input = input();
    let result = super::inner_easy_numbers_count(&mut input);

    assert_eq!(result, 26);
  }

  #[test]
  fn output_value() {
    let mut input = input();
    let result = super::inner_output_value(&mut input);

    assert_eq!(result, 61229);
  }
}
