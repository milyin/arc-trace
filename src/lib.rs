use std::{sync::{Arc, atomic::AtomicU64, Mutex}, any::type_name, backtrace::{Backtrace}, collections::HashMap};

static TRACERS: Mutex<Option<Vec<Arc<Tracer>>>> = Mutex::new(None);

pub struct Tracer {
    name: String,
    next_id: AtomicU64,
    locations: Mutex<HashMap<u64, Backtrace>>,
}

impl Tracer {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            name,
            next_id: AtomicU64::new(0),
            locations: Mutex::new(HashMap::new()),
        }
    }
    pub fn get_next_id(&self) -> u64 {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut locations = self.locations.lock().unwrap();
        let backtrace = Backtrace::capture();
        locations.insert(id, backtrace);
        id
    }

    pub fn drop_id(&self, id: u64) {
        let mut locations = self.locations.lock().unwrap();
        locations.remove(&id);
    }
}

pub fn print_traces() {
    let tracers = TRACERS.lock().unwrap();
    if let Some(tracers) = &*tracers {
        for tracer in tracers {
            let locations = tracer.locations.lock().unwrap();
            for (id, backtrace) in locations.iter() {
                log::error!("{} {} {}", tracer.name, id, " created at:");
                log::error!("{}", backtrace);
            }
        }
    }
}

pub struct ArcTrace<T> {
    arc: Arc<T>,
    tracer: Arc<Tracer>,
    id: u64,
}

impl<T> ArcTrace<T> {
    pub fn new(value: T) -> Self {
        let arc = Arc::new(value);
        let tracer = Arc::new(Tracer::new(type_name::<T>().to_string()));
        let id = tracer.get_next_id();
        TRACERS.lock().unwrap().get_or_insert_with(Vec::new).push(tracer.clone());
        log::trace!("{} {} {}", tracer.name, id, "created");
        Self {
            arc,
            tracer,
            id,
        }
    }
}

impl<T> Clone for ArcTrace<T> {
    fn clone(&self) -> Self {
        let arc = self.arc.clone();
        let tracer = self.tracer.clone();
        let id = tracer.get_next_id();
        log::trace!("{} {} cloned, refcount = {}", tracer.name, id, Arc::strong_count(&arc));
        Self {
            arc,
            tracer,
            id,
        }
    }
}

impl<T> Drop for ArcTrace<T> {
    fn drop(&mut self) {
        self.tracer.drop_id(self.id);
        log::trace!("{} {} dropped, refcount = {}", self.tracer.name, self.id, Arc::strong_count(&self.arc));
    }
}

impl<T> std::ops::Deref for ArcTrace<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}
