// Summarize TAP output: print pass/fail/skip counts plus failing test details.

use std::io::{self, BufRead};
use std::process::exit;

fn main() {
    let mut buffered: Vec<String> = Vec::new();
    let mut tfailed = false;
    let mut ntests: Option<i64> = None;
    let mut passed: i64 = 0;
    let mut failed: i64 = 0;
    let mut skipped: i64 = 0;

    println!("Test summary:");
    println!();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap_or_default();
        if l.starts_with("not ok") {
            buffered.push(l);
            failed += 1;
            tfailed = true;
        } else if l.starts_with("ok") {
            if l.contains("# SKIP") {
                skipped += 1;
            } else {
                passed += 1;
            }
            tfailed = false;
        } else if let Some(rest) = l.strip_prefix("1..") {
            match rest.trim().parse::<i64>() {
                Ok(n) => ntests = Some(n),
                Err(_) => {
                    eprintln!("bad 1..n line: {}", l);
                    exit(1);
                }
            }
        } else if l.starts_with('#') {
            if tfailed {
                buffered.push(format!("  {}", &l[1..]));
            }
        }
    }

    let ntests = match ntests {
        Some(n) => n,
        None => {
            eprintln!("expected a 1..n line");
            exit(1);
        }
    };

    if ntests != passed + failed + skipped {
        eprintln!(
            "{} tests did not report results",
            ntests - (passed + failed)
        );
        exit(1);
    }

    println!("pass {}", passed);
    println!("fail {}", failed);
    println!("skip {}", skipped);
    let totalln = format!("total {}", ntests);
    println!("{}", "-".repeat(totalln.len()));
    println!("{}", totalln);

    println!();
    for l in &buffered {
        println!("{}", l);
    }
}
