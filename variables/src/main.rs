fn main() {
    // 不可變的變數
    let x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable `x`
    // 不可變變數只能在初始化使給值，不可再修改
    //x = 6;
    println!("The value of x is: {}", x);

    // 再次宣告為可變的變數
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定義常數
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const value {}", THREE_HOURS_IN_SECONDS);

    /* 隱藏（Shadowing）
       隱藏與將變量標記為 mut 是有區別的。
       當不小心嘗試對變量重新賦值時，如果沒有使用 let 關鍵字，就會導致編譯時錯誤。
    */
    // 定義
    let y = 5;
    // 隱藏y
    let y = y + 1;

    {
        // 隱藏y
        let y = y * 2;
        println!("內部的y值: {}", y); //12
    }
    // 外部的y不受內部y的影響
    println!("外部的y值: {}", y); // 6
}
