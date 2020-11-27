Attempting to reproduce an issue when generating code-coverage for a project that uses the crate `prometheus-0.9`

To reproduce (using nightly-2020-11-25):

```
$ rm *.prof*
$ cargo clean
$ RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="cargo-%m.profraw" cargo +nightly-2020-11-25 test --tests
...
     Running target/debug/deps/prometheus_code_cov-24bcfe5d001389f5

running 1 test
test tests::it_returns_1 ... ok
...
$ cargo +nightly-2020-11-25 profdata -- merge -sparse *.profraw -o out.profdata
$ cargo cov -- report --instr-profile=./out.profdata target/debug/deps/prometheus_code_cov-24bcfe5d001389f5
error: target/debug/deps/prometheus_code_cov-24bcfe5d001389f5: Failed to load coverage: Truncated coverage data
```

To generate code coverage, do one of the following:
* Change `prometheus` in `Cargo.toml` to `0.8`
* Remove `use prometheus::Encoder;` in `src/lib.rs:4`
* Remove `tokio::spawn(start_server());` in `src/lib.rs:13`


Output when one of the above is changed:
```
$ rm *.prof*
$ cargo clean
$ RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="cargo-%m.profraw" cargo +nightly-2020-11-25 test --tests
$ cargo +nightly-2020-11-25 profdata -- merge -sparse *.profraw -o out.profdata
$ cargo cov -- report --instr-profile=./out.profdata target/debug/deps/prometheus_code_cov-24bcfe5d001389f5
...
src/lib.rs                                                                                                                12                 8    33.33%           6                 2    66.67%          24                 9    62.50%
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                                                                                  17839             17136     3.94%        2078              1765    15.06%       20773             19266     7.25%
```
