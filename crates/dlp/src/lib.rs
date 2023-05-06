#![allow(dead_code)]
#![allow(unused_variables)]

use std::process::Command;

pub struct YtDlp {
    cmd: Command,
}

impl Default for YtDlp {
    fn default() -> Self {
        Self {
            cmd: Command::new("yt-dlp"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command() {
        let dlp = YtDlp::default();

        assert_eq!(dlp.cmd.get_program().to_str().unwrap(), "yt-dlp");
    }
}
