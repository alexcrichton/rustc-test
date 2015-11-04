#[macro_use]
extern crate rustc_test;

use std::sync::mpsc::channel;

use rustc_test::{TrFailed, TrIgnored, TrOk, filter_tests, parse_opts,
                 TestDesc, TestDescAndFn, TestOpts, run_test, MetricMap,
                 StaticTestName, DynTestName, DynTestFn, ShouldPanic};

fn do_not_run_ignored_tests() {
    fn f() { panic!(); }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: true,
            should_panic: ShouldPanic::No,
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res != TrOk);
}

fn ignored_tests_result_in_ignored() {
    fn f() { }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: true,
            should_panic: ShouldPanic::No,
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res == TrIgnored);
}

fn test_should_panic() {
    fn f() { panic!(); }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            should_panic: ShouldPanic::Yes,
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res == TrOk);
}

fn test_should_panic_good_message() {
    fn f() { panic!("an error message"); }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            should_panic: ShouldPanic::YesWithMessage("error message"),
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res == TrOk);
}

fn test_should_panic_bad_message() {
    fn f() { panic!("an error message"); }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            should_panic: ShouldPanic::YesWithMessage("foobar"),
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res == TrFailed);
}

fn test_should_panic_but_succeeds() {
    fn f() { }
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("whatever"),
            ignore: false,
            should_panic: ShouldPanic::Yes,
        },
        testfn: DynTestFn(Box::new(move|| f())),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, _) = rx.recv().unwrap();
    assert!(res == TrFailed);
}

fn parse_ignored_flag() {
    let args = vec!("progname".to_string(),
                    "filter".to_string(),
                    "--ignored".to_string());
    let opts = match parse_opts(&args) {
        Some(Ok(o)) => o,
        _ => panic!("Malformed arg in parse_ignored_flag")
    };
    assert!((opts.run_ignored));
}

fn filter_for_ignored_option() {
    // When we run ignored tests the test filter should filter out all the
    // unignored tests and flip the ignore flag on the rest to false

    let mut opts = TestOpts::new();
    opts.run_tests = true;
    opts.run_ignored = true;

    let tests = vec!(
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("1"),
                ignore: true,
                should_panic: ShouldPanic::No,
            },
            testfn: DynTestFn(Box::new(move|| {})),
        },
        TestDescAndFn {
            desc: TestDesc {
                name: StaticTestName("2"),
                ignore: false,
                should_panic: ShouldPanic::No,
            },
            testfn: DynTestFn(Box::new(move|| {})),
        });
    let filtered = filter_tests(&opts, tests);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].desc.name.to_string(),
               "1");
    assert!(filtered[0].desc.ignore == false);
}

fn sort_tests() {
    let mut opts = TestOpts::new();
    opts.run_tests = true;

    let names =
        vec!("sha1::test".to_string(),
             "isize::test_to_str".to_string(),
             "isize::test_pow".to_string(),
             "test::do_not_run_ignored_tests".to_string(),
             "test::ignored_tests_result_in_ignored".to_string(),
             "test::first_free_arg_should_be_a_filter".to_string(),
             "test::parse_ignored_flag".to_string(),
             "test::filter_for_ignored_option".to_string(),
             "test::sort_tests".to_string());
    let tests =
    {
        fn testfn() { }
        let mut tests = Vec::new();
        for name in &names {
            let test = TestDescAndFn {
                desc: TestDesc {
                    name: DynTestName((*name).clone()),
                    ignore: false,
                    should_panic: ShouldPanic::No,
                },
                testfn: DynTestFn(Box::new(testfn)),
            };
            tests.push(test);
        }
        tests
    };
    let filtered = filter_tests(&opts, tests);

    let expected =
        vec!("isize::test_pow".to_string(),
             "isize::test_to_str".to_string(),
             "sha1::test".to_string(),
             "test::do_not_run_ignored_tests".to_string(),
             "test::filter_for_ignored_option".to_string(),
             "test::first_free_arg_should_be_a_filter".to_string(),
             "test::ignored_tests_result_in_ignored".to_string(),
             "test::parse_ignored_flag".to_string(),
             "test::sort_tests".to_string());

    for (a, b) in expected.iter().zip(filtered) {
        assert!(*a == b.desc.name.to_string());
    }
}

fn test_metricmap_compare() {
    let mut m1 = MetricMap::new();
    let mut m2 = MetricMap::new();
    m1.insert_metric("in-both-noise", 1000.0, 200.0);
    m2.insert_metric("in-both-noise", 1100.0, 200.0);

    m1.insert_metric("in-first-noise", 1000.0, 2.0);
    m2.insert_metric("in-second-noise", 1000.0, 2.0);

    m1.insert_metric("in-both-want-downwards-but-regressed", 1000.0, 10.0);
    m2.insert_metric("in-both-want-downwards-but-regressed", 2000.0, 10.0);

    m1.insert_metric("in-both-want-downwards-and-improved", 2000.0, 10.0);
    m2.insert_metric("in-both-want-downwards-and-improved", 1000.0, 10.0);

    m1.insert_metric("in-both-want-upwards-but-regressed", 2000.0, -10.0);
    m2.insert_metric("in-both-want-upwards-but-regressed", 1000.0, -10.0);

    m1.insert_metric("in-both-want-upwards-and-improved", 1000.0, -10.0);
    m2.insert_metric("in-both-want-upwards-and-improved", 2000.0, -10.0);
}

fn capture_print() {
    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("1"),
            ignore: false,
            should_panic: ShouldPanic::No,
        },
        testfn: DynTestFn(Box::new(move|| println!("hello"))),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, out) = rx.recv().unwrap();
    assert!(res == TrOk);
    assert_eq!(out, b"hello\n");

    let desc = TestDescAndFn {
        desc: TestDesc {
            name: StaticTestName("1"),
            ignore: false,
            should_panic: ShouldPanic::No,
        },
        testfn: DynTestFn(Box::new(move|| print!("hello"))),
    };
    let (tx, rx) = channel();
    run_test(&TestOpts::new(), false, desc, tx);
    let (_, res, out) = rx.recv().unwrap();
    assert!(res == TrOk);
    assert_eq!(out, b"hello");
}

test_main! {
    test_metricmap_compare,
    sort_tests,
    filter_for_ignored_option,
    parse_ignored_flag,
    test_should_panic,
    test_should_panic_good_message,
    test_should_panic_bad_message,
    test_should_panic_but_succeeds,
    ignored_tests_result_in_ignored,
    do_not_run_ignored_tests,
    capture_print,
}
