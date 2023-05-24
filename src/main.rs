use std::io::Write;

use randomizer::Randomizer;

const BAD_SYMBOL: &str = "O";
const ULTRA_SYMBOL: &str = "$";
const BET: i32 = 10;
const BANK: i32 = 10000;

fn main() {
    start_casino();
}

fn start_casino() {
    let mut r = Randomizer::new();
    r.add(BAD_SYMBOL, 40.);
    r.add("♠", 50.);
    r.add("♥", 50.);
    r.add("♣", 50.);
    r.add("♦", 50.);
    r.add(ULTRA_SYMBOL, 20.);

    let mut bank = BANK;
    let mut bet: i32 = BET;

    loop {
        let mut bet_str = String::new();
        println!("$ Casino777 (Bank: ${}) $", bank);
        print!("Enter bet (or enter for {}) >>> ", bet);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut bet_str).expect("Error!");
        bet_str = bet_str.replace('\n', "");
        match bet_str.parse::<i32>() {
            Ok(n) => bet = n,
            _ => {}
        }

        bank -= bet;
        println!();
        let mut win: Vec<Vec<String>> = Vec::with_capacity(3);
        for _col in 0..3 {
            let mut row_vec = Vec::with_capacity(3);
            print!("|");
            std::io::stdout().flush().unwrap();
            for _row in 0..3 {
                let rand_item = r.get_random_item();
                row_vec.push(rand_item.to_string());
                print!("{}|", rand_item);
                std::io::stdout().flush().unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            println!();
            win.push(row_vec);
        }
        let multi = validate_win(&win, &bet);
        if multi != 0 {
            bank += multi * bet;
        }
    }
}

#[allow(unused_assignments)]
fn validate_win(arr: &[Vec<String>], bet: &i32) -> i32 {
    let mut win_multi: i32 = 0;
    for row in arr {
        if row.iter().all(|item| item == &row[0] && item != BAD_SYMBOL) {
            if win_multi == 0 {
                win_multi = 2
            } else {
                win_multi *= 2
            };
        }
    }
    if arr[0][0] == arr[1][1] && arr[1][1] == arr[2][2] && arr[0][0] != BAD_SYMBOL {
        if win_multi == 0 {
            win_multi = 2
        } else {
            win_multi *= 2
        };
    }
    if arr[2][0] == arr[1][1] && arr[1][1] == arr[0][2] && arr[2][0] != BAD_SYMBOL {
        if win_multi == 0 {
            win_multi = 2
        } else {
            win_multi *= 2
        };
    }
    if arr.iter().fold(0, |res, item| {
        res + item
            .iter()
            .fold(0, |res, i| if i == ULTRA_SYMBOL { res + 1 } else { res })
    }) >= 3
    {
        if win_multi == 0 {
            win_multi = 100
        } else {
            win_multi *= 100
        };
    }

    println!(
        "\nYour prize X{win_multi} (${})\n\n=============================\n",
        bet * win_multi
    );
    win_multi
}
