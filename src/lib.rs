use std::{sync::{Arc, atomic::AtomicU64}, any::type_name};

pub struct Tracer {
    name: String,
    next_id: AtomicU64
}

impl Tracer {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            name,
            next_id: AtomicU64::new(0)
        }
    }
    pub fn get_next_id(&self) -> u64 {
        self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
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
        log::warn!("{} {} {}", tracer.name, id, "created");
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
        log::warn!("{} {} {}", tracer.name, id, "cloned");
        Self {
            arc,
            tracer,
            id,
        }
    }
}

impl<T> Drop for ArcTrace<T> {
    fn drop(&mut self) {
        log::error!("{} {} {}", self.tracer.name, self.id, "dropped");
    }
}

impl<T> std::ops::Deref for ArcTrace<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}
