use std::sync::Arc;

pub struct ArcTrace<T> {
    arc: Arc<T>,
}

impl<T> ArcTrace<T> {
    pub fn new(value: T) -> Self {
        Self {
            arc: Arc::new(value),
        }
    }
}

impl<T> Clone for ArcTrace<T> {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}

impl<T> std::ops::Deref for ArcTrace<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.arc
    }
}
