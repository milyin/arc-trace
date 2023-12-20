# Arc-Trace

Arc-Trace is a library for ease debugging memory leaks due to cyclic `Arc` references. In complex projects sometimes it's not easy to localise place where cyclic reference occurs.

How to use it:

// TODO -- write doc, make example

To enable logging for this crate only use this syntax:
```
RUST_LOG=none,arc_trace=trace cargo run
```
