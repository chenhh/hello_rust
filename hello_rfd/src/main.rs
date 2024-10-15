use rfd::AsyncFileDialog;
use rfd::FileDialog;

fn main() {
    // let future = async {
    //     // let file = AsyncFileDialog::new()
    //     //     .add_filter("text", &["txt", "rs"])
    //     //     .add_filter("rust", &["rs", "toml"])
    //     //     .set_directory("/")
    //     //     .pick_file()
    //     //     .await;
    //
    //     let data = file.unwrap().read().await;
    // };
    let files = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();
}
