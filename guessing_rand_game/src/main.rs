// 使用std lib的io套件，std為preclude的模組
use std::io;

fn main() {
    println!("Guess the random number!");

    println!("Please input your guess.");

    // 因為guess為使用者輸入的變數，必需為可變
    // String 是一個標准庫提供的字串類型，它是 UTF-8 編碼的可增長文字塊
    let mut guess = String::new();

    // & 表示這個參數是一個 引用（reference）
    // read_line 將用戶輸入放置到傳遞給它的字串中，不過它也返回一個值 io::Result
    // Result 類型是 枚舉（enumerations),
    // Result 的成員是 Ok 和 Err，
    // Ok 成員表示操作成功，內部包含成功時產生的值。
    // Err 成員則意味著操作失敗，並且包含失敗的前因後果。
    // io::Result 的實例擁有 expect 方法。
    // 如果 io::Result 實例的值是 Err，expect 會導致程式崩潰，並顯示當做參數傳遞給 expect 的資訊。
    // 如果 read_line 方法返回 Err，則可能是來源於底層操作系統錯誤的結果。
    // 如果 io::Result 實例的值是 Ok，expect 會獲取 Ok 中的值並原樣返回。
    // 如果不調用 expect，程式也能編譯，不過會出現一個警告
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
