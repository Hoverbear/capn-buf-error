Output:

    Compiling capn-buf-test v0.0.1 (file:///Users/hoverbear/git/capn-buf-test)
    src/lib.rs:11:5: 11:19 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:11 use bytes::RingBuf;
                   ^~~~~~~~~~~~~~
    src/lib.rs:12:5: 12:28 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:12 use capnp::serialize_packed;
                   ^~~~~~~~~~~~~~~~~~~~~~~
    src/lib.rs:14:5: 14:19 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:14     MessageBuilder,
                   ^~~~~~~~~~~~~~
    src/lib.rs:15:5: 15:25 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:15     MallocMessageBuilder,
                   ^~~~~~~~~~~~~~~~~~~~
    src/lib.rs:17:5: 17:32 warning: unused import, #[warn(unused_imports)] on by default
    src/lib.rs:17 use messages_capnp::rpc_request;
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~
      Running target/debug/capn_buf_test-2b086c26049a850e

    running 1 test
    thread 'it_panics' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Custom(Custom { kind: WriteZero, desc: "failed to write whole buffer", detail: None }) }', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libcore/result.rs:774
    stack backtrace:
    1:        0x1067fe2c4 - sys::backtrace::write::h06710d63b1dcdeecazD
    2:        0x106802651 - panicking::on_panic::h092df8834d8af2908oJ
    3:        0x1067ed56e - rt::unwind::begin_unwind_inner::h6d975a0bc7cb8521i7I
    4:        0x1067eda36 - rt::unwind::begin_unwind_fmt::hcece9dcd528b22b8T5I
    5:        0x106801f9d - rust_begin_unwind
    6:        0x106822235 - panicking::panic_fmt::h910baae85b00de179qB
    7:        0x106789e5a - result::Result<T, E>::unwrap::h12050628802221074052
    8:        0x106785c83 - it_panics::hbb04f1850807a404IEa
    9:        0x1067b58c0 - thunk::F.Invoke<A, R>::invoke::h16869512477737918925
    10:        0x1067b9718 - thunk::F.Invoke<A, R>::invoke::h11481595048560544646
    11:        0x1067b6417 - rt::unwind::try::try_fn::h17128231288289512860
    12:        0x106803c98 - rust_try_inner
    13:        0x106803c85 - rust_try
    14:        0x1067b67e8 - thunk::F.Invoke<A, R>::invoke::h6785311331499763573
    15:        0x106800c5d - sys::thread::create::thread_start::h5cc5a46a7850c86a84H
    16:     0x7fff9a043267 - _pthread_body
    17:     0x7fff9a0431e4 - _pthread_start
    test it_panics ... FAILED

    failures:

    failures:
     it_panics

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

    thread '<main>' panicked at 'Some tests failed', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libtest/lib.rs:261
    stack backtrace:
    1:        0x1067fe2c4 - sys::backtrace::write::h06710d63b1dcdeecazD
    2:        0x106802651 - panicking::on_panic::h092df8834d8af2908oJ
    3:        0x1067ed56e - rt::unwind::begin_unwind_inner::h6d975a0bc7cb8521i7I
    4:        0x1067988bf - rt::unwind::begin_unwind::h4301434800940568848
    5:        0x10679277c - test_main::he7cbce8ea654f0edh2a
    6:        0x106799682 - test_main_static::h3bedd21174e00852S4a
    7:        0x1067907f8 - __test::main::h6fd2f003e883f179IFa
    8:        0x106803c98 - rust_try_inner
    9:        0x106803c85 - rust_try
    10:        0x10680319e - rt::lang_start::hdeee64ec2a7b4630BjJ
    11:        0x10679084e - main
