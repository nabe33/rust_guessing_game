use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数当てゲームだよ！");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("秘密の数字は: {}", secret_number);

    loop {
        println!("1から100までで答えだと思う数字を入力してね！");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してね！");
                continue;
            }
        };

        println!("あなたの予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ！"),
            Ordering::Greater => println!("大きすぎ！"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
