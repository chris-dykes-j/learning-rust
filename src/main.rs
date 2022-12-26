use std::io;
use rand::prelude::SliceRandom;
use rand::{thread_rng};

fn main() {
    println!("Tic Tac Toe");
    println!("You are x\n");
    let mut vec: Vec<&str> = vec![" - "; 9];
    while !is_finished(&vec) {
        display_board(&vec);
        choose_move(&mut vec);
        computer_move(&mut vec);
    }
    println!("Finish\n");
    display_board(&vec);
}

fn display_board(vec: &Vec<&str>) {
    println!("{}|{}|{}", vec[0], vec[1], vec[2]);
    println!("{}|{}|{}", vec[3], vec[4], vec[5]);
    println!("{}|{}|{}\n", vec[6], vec[7], vec[8]);
}

fn choose_move(vec: &mut Vec<&str>) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failure");

    let trimmed = input.trim();

    match trimmed.parse::<usize>() {
        Ok(i) => {
            if i < 9 && vec[i] != " o " {
                vec[i] = " x ";
            }
            else {
                println!("NO\n");
            }
        },
        Err(..) => println!("Error"),
    }
}

fn computer_move(vec: &mut Vec<&str>) {
    let options: Vec<usize> =  make_options(&vec);
    let mut rng = thread_rng();
    let choice = options.choose(&mut rng);
    match choice {
        None => {},
        Some(i) => vec[*i] = " o ",
    }
}

fn make_options(vec: &Vec<&str>) -> Vec<usize> {
    let mut options: Vec<usize> = vec![];
    for (i, k) in vec.iter().enumerate() {
        if k == &" - " {
            options.push(i);
        }
    }
    options
}

fn is_finished(vec: &Vec<&str>) -> bool {
    horizontal_win(&vec) || vertical_win(&vec) || diagonal_win(&vec) || is_full(&vec)
}

fn is_full(vec: &Vec<&str>) -> bool {
    !vec.contains(&" - ")
}

fn horizontal_win(vec: &Vec<&str>) -> bool {
    if vec[0] == " o " && vec[1] == " o " && vec[2] == " o " {
        return true
    }
    if vec[3] == " o " && vec[4] == " o " && vec[5] == " o " {
        return true
    }
    if vec[6] == " o " && vec[7] == " o " && vec[8] == " o " {
        return true
    }
    if vec[0] == " x " && vec[1] == " x " && vec[2] == " x " {
        return true
    }
    if vec[3] == " x " && vec[4] == " x " && vec[5] == " x " {
        return true
    }
    if vec[6] == " x " && vec[7] == " x " && vec[8] == " x " {
        return true
    }
    false
}

fn vertical_win(vec: &Vec<&str>) -> bool {
    if vec[0] == " o " && vec[3] == " o " && vec[6] == " o " {
        return true
    }
    if vec[1] == " o " && vec[4] == " o " && vec[7] == " o " {
        return true
    }
    if vec[2] == " o " && vec[5] == " o " && vec[8] == " o " {
        return true
    }
    if vec[0] == " x " && vec[3] == " x " && vec[6] == " x " {
        return true
    }
    if vec[1] == " x " && vec[4] == " x " && vec[7] == " x " {
        return true
    }
    if vec[2] == " x " && vec[5] == " x " && vec[8] == " x " {
        return true
    }
    false
}

fn diagonal_win(vec: &Vec<&str>) -> bool {
    if vec[0] == " o " && vec[4] == " o " && vec[8] == " o " {
        return true
    }
    if vec[2] == " o " && vec[4] == " o " && vec[6] == " o " {
        return true
    }
    if vec[0] == " x " && vec[4] == " x " && vec[8] == " x " {
        return true
    }
    if vec[2] == " x " && vec[4] == " x " && vec[6] == " x " {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computer_chooses_properly() {
        let mut board:Vec<&str> = vec![" x "; 9];
        board[8] = " - ";
        // assert!(Ok(computer_move(&mut board))) // WIP
    }
}