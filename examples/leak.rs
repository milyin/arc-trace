use arc_trace::ArcTrace;
use std::sync::Mutex;

#[derive(Clone)]
struct Foo {
    bar: ArcTrace<Mutex<Option<Bar>>>,
}

impl Foo {
    pub fn set_bar(&self, r: &Bar) {
        *self.bar.lock().unwrap() = Some(r.clone());
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            bar: ArcTrace::new(Mutex::new(None)),
        }
    }
}

#[derive(Clone)]
struct Bar {
    foo: ArcTrace<Mutex<Option<Foo>>>,
}

impl Bar {
    pub fn set_foo(&self, r: &Foo) {
        *self.foo.lock().unwrap() = Some(r.clone());
    }
}

impl Default for Bar {
    fn default() -> Self {
        Self {
            foo: ArcTrace::new(Mutex::new(None)),
        }
    }
}

fn main() {
    // initialize logginng library
    // run with RUST_LOG=arc_trace=trace cargo run --example leak
    env_logger::init();
    {
        let a = Foo::default();
        let b = Bar::default();
        a.set_bar(&b);
        b.set_foo(&a);
    } // do the drops here
      // show places where the cycle is created
    arc_trace::print_traces();
}
