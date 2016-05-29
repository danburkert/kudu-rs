# kudu-rs

Experimental Rust bindings for Apache Kudu (incubating).
interface.

# build

You will need to set `$KUDU_HOME` to your checkout of the kudu repository, and
build Kudu in `$KUDU_HOME/build/<build-type>` (see the Kudu
[build from source instructions](http://getkudu.io/docs/installation.html#_build_from_source)).

```bash
env KUDU_HOME=<path-to-kudu-checkout> cargo build
```
