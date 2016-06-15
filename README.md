# kudu-rs

Experimental Rust bindings for Apache Kudu (incubating). Not feature complete.

[Documentation](https://danburkert.github.io/kudu-rs/kudu/index.html)

[![Build Status](https://travis-ci.org/danburkert/kudu-rs.svg?branch=master)](https://travis-ci.org/danburkert/kudu-rs)

# build

You will need to add `kudu-master` and `kudu-tserver` to the $PATH, or set
`$KUDU_HOME` to your checkout of the kudu repository, and build Kudu in
`$KUDU_HOME/build/latest` (see the Kudu [build from source
instructions](http://getkudu.io/docs/installation.html#_build_from_source)).

```bash
env KUDU_HOME=<path-to-kudu-checkout> cargo build
```
