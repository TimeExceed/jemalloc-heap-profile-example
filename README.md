# An Example on How to profile heap by jemalloc in Rust

## Prerequisites

1.  Rust development environment
1.  C compiler
1.  tools released with jemalloc

    ```console
    $ sudo apt install libjemalloc-dev
    ```

## Profile

```console
$ cargo build && _RJEM_MALLOC_CONF="prof:true" target/debug/h
$ jeprof --base=test.0.hprof --pdf target/debug/h test.10.hprof > test.pdf
```

## References

* https://github.com/jemalloc/jemalloc/wiki/Use-Case%3A-Heap-Profiling
* https://gist.github.com/ordian/928dc2bd45022cddd547528f64db9174
