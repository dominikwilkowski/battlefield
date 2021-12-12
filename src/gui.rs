extern crate termion;

use crate::config;
use crate::game;
use crate::Cell;

use termion::color;
use Cell::{Crosshair, Damage, Empty, Placeholder, Ship, ShipOne, ShipThree, ShipTwo, Shot};

pub fn get_header() -> String {
	let reset = color::Fg(color::Reset);
	let logo1 = format!("{}           ┏┓         ┏┓   ┏┓  ┏┓            ┏┓   ┏┓{}\r\n", color::Fg(color::White), reset);
	let logo2 =
		format!("{}           ┃┗━┓ ┏━━┓ ┏┛┗┓ ┏┛┗┓ ┃┃  ┏━━┓ ┏━━┓ ┃┗━┓ ┗┛ ┏━━┓\r\n{}", color::Fg(color::White), reset);
	let logo3 =
		format!("{}           ┃┏┓┃ ┃┏┓┃ ┗┓┏┛ ┗┓┏┛ ┃┃  ┃┃━┫ ┃━━┫ ┃┏┓┃ ┏┓ ┃┏┓┃\r\n{}", color::Fg(color::White), reset);
	let logo4 = format!(
		"{}           ┃┗┛┃ ┃┏┓┃  ┃┗┓  ┃┗┓ ┃┗┓ ┃┃━┫ ┣━━┃ ┃┃┃┃ ┃┃ ┃┗┛┃\r\n{}",
		color::Fg(color::Rgb(180, 209, 245)),
		reset
	);
	let logo5 = format!(
		"{}           ┗━━┛ ┗┛┗┛  ┗━┛  ┗━┛ ┗━┛ ┗━━┛ ┗━━┛ ┗┛┗┛ ┗┛ ┃┏━┛\r\n{}",
		color::Fg(color::Rgb(93, 156, 233)),
		reset
	);
	let logo6 = format!(
		"{}                                            {:>8} ┗┛{}",
		color::Fg(color::Rgb(93, 156, 233)),
		config::VERSION,
		reset
	);

	format!("{}{}{}{}{}{}\r\n\r\n", logo1, logo2, logo3, logo4, logo5, logo6)
}

pub fn get_score(board_me: [[Cell; 10]; 10], board_ai: [[Cell; 10]; 10], show_score: bool) -> String {
	let score_me = if show_score {
		game::get_score(&board_ai)
	} else {
		String::from("--")
	};
	let score_ai = if show_score {
		game::get_score(&board_me)
	} else {
		String::from("--")
	};

	format!(
		"ME                     {open}SCORE: {score_me}{close}   ║  AI                     {open}SCORE: {score_ai}{close}\r\n",
		open=color::Fg(color::Magenta),
		close=color::Fg(color::White),
		score_me=score_me,
		score_ai=score_ai,
	)
}

// return one line of a board and interpret states to visual styles
fn get_board_row(
	board_row: &[Cell; 10],
	y: usize,
	pos_x: usize,
	pos_y: usize,
	cell: Cell,
	show_position: bool,
) -> String {
	let mut output = String::new();

	for (x, item) in board_row.iter().enumerate() {
		match (item, x, y) {
			(_, this_pos_x, this_pos_y) if this_pos_x == pos_x && this_pos_y == pos_y && show_position => {
				match (board_row[x], cell) {
					(Empty, Crosshair) => {
						output += &format!("{}{}{}", color::Fg(color::Green), config::CROSSHAIR, color::Fg(color::White))
					}
					(Empty, _) => output += &format!("{}{}{}", color::Fg(color::Green), config::SHIP, color::Fg(color::White)),
					(_, Crosshair) => {
						output += &format!("{}{}{}", color::Fg(color::Red), config::CROSSHAIR, color::Fg(color::White))
					}
					(_, _) => output += &format!("{}{}{}", color::Fg(color::Red), config::SHIP, color::Fg(color::White)),
				}
			}
			(Placeholder, _, _) => {
				output += &format!("{}{}{}", color::Fg(color::Green), config::SHIP, color::Fg(color::White))
			}
			(Shot, _, _) => output += config::SHOT,
			(Ship, _, _) | (ShipOne(_), _, _) | (ShipTwo(_), _, _) | (ShipThree(_), _, _) => output += config::SHIP,
			(Damage, _, _) => output += config::DAMAGE,
			(_, _, _) => match x % 2 {
				0 => output += &format!("{}{}{}", color::Fg(color::Rgb(100, 100, 100)), config::EMPTY, color::Fg(color::Reset)),
				_ => output += config::EMPTY,
			},
		}
	}

	output
}

