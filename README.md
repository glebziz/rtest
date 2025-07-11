# rtest

A lightweight Rust testing library that enhances the standard testing framework with execution time logging, structured test cases, and convenient test skipping.

## Features

- **Execution Time Logging**: Automatically logs the execution time of each test
- **Structured Test Cases**: Define test cases with shared variables and custom structures
- **Test Skipping**: Easily skip tests using the `skip` prefix
- **Pretty Output Format**: Clean, Go-inspired test output format
- **Zero Dependencies**: No external dependencies required

## Installation

Add `rtest` to your `Cargo.toml`:

```toml
[dependencies]
rtest = { version = "0.1.0", git = "https://github.com/glebziz/rtest" }
```

Or use the cargo command:

```bash
cargo add rtest --git https://github.com/glebziz/rtest
```

## Usage

### Simple Test Functions

Use the `test_fn!` macro to create test functions with execution time logging:

```rust
use rtest::test_fn;

#[cfg(test)]
mod tests {
    use super::*;
    
    test_fn!(basic_test => {
        assert_eq!(2 + 2, 4);
    });
    
    test_fn!(skip skipped_test => {
        // This test will be ignored when running cargo test
        // But will run when using cargo test -- --include-ignored
        assert_eq!(1, 1);
    });
}
```

### Structured Test Cases

Use the `test_cases!` macro to create a test module with multiple test cases:

```rust
use rtest::test_cases;

test_cases!(string_length =>
    vars {
        const TEST_STR: &'static str = "Hello";
    }, 
    cases {
        struct TestCase {
            input: &'static str,
            expected: usize,
        }
    }[
        case(empty_string, TestCase { input: "", expected: 0 }),
        skip case(null_string, TestCase { input: "\0", expected: 1 }),
        case(hello_string, TestCase { input: TEST_STR, expected: TEST_STR.len() }),
    ] => |tc: TestCase| {
        assert_eq!(tc.input.len(), tc.expected);
    }
);
```

### Alternating Regular and Skipped Tests

The `test_cases!` macro supports alternating patterns of regular and skipped tests:

```rust
use rtest::test_cases;

// Starting with a skipped test
test_cases!(alternating_test => vars{}, cases{
    struct T;
}[
    skip case(skipped_test1, ()),
    case(regular_test1, ()),
    skip case(skipped_test2, ()),
    case(regular_test2, ())
] => |_| {
    assert_eq!(1, 1);
});

// Starting with a regular test
test_cases!(alternating_reverse_test => vars{}, cases{
    struct T;
}[
    case(regular_test1, ()),
    skip case(skipped_test1, ()),
    case(regular_test2, ()),
    skip case(skipped_test2, ())
] => |_| {
    assert_eq!(1, 1);
});
```

## Running Tests

Run tests normally with cargo:

```bash
cargo test
```

Include ignored (skipped) tests:

```bash
cargo test -- --include-ignored
```

## Output Format

The test output is formatted in a clean, Go-inspired style:

```
=== RUN  	test_name
--- PASS:	test_name (0.123)
```

or in case of failure:

```
=== RUN  	test_name
--- FAIL:	test_name (0.123)
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.