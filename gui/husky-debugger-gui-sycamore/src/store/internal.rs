use super::*;

pub struct StoreInternal<T>
where
    T: Storable,
{
    pub(super) value: T,
    subscribers: Vec<(u8, Box<dyn Fn(T)>)>,
    next_subscriber_id: u8,
}

impl<T> StoreInternal<T>
where
    T: Storable,
{
    pub(super) fn new(value: T) -> Self {
        StoreInternal {
            value,
            subscribers: vec![],
            next_subscriber_id: 0,
        }
    }

    pub(super) fn set(&mut self, value: T) {
        for subscriber in &self.subscribers {
            subscriber.1(value.clone())
        }
        self.value = value
    }

    pub(super) fn subscribe(&mut self, subscriber: Box<dyn Fn(T)>) -> u8 {
        let id = self.issue_subscriber_id();
        self.subscribers.push((id, subscriber));
        id
    }

    pub(super) fn issue_subscriber_id(&mut self) -> u8 {
        let id = self.next_subscriber_id;
        self.next_subscriber_id += 1;
        id
    }

    pub(super) fn unsubscribe(&mut self, id: u8) {
        let index = self
            .subscribers
            .iter()
            .position(|elem| elem.0 == id)
            .unwrap();
        self.subscribers.remove(index);
    }

    pub(super) fn value(&self) -> T {
        self.value.clone()
    }
}
