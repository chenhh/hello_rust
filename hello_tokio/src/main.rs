use tokio;
use tokio::{time, runtime::Runtime};
use chrono::Local;



fn main() {
    // 建立runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    //
    // // 建立帶有執行緒池的runtime
    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(8) // 8個工作執行緒
    //     .enable_io() // 可在runtime中使用非同步IO
    //     .enable_time() // 可在runtime中使用非同步計時器(timer)
    //     .build() // 建立runtime
    //     .unwrap();
    //
    // // 預設情況下(比如以上兩種方式)，建立出來的runtime都是多執行緒runtime，且沒有指定工作執行緒數量時，預設的工作執行緒數量將和CPU核數(虛擬核，即CPU執行緒數)相同。
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // std::thread::sleep(std::time::Duration::from_secs(10));

    // tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(10)
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async { });



    // let rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     println!("before sleep: {}", Local::now().format("%F %T.%3f"));
    //     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    //     println!("after sleep: {}", Local::now().format("%F %T.%3f"));
    // });

    let rt = Runtime::new().unwrap();
    let res: i32 = rt.block_on(async{
        time::sleep(time::Duration::from_secs(2)).await;
        3
    });
    println!("{}", res);  // 3
}
