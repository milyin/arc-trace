# Arc-Trace

Arc-Trace is a library for ease debugging memory leaks due to cyclic `Arc` references. In complex projects sometimes it's not easy to localise place where cyclic reference occurs.

To diagnose memory leaks using this library do the following steps:

1. Search for memory leaks with, for example, valgrind [https://valgrind.org/]
```
cargo build --example leak
valgrind target/debug/examples/leak
```

For this simple example valgring is enough, it points exactly to the places where memory leak occurs:
```
==18309== Memcheck, a memory error detector
==18309== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==18309== Using Valgrind-3.18.1 and LibVEX; rerun with -h for copyright info
==18309== Command: target/debug/examples/leak
==18309== 
Tracer: std::sync::mutex::Mutex<core::option::Option<leak::Bar>>
Remained instance 1, created at:
   0: arc_trace::Tracer::get_next_id
             at ./src/lib.rs:32:25
   1: <arc_trace::ArcTrace<T> as core::clone::Clone>::clone
             at ./src/lib.rs:85:18
   2: <leak::Foo as core::clone::Clone>::clone
             at ./examples/leak.rs:6:5
   3: leak::Bar::set_foo
             at ./examples/leak.rs:30:42

```

But for more complex project with multiple leaks it may be hard to analyze gigantic valgrind output and, which is more important, it can be hard to make sure that after the fix the problem had really disappeared. So it may be convenient to explictly validate suspicious `Arc`s found by valgrind.

2. To do this replace `Arc` being tested to `ArcTrace`, add call `arc_trace::print_traces()` at the end. Then run your application with logs enabled 
```
RUST_LOG=none,arc_trace=trace cargo run --example leak
```
result is
```
[2023-12-21T13:54:43Z TRACE arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Bar>> 0 created
[2023-12-21T13:54:43Z TRACE arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Foo>> 0 created
[2023-12-21T13:54:43Z TRACE arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Foo>> 1 cloned, refcount = 2
[2023-12-21T13:54:43Z TRACE arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Bar>> 1 cloned, refcount = 2
[2023-12-21T13:54:43Z DEBUG arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Foo>> 0 dropped, refcount = 2
[2023-12-21T13:54:43Z DEBUG arc_trace] std::sync::mutex::Mutex<core::option::Option<leak::Bar>> 0 dropped, refcount = 2
Tracer: std::sync::mutex::Mutex<core::option::Option<leak::Bar>>
Remained instance 1, created at:
   0: arc_trace::Tracer::get_next_id
             at ./src/lib.rs:32:25
   1: <arc_trace::ArcTrace<T> as core::clone::Clone>::clone
             at ./src/lib.rs:85:18
   2: <leak::Foo as core::clone::Clone>::clone
             at ./examples/leak.rs:6:5
   3: leak::Bar::set_foo
             at ./examples/leak.rs:30:42
   4: leak::main
             at ./examples/leak.rs:50:9
...
```

3. This confirmes the leak by this specific `Arc`. This can be fixed with `Weak` reference as in other example. Running it confirms that no leak occurs by this specific `Arc`:
```
RUST_LOG=none,arc_trace=trace cargo run --example weak
```
result is
```
[2023-12-21T14:05:40Z TRACE arc_trace] std::sync::mutex::Mutex<core::option::Option<weak::Bar>> 0 created
[2023-12-21T14:05:40Z DEBUG arc_trace] std::sync::mutex::Mutex<core::option::Option<weak::Bar>> 0 dropped, refcount = 1
Tracer: std::sync::mutex::Mutex<core::option::Option<weak::Bar>>
```

Despite that the same results can be achieved with valgrind only, the tool is still usefull as it allows to easily confirm the leak in the specific place and make sure that the leak disappears after the fix.