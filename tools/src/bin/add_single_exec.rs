// Add a file to the single-exec suite under the next free number,
// with guessed tags. The contributor must still verify the test,
// write the .expected file and check the tags.

use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

// Substring search with grep -w style word boundaries on both ends.
fn contains_word(haystack: &str, needle: &str) -> bool {
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    if n.is_empty() || h.len() < n.len() {
        return false;
    }
    for i in 0..=(h.len() - n.len()) {
        if &h[i..i + n.len()] == n {
            let before_ok = i == 0 || !is_word_byte(h[i - 1]);
            let after_ok = i + n.len() == h.len() || !is_word_byte(h[i + n.len()]);
            if before_ok && after_ok {
                return true;
            }
        }
    }
    false
}

fn main() {
    if !Path::new("README.md").is_file() {
        eprintln!("run from the base directory.");
        exit(1);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: add-single-exec <file>");
        exit(1);
    }
    let input = &args[1];
    if !Path::new(input).is_file() {
        eprintln!("{} does not exist", input);
        exit(1);
    }

    let mut chosen: Option<String> = None;
    for n in 1..=10000 {
        let candidate = format!("./tests/single-exec/{:05}.clj", n);
        if !Path::new(&candidate).exists() {
            chosen = Some(candidate);
            break;
        }
    }
    let chosen = match chosen {
        Some(c) => c,
        None => {
            eprintln!("no numbers left.");
            exit(1);
        }
    };

    if fs::copy(input, &chosen).is_err() {
        eprintln!("cannot copy {} to {}", input, chosen);
        exit(1);
    }

    eprintln!("inferring tags...");
    let content = fs::read(input).unwrap_or_default();
    let text = String::from_utf8_lossy(&content);

    let mut tags: Vec<&str> = vec!["jvm"];
    // Guess host interop; such tests are unlikely to be portable.
    let interop = [
        "clojure.java",
        "System/",
        "Thread/",
        "java.",
        "(import ",
        "proxy ",
        "gen-class",
    ];
    if interop.iter().any(|p| text.contains(p)) {
        tags.push("needs-java-interop");
    } else {
        tags.push("portable");
    }
    let io_words = [
        "slurp",
        "spit",
        "with-open",
        "clojure.java.io",
        "clojure.java.shell",
    ];
    if io_words.iter().any(|p| contains_word(&text, p)) {
        tags.push("needs-io");
    }
    let thread_words = [
        "future", "pmap", "pcalls", "promise", "agent", "send", "await", "dosync", "ref-set",
        "alter",
    ];
    if thread_words.iter().any(|p| contains_word(&text, p)) {
        tags.push("needs-threads");
    }

    let tagfile = format!("{}.tags", chosen);
    let body = tags.join("\n") + "\n";
    if fs::write(&tagfile, &body).is_err() {
        eprintln!("cannot write {}", tagfile);
        exit(1);
    }

    eprintln!(
        "verify the test passes, write {}.expected, and check the tags:",
        chosen
    );
    eprint!("{}", body);
    println!("{}", chosen);
}
