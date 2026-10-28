// Self contained tag search: reads tests/*/*.tags directly, no index step.
//
// Query language: and, or, not (whitespace between two queries also means
// and), parentheses, plain tags and key=value matches (only suite=...).

use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

enum Node {
    Or(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Eq(String, String),
    Tag(String),
}

fn tokenize(s: &str) -> Vec<String> {
    let mut toks = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c == '(' || c == ')' {
            if !cur.is_empty() {
                toks.push(std::mem::take(&mut cur));
            }
            toks.push(c.to_string());
        } else if c.is_whitespace() {
            if !cur.is_empty() {
                toks.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        toks.push(cur);
    }
    toks
}

struct Parser {
    toks: Vec<String>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&str> {
        self.toks.get(self.pos).map(|s| s.as_str())
    }

    fn next(&mut self) -> Option<String> {
        let t = self.toks.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn parse(&mut self) -> Result<Node, String> {
        if self.toks.is_empty() {
            return Err("empty query".to_string());
        }
        let node = self.parse_or()?;
        if let Some(t) = self.peek() {
            return Err(format!("unexpected '{}'", t));
        }
        Ok(node)
    }

    fn parse_or(&mut self) -> Result<Node, String> {
        let mut node = self.parse_and()?;
        while matches!(self.peek(), Some(t) if t.eq_ignore_ascii_case("or")) {
            self.next();
            let rhs = self.parse_and()?;
            node = Node::Or(Box::new(node), Box::new(rhs));
        }
        Ok(node)
    }

    fn parse_and(&mut self) -> Result<Node, String> {
        let mut node = self.parse_not()?;
        loop {
            match self.peek() {
                None => return Ok(node),
                Some(t) if t == ")" => return Ok(node),
                Some(t) if t.eq_ignore_ascii_case("or") => return Ok(node),
                Some(t) if t.eq_ignore_ascii_case("and") => {
                    self.next();
                }
                _ => {}
            }
            let rhs = self.parse_not()?;
            node = Node::And(Box::new(node), Box::new(rhs));
        }
    }

    fn parse_not(&mut self) -> Result<Node, String> {
        if matches!(self.peek(), Some(t) if t.eq_ignore_ascii_case("not")) {
            self.next();
            Ok(Node::Not(Box::new(self.parse_not()?)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Node, String> {
        match self.next() {
            None => Err("unexpected end of query".to_string()),
            Some(t) if t == "(" => {
                let node = self.parse_or()?;
                match self.next() {
                    Some(c) if c == ")" => Ok(node),
                    _ => Err("expected ')'".to_string()),
                }
            }
            Some(t) if t == ")" => Err("unexpected ')'".to_string()),
            Some(t) if t.eq_ignore_ascii_case("and")
                || t.eq_ignore_ascii_case("or")
                || t.eq_ignore_ascii_case("not") =>
            {
                Err(format!("unexpected '{}'", t))
            }
            Some(t) => match t.split_once('=') {
                Some((key, value)) => Ok(Node::Eq(key.to_string(), value.to_string())),
                None => Ok(Node::Tag(t)),
            },
        }
    }
}

fn eval_node(node: &Node, suite: &str, tags: &[String]) -> bool {
    match node {
        Node::Or(a, b) => eval_node(a, suite, tags) || eval_node(b, suite, tags),
        Node::And(a, b) => eval_node(a, suite, tags) && eval_node(b, suite, tags),
        Node::Not(a) => !eval_node(a, suite, tags),
        Node::Eq(key, value) => key == "suite" && suite == value,
        Node::Tag(tag) => tags.iter().any(|t| t == tag),
    }
}

struct Test {
    suite: String,
    path: String,
    tags: Vec<String>,
}

fn load_tests() -> Vec<Test> {
    let mut found = Vec::new();
    let testsdir = Path::new("./tests");
    let mut suites: Vec<String> = fs::read_dir(testsdir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    suites.sort();
    for suite in suites {
        let suitedir = testsdir.join(&suite);
        let mut names: Vec<String> = fs::read_dir(&suitedir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| n.ends_with(".clj"))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        for name in names {
            // Print paths exactly as single-exec names them.
            let path = format!("./tests/{}/{}", suite, name);
            let mut tags = Vec::new();
            let tagfile = format!("{}.tags", path);
            if let Ok(content) = fs::read_to_string(&tagfile) {
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        tags.push(line.to_string());
                    }
                }
            }
            found.push(Test {
                suite: suite.clone(),
                path,
                tags,
            });
        }
    }
    found
}

fn main() {
    if !Path::new("README.md").is_file() || !Path::new("./tests").is_dir() {
        eprintln!("run from the base directory.");
        exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let tests = load_tests();
    if args.len() == 1 {
        for t in &tests {
            println!("{}", t.path);
        }
        return;
    }
    if args.len() != 2 {
        eprintln!("usage: search-tests [query]");
        exit(1);
    }
    let mut parser = Parser {
        toks: tokenize(&args[1]),
        pos: 0,
    };
    let node = match parser.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("bad query: {}", e);
            exit(1);
        }
    };
    for t in &tests {
        if eval_node(&node, &t.suite, &t.tags) {
            println!("{}", t.path);
        }
    }
}
