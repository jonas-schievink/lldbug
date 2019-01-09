Running this crashes with a segmentation fault on my Linux machine:

```
docker build .
docker run
```

Or outside of Docker:

```
env RUST_BACKTRACE=1 cargo run
```

This seems to happen due to malformed debug info when the program panics and
builds a backtrace.

Running this `readelf` invokation reveals problems in the `.debug_line` header:

```
$ readelf --wide --debug-dump=rawline target/debug/ta-client | grep debug_line
readelf: Warning: The length field (0x97000000) in the debug_line header is wrong - the section is too small
```
