A string buffer, print content when test has panicked.

# Examples

```rust,no_run
#[test]
fn some_test() {
    let mut s = TestBuffer::new();
    writeln!(s, "foo");
    writeln!(s, "bar");
    panic!();
    writeln!(s, "baz");
}
```

Outputs:

```text
---- some_test stdout ----

thread 'some_test' (15399) panicked at src/lib.rs:120:5:
explicit panic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
===> TestBuffer <unnamed> content preview in panicking <===
foo
bar
```

# Features

- expect-test : can using `.expect(expect![])` etc
