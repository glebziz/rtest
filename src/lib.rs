/// Creates a test module with multiple test cases.
///
/// Provides a structured way to define test cases with shared variables and test case structures.
/// Test cases can be marked with `skip` to be ignored during testing.
///
/// # Example
/// ```rust
/// use rtest::test_cases;
///
/// test_cases!(string_length =>
///     vars {
///         const TEST_STR: &'static str = "Hello";
///     }, 
///     cases {
///         struct TestCase {
///             input: &'static str,
///             expected: usize,
///         }
///     }[
///         case(empty_string, TestCase { input: "", expected: 0 }),
///         skip case(null_string, TestCase { input: "\0", expected: 1 }),
///         case(hello_string, TestCase { input: TEST_STR, expected: TEST_STR.len() }),
///     ] => |tc: TestCase| {
///         assert_eq!(tc.input.len(), tc.expected);
///     }
/// );
/// ```
#[macro_export]
macro_rules! test_cases {
    ($name:ident => vars{
        $($init:item)*
    }, cases{$case:item}[
        $(
            skip case($skip_name:ident, $skip_case:expr)
            $(, case($regular_name:ident, $regular_case:expr))?
        ),+ $(,)?
    ] => $code:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use rtest::test_fn;

            $(
                #[allow(unused_variables)]
                $init
            )*
            $case

            $(
                test_fn!(skip $skip_name, $name => {
                    $code($skip_case)
                });

                $(
                    test_fn!($regular_name, $name => {
                        $code($regular_case)
                    });
                )?
            )*
        }
    };

    ($name:ident => vars{
        $($init:item)*
    }, cases{$case:item}[
        $(
            case($regular_name:ident, $regular_case:expr)
            $(, skip case($skip_name:ident, $skip_case:expr))?
        ),+ $(,)?
    ] => $code:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use rtest::test_fn;

            $(
                #[allow(unused_variables)]
                $init
            )*
            $case

            $(
                test_fn!($regular_name, $name => {
                    $code($regular_case)
                });

                $(
                    test_fn!(skip $skip_name, $name => {
                        $code($skip_case)
                    });
                )?
            )*
        }
    };
}

/// Creates a test function with execution time logging.
///
/// This macro provides a simple way to create test functions that include execution time logging.
/// Tests can be marked with `skip` to be ignored during testing.
///
/// # Example
/// ```rust
/// mod tests {
///     use rtest::test_fn;
///
///     test_fn!(basic_test => {
///         assert_eq!(2 + 2, 4);
///     });
///     
///     test_fn!(skip skipped_test => {
///         // This test will be ignored
///         assert_eq!(1, 1);
///     });
/// }
/// ```
#[macro_export]
macro_rules! test_fn {
    (skip $name:ident $(, $sup_name:ident)? => $code:block) => {
        #[test]
        #[ignore]
        fn $name() {}
    };

    ($name:ident $(, $sup_name:ident)? => $code:block) => {
        #[test]
        fn $name() {
            let fn_name = match stringify!($($sup_name)?) {
                "" => stringify!($name).to_string(),
                _ => format!("{}/{}", stringify!($($sup_name)?), stringify!($name))
            };

            let start = std::time::Instant::now();
            println!("=== RUN  \t{}", fn_name);
            match std::panic::catch_unwind(|| $code) {
                Ok(_) => {
                    let t = start.elapsed();
                    println!("--- PASS:\t{} ({}.{})", fn_name, t.as_secs(), t.subsec_millis());
                },
                Err(err) => {
                    let t = start.elapsed();
                    println!("--- FAIL:\t{} ({}.{})", fn_name, t.as_secs(), t.subsec_millis());
                    std::panic::resume_unwind(err);
                }
            };
        }
    };
}
