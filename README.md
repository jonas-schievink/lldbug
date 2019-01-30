Running this crashes with a segmentation fault on my Linux machine:

```
docker build .
docker run -it [image] /bin/bash
# Inside the container:
./bad.sh
```

If you've replicated the setup with `lld` on your host, you can just run
`./bad.sh` there. Note that you have to install `dwarf-validate` first:
`cargo install gimli --examples`.

This seems to happen due to malformed debug info when the program panics and
builds a backtrace.

Running this `readelf` invokation reveals problems in the `.debug_line` header:

```
$ readelf --wide --debug-dump=rawline target/debug/ta-client | grep debug_line
readelf: Warning: The length field (0x97000000) in the debug_line header is wrong - the section is too small
```

Note that `readelf` might not always work reliably. The gimli crate has a
dwarf-validate example which is used here instead. The Docker container already
installs it, but you can also run `cargo install gimli --examples` to install
manually.
