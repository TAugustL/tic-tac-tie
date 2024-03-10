use std::collections::HashMap;
use std::io;

const WIN_POS: [(i32, i32, i32); 8] = [
    (1, 2, 3), 
    (4, 5, 6), 
    (7, 8, 9), 
    (1, 4, 7), 
    (2, 5, 8), 
    (3, 6, 9), 
    (1, 5, 9), 
    (3, 5, 7)
];
const FIRST: [i32; 5] = [1, 2, 3, 4, 7];
const SECOND: [i32; 5] = [2, 4, 5, 6, 8];

fn main() {
    let mut field: String = String::from(" 1 │ 2 │ 3\n───┼───┼───\n 4 │ 5 │ 6\n───┼───┼───\n 7 │ 8 │ 9");
    let mut map: HashMap<i32, &str> = HashMap::new();

    let mut color: &str = "X";
    let mut color1: &str = "O";

    let mut xpos: Vec<i32> = Vec::new();
    let mut opos: Vec<i32> = Vec::new();

    'master: loop {
        for key in map.keys() {
            field = field.replace(&key.to_string(), map.get(key).unwrap());
        }
        println!("\n{field}");

        // GAME END
        if check_for_win(xpos.clone()) {
            println!("\nX WON!");
            break 'master;
        } else if check_for_win(opos.clone())  {
            println!("\nO WON!");
            break 'master;
        }
        let mut count: i32 = 0;
        for i in map.keys() {
            count += i;
            if count == 45 {
                println!("\nTIE!");
                break 'master;
            }
        }

        if color == "X" {println!("X: ")}
        else if color == "O" {println!("O: ")}

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input!");
        let choice: i32 = match choice.trim().parse() {
            Ok(index) => index,
            Err(err) => {eprintln!("{err}, enter valid number!"); continue},
        };

        if !check_for_valid_move(choice, &map, color) || (choice > 9 || choice < 1) {
            eprintln!("Invalid move!");
            continue;
        }

        map.entry(choice).or_insert(color);

        if color == "X" {xpos.push(choice);}
        else if color == "O" {opos.push(choice);}
        (color, color1) = (color1, color);
    }
}

fn check_for_win(index_list: Vec<i32>) -> bool {
    for i in &index_list {
        if FIRST.contains(i) {
            for j in &index_list {
                if SECOND.contains(j) {
                    for l in &index_list {
                        let tup: (i32, i32, i32) = (*i, *j, *l);
                        if WIN_POS.contains(&tup) {
                            return true;
                        }
                    }
                }
            } //Oh my god, this is ugly
        }
    }
    false
}

fn check_for_valid_move(turn: i32, map: &HashMap<i32, &str>, cur_col: &str) -> bool {
    let count: usize = map.len();
    if map.contains_key(&turn) {
        return false;
    }
    match map.get(&turn) {
        Some(value) => {
            if count > 1 && value.to_string() != cur_col.to_string() {
                return false;
            }
        }
        None => {}
    }
    true
}

