use std::path::Path;

use dlp::simple_download;

fn main() {
    println!("Hello, downloading: https://www.youtube.com/watch?v=jNQXAC9IVRw");
    simple_download(
        "https://www.youtube.com/watch?v=jNQXAC9IVRw",
        Path::new("./videos"),
    )
    .expect("Unable to download video");
}
