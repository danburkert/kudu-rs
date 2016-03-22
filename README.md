# kudu-rs

Experimental Rust bindings for Apache Kudu (incubating). Relies on a
[branch](https://github.com/danburkert/kudu/tree/c-api) of Kudu with a C
interface.

You will need to set `$KUDU_HOME` to your checkout of the kudu repository, and
build Kudu in `$KUDU_HOME/build/debug` (see the Kudu
[build from source instructions](http://getkudu.io/docs/installation.html#_build_from_source)).
Finally, you will need to add `$KUDU_HOME/build/latest/lib` to
`$LD_LIBRARY_PATH` (or `$DYLD_LIBRARY_PATH` on OS X) when running so that the
runtime linker knows where to find `libkudu_client.so`.
