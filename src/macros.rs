#[macro_export]
macro_rules! test_main {
    ($($testfn:expr,)*) => {
        fn main() {
            extern crate rustc_test;
            rustc_test::test_main_static(&[$(
                rustc_test::TestDescAndFn {
                    desc: rustc_test::TestDesc {
                        name: rustc_test::TestName::StaticTestName(
                                    stringify!($testfn)),
                        ignore: false,
                        should_panic: rustc_test::ShouldPanic::No,
                    },
                    testfn: rustc_test::TestFn::StaticTestFn($testfn),
                }
            ),*])
        }
    }
}

#[macro_export]
macro_rules! bench_main {
    ($($testfn:expr,)*) => {
        fn main() {
            extern crate rustc_test;
            rustc_test::test_main_static(&[$(
                rustc_test::TestDescAndFn {
                    desc: rustc_test::TestDesc {
                        name: rustc_test::TestName::StaticTestName(
                                    stringify!($testfn)),
                        ignore: false,
                        should_panic: rustc_test::ShouldPanic::No,
                    },
                    testfn: rustc_test::TestFn::StaticBenchFn($testfn),
                }
            ),*])
        }
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        ::rustc_test::__stdio::__print(&format_args!(concat!($fmt, "\n")))
    };
    ($fmt:expr, $($e:tt)*) => {
        ::rustc_test::__stdio::__print(&format_args!(concat!($fmt, "\n"), $($e)*))
    }
}

#[macro_export]
macro_rules! print {
    ($($e:tt)*) => (::rustc_test::__stdio::__print(&format_args!($($e)*)))
}
