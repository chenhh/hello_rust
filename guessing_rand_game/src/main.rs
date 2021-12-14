use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 不要在用戶輸入非數字時崩潰，需要忽略非數字，讓用戶可以繼續猜測
        // parse 返回一個 Result 類型，而 Result 是一個擁有 Ok 或 Err 成員的枚舉
        let guess: u32 = match guess.trim().parse() {
            // 如果 parse 能夠成功的將字串轉換為一個數字，它會返回一個包含結果數字的 Ok
            Ok(num) => num,
            // 如果 parse 不 能將字串轉換為一個數字，它會返回一個包含更多錯誤資訊的 Err
            Err(_) => continue,
        };

        println!("猜數字: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小!"),
            Ordering::Greater => println!("太大!"),
            Ordering::Equal => {
                println!("你贏了!");
                break;
            }
        }
    }
}
