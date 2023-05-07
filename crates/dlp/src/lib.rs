#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::Result as IoResult;
use std::process::Command;

use model::JsonOutput;
use serde_json::Value;

mod model;

pub struct YoutubeQuery {
    cmd: Command,
}

impl Default for YoutubeQuery {
    fn default() -> Self {
        let mut cmd = Command::new("yt-dlp");
        cmd.arg("-J");

        Self { cmd }
    }
}

impl YoutubeQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&mut self, url: &str) -> Result<JsonOutput, ()> {
        self.cmd.arg(url);

        let output = self.cmd.output().unwrap().stdout;
        let value: Value = serde_json::from_slice(&output).unwrap();
        let json: JsonOutput = serde_json::from_value(value).unwrap();

        Ok(json)
    }
}

pub struct YoutubeDlp {
    cmd: Command,
}

impl Default for YoutubeDlp {
    fn default() -> Self {
        let cmd = Command::new("yt-dlp");

        Self { cmd }
    }
}

impl YoutubeDlp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download(mut self, url: &str) -> IoResult<()> {
        self.cmd.arg(url);

        let output = dbg!(self.cmd).output()?;

        dbg!(output);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command() {
        let dlp = YoutubeDlp::default();

        assert_eq!(dlp.cmd.get_program().to_str().unwrap(), "yt-dlp");
    }
}
