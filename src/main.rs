mod caculate;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use caculate::for_loop;

/**
 * Rust-example:猜数字游戏
 * @date 2020-05-17
 * @author fengxinzi
 */

fn main() {
    
    for_loop();
    
}


fn guess_game() {

    println!("just guess game");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
      
        println!("please type your guess number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
                continue;
            }
        };

        println!("your guess num is:{}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("sorry! it's smaller, please once agagin"),
            Ordering::Greater => println!("sorry! it's bigger，please once agagin"),
            Ordering::Equal => {
                println!("congratuations!you win");
                break;
            }
        }
    }
}