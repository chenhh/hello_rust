fn main() {
    let s = "attachment; filename=\"Ch1_2024-01-31_13-45-49_12_2024-01-31_13-46-01_.mp3\"";
    println!("{s}");
     // 使用split找到filename開頭的位置
     if let Some(part) = s.split("filename=\"").nth(1) {
        // 再次使用split找到結束引號的位置
        if let Some(filename) = part.split('"').next() {
            println!("{filename}");
        }
    }
    
}
