use std::io::BufRead;
use std::process::Command;

static TEST_PROG: &str = include_str!("test_progs/main.cpp");

async fn self_test(p: String) -> Option<String> {
    // assuming `g++ (GCC) 15.2.1 20251112`
    Some(
        Command::new(p)
            .args(["--version"])
            .output()
            .ok()?
            .stdout
            .lines()
            .next()?
            .ok()?
            .split_whitespace()
            .nth(2)?
            .to_string(),
    )
}

struct Submission;

// #[executor(bin = "gcc", default_compile_args = ["-std=c++11", "-Wall"], self_test = self_test ty = "Compiled")]
struct Gcc;
