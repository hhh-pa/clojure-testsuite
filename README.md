# clojure-testsuite

A Clojure equivalent of [c-testsuite](https://github.com/c-testsuite/c-testsuite):
a collaborative database of Clojure implementation test cases, minimal test
runners, and public test results. The general idea is that the various test
suites here adhere to well defined, simple interfaces and projects can
contribute tests back that match those interfaces, or use those tests by
implementing runner scripts matching the specifications.

There are many tools that may benefit such as Clojure runtimes (JVM Clojure,
Babashka, ClojureScript, ClojureCLR, jank, Basilisp, ...), transpilers and
linters, so we seek to agree on simple easy test interfaces, and implement
those interfaces for a variety of tools.

# Test suites

The top level test-suite runners output https://testanything.org output.

## single-exec suite

entry point is ```./single-exec```

### runners/single-exec/*

The runner will be invoked as:

```
$ ./runners/single-exec/$NAME ./tests/single-exec/case.clj
```

The runner is free to output any data it wants, but must return
nonzero on failure.

The runner will be considered a failure if it takes more than 5 minutes.

Currently implemented runners:

| runner     | implementation                  | requires |
|------------|---------------------------------|----------|
| `clojure`  | JVM Clojure via the Clojure CLI | `clojure` on PATH |
| `babashka` | Babashka                        | `bb` on PATH |

### tests/single-exec/*

- Single .clj file tests.
- The file is evaluated top level form by top level form, as a script.
- The file $t.expected must match stdout+stderr of the test, byte for byte.
- The test programs exit with 0 on success.

Portability and host requirements are specified via tags that can be
filtered against using search queries, that can generate skip lists.

### Example:

```$ ./single-exec clojure ```

# Skipping tests

Try to skip a test if your implementation can NEVER pass it, the test is not appropriate.
In that case, there is only one mechanism, add a command that prints a list of tests to skip
on stdout named:

```
./runners/*/$TOOL.skip
```

# Search and query

All tests have a matching $t.tags file. This file specifies attributes
of the test that can be filtered and queried.

Unlike c-testsuite, no third party index is needed: ```./scripts/search-tests```
is a small Rust tool (see ```tools/```, built automatically when missing or out of date)
that reads the .tags files directly, so there is no index to build.

The query language supports ```and```, ```or``` and ```not``` (whitespace
between two queries also means ```and```), parentheses, plain tags and
```key=value``` matches:

```
$ ./scripts/search-tests "portable"
$ ./scripts/search-tests "suite=single-exec portable not needs-io"
$ ./scripts/search-tests "not (jvm or portable)"
```

Support tags are currently

```
suite={single-exec, ...}
portable
	The test should pass on every supported implementation. Portable
	tests stick to host-agnostic core language behavior.
jvm
	The test should pass on JVM Clojure.
bb
	The test should pass on Babashka.
needs-java-interop
    Test relies on Java host interop.
needs-io
    Test relies on IO beyond stdout/stderr (files, sockets, subprocesses).
needs-threads
    Test relies on threads, futures, agents or STM.
```

These queries can be used to generate skip lists.

# otags files

otag files are intended to allow
a tests origin to be discovered. They contain the following fields.

```
org=$DOMAINNAME
repository=$SRCURL
version=$UNIQUE_VERSION
path=$TEST_PATH_IN_REPOSITORY
```

# Dependencies

Running tests

- posix sh
- Rust (cargo) to build the tools in ```tools/``` (built automatically
  on first use)
- coreutils
- implementation under test

# Contributing tests

Write a single .clj file whose combined stdout+stderr is deterministic,
add it with ```./scripts/add-single-exec mytest.clj```, verify the inferred
tags, run it with each runner, and write the verified output to the
matching .expected file.

The helper tools (```search-tests```, ```tapsummary```, ```htmlescape```,
```add-single-exec```) are implemented in Rust under ```tools/``` with no
external dependencies; the ```scripts/``` entry points build them when missing
or out of date (or run ```cargo build --release``` in ```tools/``` explicitly).

Guidelines for portable tests:

- never print hash-maps or hash-sets directly (iteration order is
  unspecified); print ```(sort m)``` / ```(sort s)``` instead.
- never print stack traces, object identities, timings or absolute paths.
- end output with a newline and avoid trailing whitespace.
- keep test files ASCII-only; build non-ASCII strings from code points
  (e.g. ```(char 233)```) so editors cannot silently renormalize them.

# Tips

## Naming test cases

The names are not stable for now, so if you
refer to a test case in your issue tracker, it is best to
name it something like ```clojure-testsuite/$CLJTESTGITCOMMIT/path/to/test```.

## Getting a summary from the command line

./single-exec $runner | ./scripts/tapsummary | head

## Differences from c-testsuite

- Test files are .clj scripts instead of .c files; there is no compile
  step and no ```main``` entry point convention.
- Helper tools (```search-tests```, ```tapsummary```, ```htmlescape```,
  ```add-single-exec```) are implemented in Rust under ```tools/``` instead
  of python3; ```scripts/``` holds shims that build and exec them.
- ```scripts/search-tests``` needs no index: there is no
  ```make-search-index``` step and no TMSU dependency.
- Skip lists are matched with ```grep``` instead of a sqlite lookup table.
- Runners write transient outputs to a scratch directory instead of
  next to the test files.

## License / attribution

The test harness is MIT licensed, see LICENSE. The harness design and
some scripts are derived from c-testsuite
(Copyright (c) 2018 Andrew Chambers, MIT). Individual test cases are
covered by tests/LICENSE.
