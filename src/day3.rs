mod common;

#[common::aoc(day = 3, name = "Binary Diagnostic")]
fn main() {
  common::read_lined_file(3)
}

#[common::task(part = 1, name = "Power consumption")]
fn power_consumption(input: &[String]) {
  let cardinalities = get_cardinalities(input);

  let (gamma, epsilon) = cardinalities.iter().fold((String::new(), String::new()), |mut values, cardinality| {
    values.0.push(if cardinality < &0.5 { '0' } else { '1' });
    values.1.push(if cardinality > &0.5 { '0' } else { '1' });

    values
  });

  let (gamma, epsilon) = (u64::from_str_radix(&gamma, 2).unwrap(), u64::from_str_radix(&epsilon, 2).unwrap());

  gamma * epsilon
}

#[common::task(part = 2, name = "Life support rating")]
fn life_support_rating(input: &[String]) {
  let length = input.iter().peekable().next().unwrap().len();

  let o2 = compute(length, input.to_vec(), |cardinalities, index, value| {
    let char = value.chars().nth(index).unwrap();

    if cardinalities[index] < 0.5 && char == '0' {
      true
    } else {
      cardinalities[index] >= 0.5 && char == '1'
    }
  });

  let co2 = compute(length, input.to_vec(), |cardinalities, index, value| {
    let char = value.chars().nth(index).unwrap();

    if cardinalities[index] >= 0.5 && char == '0' {
      true
    } else {
      cardinalities[index] < 0.5 && char == '1'
    }
  });

  let o2 = u64::from_str_radix(o2.get(0).unwrap(), 2).unwrap();
  let co2 = u64::from_str_radix(co2.get(0).unwrap(), 2).unwrap();

  o2 * co2
}

fn compute(size: usize, mut input: Vec<String>, filter: fn(&Vec<f64>, usize, &String) -> bool) -> Vec<String> {
  for index in 0..size {
    if input.len() == 1 {
      break;
    }

    let cardinalities = get_cardinalities(&input);

    input = input.into_iter().filter(|value| filter(&cardinalities, index, value)).collect();
  }

  input
}

fn get_cardinalities(input: &[String]) -> Vec<f64> {
  let length = input.iter().peekable().next().unwrap().len();

  input
    .iter()
    .fold(vec![0u128; length], |mut cardinalities, reading| {
      for (index, bit) in reading.chars().enumerate() {
        *cardinalities.get_mut(index).unwrap() += bit.to_string().parse::<u128>().unwrap();
      }

      cardinalities
    })
    .into_iter()
    .map(|count| count as f64 / input.len() as f64)
    .collect()
}

#[cfg(test)]
mod tests {
  fn input() -> Vec<String> {
    vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
      .into_iter()
      .map(String::from)
      .collect::<Vec<_>>()
  }

  #[test]
  fn power_consumption() {
    let mut input = input();
    let result = super::inner_power_consumption(&mut input);

    assert_eq!(result, 198);
  }

  #[test]
  fn life_support_rating() {
    let mut input = input();
    let result = super::inner_life_support_rating(&mut input);

    assert_eq!(result, 230);
  }
}