pub fn get_board(
	board_me: &[[Cell; 10]; 10],
	board_ai: &[[Cell; 10]; 10],
	pos_x: usize,
	pos_y: usize,
	is_first_round: bool,
) -> String {
	let coord_top = "   1  2  3  4  5  6  7  8  9  10   ║     1  2  3  4  5  6  7  8  9  10";
	let frame_top = " ┌──────────────────────────────┐  ║   ┌──────────────────────────────┐";
	let coord_dict = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
	let frame_bottom = " └──────────────────────────────┘  ║   └──────────────────────────────┘";

	let mut output = format!("{}{}\r\n{}\r\n", color::Fg(color::White), coord_top, frame_top);
	for row in 0..10 {
		output += coord_dict[row];
		output += "│";
		output += &get_board_row(&board_me[row], row, pos_x, pos_y, Empty, false);
		output += "│  ║  ";
		output += coord_dict[row];
		output += "│";
		output += &get_board_row(&board_ai[row], row, pos_x, pos_y, Crosshair, !is_first_round);
		output += "│\r\n";
	}
	output += frame_bottom;
	output += "\r\n\r\n";
	output += &format!("{}", color::Fg(color::Reset));

	output
}

pub fn get_coord(pos_x: usize, pos_y: usize) -> String {
	let alphabet = ('A'..='J')
		.filter_map(|c| {
			let c = c as char;
			if c.is_alphabetic() {
				Some(c)
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	format!("{}{}", alphabet[pos_y], pos_x + 1)
}

#[test]
fn get_coord_works() {
	assert_eq!(get_coord(0, 0), String::from("A1"));
	assert_eq!(get_coord(1, 0), String::from("A2"));
	assert_eq!(get_coord(0, 1), String::from("B1"));
	assert_eq!(get_coord(9, 0), String::from("A10"));
	assert_eq!(get_coord(0, 9), String::from("J1"));
	assert_eq!(get_coord(9, 9), String::from("J10"));
}

pub fn get_round1_instructions() -> String {
	format!(
		"\r\n{}PLACING ROUND - Place your ships strategically on your map{}\r\n\r\n[←↑↓→] position ║ [r] rotate ║ [enter] place ║ [del] restart ║ [q] quit\r\n\r\n",
		color::Fg(color::Green),
		color::Fg(color::Reset),
	)
}

pub fn get_round2_instructions() -> String {
	format!(
		"\r\n{}PLAY - Hit all your opponents ships and reach a score of 10 to win{}\r\n\r\n[←↑↓→] position ║ [enter] shoot ║ [q] quit\r\n\r\n",
		color::Fg(color::Green),
		color::Fg(color::Reset),
	)
}

pub fn get_good_bye_msg(winner: bool) -> String {
	let mut result = String::new();
	if winner {
		result += "Congrats!\r\n";
		result += &format!("{}", color::Fg(color::Green));
		result += " ┏┓ ┏┓ ┏━━┓ ┏┓┏┓    ┏┓┏┓┏┓ ┏━━┓ ┏━┓\r\n";
		result += " ┃┗━┛┃ ┃┏┓┃ ┃┃┃┃    ┃┗┛┗┛┃ ┃┏┓┃ ┃┏┓┓\r\n";
		result += " ┗━┓┏┛ ┃┗┛┃ ┃┗┛┃    ┗┓┏┓┏┛ ┃┗┛┃ ┃┃┃┃\r\n";
		result += " ┗━━┛  ┗━━┛ ┗━━┛     ┗┛┗┛  ┗━━┛ ┗┛┗┛\r\n";
		result += &format!("{}", color::Fg(color::Reset));
	} else {
		result += &format!("{}", color::Fg(color::Red));
		result += "                    ┏┓             ┏┓\r\n";
		result += " ┏┓ ┏┓ ┏━━┓ ┏┓┏┓    ┃┃  ┏━━┓ ┏━━┓ ┏┛┗┓\r\n";
		result += " ┃┗━┛┃ ┃┏┓┃ ┃┃┃┃    ┃┃  ┃┏┓┃ ┃━━┫ ┗┓┏┛\r\n";
		result += " ┗━┓┏┛ ┃┗┛┃ ┃┗┛┃    ┃┗┓ ┃┗┛┃ ┣━━┃  ┃┗┓\r\n";
		result += " ┗━━┛  ┗━━┛ ┗━━┛    ┗━┛ ┗━━┛ ┗━━┛  ┗━┛\r\n";
		result += &format!("{}", color::Fg(color::Reset));
		result += "Try again soon.\r\n";
	}

	result
}
