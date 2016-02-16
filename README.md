# kudu-rs

Experimental Rust bindings for Apache Kudu (incubating). Relies on a
[branch](https://github.com/danburkert/kudu/tree/c-api) of Kudu with a C
interface. Very limited functionality.

You will probably need to set `$KUDU_HOME` to the build location of the branch
when compiling, and add `$KUDU_HOME/lib` to `$LD_LIBRARY_PATH` when running.
