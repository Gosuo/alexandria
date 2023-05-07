use dlp::{YoutubeDlp, YoutubeQuery};

fn main() {
    println!("Hello, downloading: https://www.youtube.com/watch?v=jNQXAC9IVRw");
    let query = YoutubeQuery::new()
        .query("https://www.youtube.com/watch?v=jNQXAC9IVRw")
        .unwrap();

    println!("Query:\n{:?}", query);
}
