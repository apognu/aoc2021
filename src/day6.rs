mod common;

const TIMER: u8 = 7;
const SPAWN_COST: u8 = 2;

#[common::aoc(day = 6, name = "Lanternfish")]
fn main() {
  let input = common::read_split_file(6, ",");

  setup(input)
}

fn setup(input: Vec<String>) -> Vec<usize> {
  input.iter().fold(vec![0; (TIMER + SPAWN_COST + 1) as usize], |mut acc, timer| {
    *acc.get_mut(timer.parse::<usize>().unwrap()).unwrap() += 1;

    acc
  })
}

#[common::task(part = 1, name = "Fish population after 80 days")]
fn lanternfish_simulation_80_days(input: &[usize]) {
  get_fish_population(&mut input.to_vec(), 80)
}

#[common::task(part = 2, name = "Fish population after 256 days")]
fn lanternfish_simulation_256_days(input: &[usize]) {
  get_fish_population(&mut input.to_vec(), 256)
}

fn get_fish_population(counts: &mut Vec<usize>, days: u64) -> i128 {
  for _ in 1..=days {
    let spawns = *counts.get(0).unwrap();

    *counts.get_mut(0).unwrap() = 0;
    *counts.get_mut(TIMER as usize).unwrap() += spawns;
    *counts.get_mut((TIMER + SPAWN_COST) as usize).unwrap() += spawns;

    counts.rotate_left(1);
  }

  counts.iter().map(|x| *x as i128).sum::<i128>()
}

#[cfg(test)]
mod tests {
  fn input() -> Vec<usize> {
    let input = vec!["3", "4", "3", "1", "2"].into_iter().map(String::from).collect::<Vec<_>>();

    super::setup(input)
  }

  #[test]
  fn lanternfish_simulation_80_days() {
    let mut input = input();
    let result = super::inner_lanternfish_simulation_80_days(&mut input);

    assert_eq!(result, 5934);
  }

  #[test]
  fn lanternfish_simulation_256_days() {
    let mut input = input();
    let result = super::inner_lanternfish_simulation_256_days(&mut input);

    assert_eq!(result, 26984457539);
  }
}
