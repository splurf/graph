use std::{collections::HashMap, rc::Rc};

pub type Vertex<V, E> = RawVertex<Rc<V>, Rc<E>>;

#[derive(Debug)]
pub struct RawVertex<V, E> {
    element: V,
    incoming: HashMap<V, E>,
    outgoing: HashMap<V, E>,
}

impl<V, E> RawVertex<V, E> {
    pub fn new(element: V) -> Self {
        Self {
            element,
            incoming: HashMap::default(),
            outgoing: HashMap::default(),
        }
    }

    pub fn element(&self) -> &V {
        &self.element
    }

    pub fn _incoming(&self) -> &HashMap<V, E> {
        &self.incoming
    }

    pub fn outgoing(&self) -> &HashMap<V, E> {
        &self.outgoing
    }

    pub fn incoming_mut(&mut self) -> &mut HashMap<V, E> {
        &mut self.incoming
    }

    pub fn outgoing_mut(&mut self) -> &mut HashMap<V, E> {
        &mut self.outgoing
    }
}
