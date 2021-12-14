fn main() {
    // if_condition();
    // if_as_setter();
    // data_type_consistent();
    for_loop();
}

fn if_condition() {
    let number = 3;

    // 必須使用邏輯判斷式，而不可用if number這種自動轉型的語法
    if number != 0 {
        println!("number was three");
    }
}

fn if_as_setter() {
    let n = 10;
    let x: i32 = if n >= 5 { 1 } else { 10 };
    //--------------- ------^ -------- ^
    //------------------- 這兩個地方不要加分號, 做為expression
    println!("{}", x);
}

fn data_type_consistent() {
    let condition = true;

    let number = if condition {
        5
    } else {
        //"six"
        6
    };

    println!("The value of number is: {}", number);
}

fn for_loop() {
    let array = [1, 2, 3, 4, 5];
    for val in array {
        println!("The number is {}", val);
    }

    for val in array.iter() {
        println!("The number is {}", val);
    }
    // 5, 4, 3, 2, 1
    for val in array.iter().rev() {
        println!("The number is {}", val);
    }
}
