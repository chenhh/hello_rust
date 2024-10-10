pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 這個加法函式寫得很差，本例中我們會使它失敗。
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    // 注意這個慣用法：在 tests 模組中，從外部作用域匯入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // 這個斷言會導致測試失敗。注意私有的函式也可以被測試！
        assert_eq!(bad_add(1, 2), 3);
    }

     #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}

fn main(){
    let a = add(3, 4);
    println!("add value:{a}");

    let b = [1,2,3,4,5];
    println!("{b:?}");

    b.iter()..for_each(|&x| println!("{x}"));
}