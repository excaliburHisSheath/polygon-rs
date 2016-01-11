use ecs::*;
use engine::*;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>> + Debug + Clone + Default,
          T::Message: Message<Target=T>,
{
    data: T,
    messages: RefCell<Vec<T::Message>>,
}

impl<T> SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>> + Debug + Clone + Default,
          T::Message: Message<Target=T>,
{
    pub fn new(data: T) -> SingletonComponentManager<T> {
        SingletonComponentManager {
            data: data,
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn send_message<U: Into<T::Message>>(&self, message: U) {
        self.messages.borrow_mut().push(message.into());
    }
}

impl<T> Deref for SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>> + Debug + Clone + Default,
          T::Message: Message<Target=T>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> DerefMut for SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>> + Debug + Clone + Default,
          T::Message: Message<Target=T>,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T, U> ComponentManagerBase for SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>, Message=U> + Debug + Clone + Default,
          U: Message<Target=T>,
{
    fn update(&mut self) {
        let mut messages = self.messages.borrow_mut();
        for message in messages.drain(..) {
            message.apply(&mut self.data);
        }
    }
}

impl<T, U> ComponentManager for SingletonComponentManager<T>
    where T: Component<Manager=SingletonComponentManager<T>, Message=U> + Debug + Clone + Default,
          U: Message<Target=T>,
{
    type Component = T;

    fn register(builder: &mut EngineBuilder) {
        builder.register_manager(SingletonComponentManager::new(T::default()));
    }

    fn destroy(&self, _: Entity) {}
}
