mod common;

use common::InputConversions;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Default, Clone)]
struct Bingo {
  draws: Vec<u128>,
  boards: Vec<Board>,
}

#[derive(Debug, Default, Clone)]
struct Board {
  won: bool,
  numbers: BoardSpec,
}

type BoardSpec = Vec<(u128, bool)>;
type BoardSpecRef<'a> = &'a [(u128, bool)];

#[common::aoc(day = 4, name = "Giant Squid")]
fn main() {
  let input = common::read_lined_file(4);

  setup(input)
}

fn setup(input: Vec<String>) -> Bingo {
  let mut bingo = Bingo {
    draws: input.iter().take(1).collect::<Vec<_>>().get(0).unwrap().split(',').to::<u128>(),
    ..Default::default()
  };

  let mut buffer: BoardSpec = Vec::new();

  for row in input.iter().skip(1) {
    if buffer.len() == BOARD_SIZE * BOARD_SIZE {
      bingo.boards.push(Board {
        numbers: buffer,
        ..Default::default()
      });

      buffer = Vec::new();
    }

    let mut row: Vec<_> = row.split_whitespace().map(|n| (n.parse::<u128>().unwrap(), false)).collect();

    buffer.append(&mut row);
  }

  bingo
}

#[common::task(part = 1, name = "Bingo score")]
fn winning_board(input: &mut Bingo) {
  for draw in &input.draws.clone() {
    for board in &mut input.boards {
      if let Some((index, (_, found))) = board.numbers.iter_mut().enumerate().find(|(_, (number, _))| number == draw) {
        *found = true;

        if wins(&board.numbers, index) {
          let sum: u128 = board.numbers.iter().filter(|(_, drawn)| !drawn).map(|(number, _)| number).sum();

          return sum * draw;
        }
      }
    }
  }

  0
}

#[common::task(part = 2, name = "Last winning board score")]
fn last_winning_board(input: &mut Bingo) {
  let (mut winning_board, mut winning_draw) = (Board::default(), 0);

  for draw in &input.draws.clone() {
    for board in &mut input.boards {
      if board.won {
        continue;
      };

      if let Some((index, (_, found))) = board.numbers.iter_mut().enumerate().find(|(_, (number, _))| number == draw) {
        *found = true;

        if wins(&board.numbers, index) {
          board.won = true;

          winning_board = board.clone();
          winning_draw = *draw;
        }
      }
    }
  }

  let sum: u128 = winning_board.numbers.iter().filter(|(_, drawn)| !drawn).map(|(number, _)| number).sum();

  sum * winning_draw
}

fn wins(board: BoardSpecRef, index: usize) -> bool {
  for batch in winnable_indexes(index) {
    if board.iter().enumerate().filter(|(index, _)| batch.contains(index)).all(|(_, (_, drawn))| *drawn) {
      return true;
    }
  }

  false
}

fn winnable_indexes(index: usize) -> [[usize; BOARD_SIZE]; 2] {
  let row = index - (index % BOARD_SIZE);
  let col = index - ((index / BOARD_SIZE) * BOARD_SIZE);

  [
    (0..BOARD_SIZE).map(|offset| row + offset).collect::<Vec<usize>>().try_into().unwrap(),
    (0..BOARD_SIZE).map(|offset| col + (offset * BOARD_SIZE)).collect::<Vec<usize>>().try_into().unwrap(),
  ]
}

#[cfg(test)]
mod tests {
  use super::{Bingo, Board};

  fn input() -> Bingo {
    let draws = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
    let boards = vec![
      vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19],
      vec![3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6],
      vec![14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7],
    ];

    super::Bingo {
      draws,
      boards: boards
        .into_iter()
        .map(|board| Board {
          won: false,
          numbers: board.into_iter().map(|n| (n, false)).collect(),
        })
        .collect(),
    }
  }

  #[test]
  fn winning_board() {
    let mut input = input();
    let result = super::inner_winning_board(&mut input);

    assert_eq!(result, 4512);
  }

  #[test]
  fn last_winning_board() {
    let mut input = input();
    let result = super::inner_last_winning_board(&mut input);

    assert_eq!(result, 1924);
  }
}
