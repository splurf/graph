use {
    super::Vertex,
    std::{
        cell::{Ref, RefCell, RefMut},
        collections::HashMap,
        hash::Hash,
        rc::Rc,
    },
};

#[derive(Default, Debug)]
pub struct Graph<V, E> {
    data: HashMap<Rc<V>, Rc<RefCell<Vertex<V, E>>>>,
}

impl<V: Eq + Hash, E> Graph<V, E> {
    /**
     * Add a new vertex to the graph if the value does the exist
     */
    pub fn add_vertex(&mut self, element: V) -> Option<Rc<RefCell<Vertex<V, E>>>> {
        if self.data.contains_key(&element) {
            None
        } else {
            let key = Rc::new(element);
            let raw = Rc::new(RefCell::new(Vertex::new(key.clone())));
            self.data.insert(key.clone(), raw.clone());
            Some(raw)
        }
    }

    /**
     * Insert an edge between the provided vertices if there is no edge
     */
    pub fn add_edge(
        &mut self,
        mut u: RefMut<Vertex<V, E>>,
        mut v: RefMut<Vertex<V, E>>,
        element: E,
    ) -> Option<Rc<E>> {
        if u.outgoing().contains_key(v.element()) {
            None
        } else {
            let edge = Rc::new(element);
            u.outgoing_mut().insert(v.element().clone(), edge.clone());
            v.incoming_mut().insert(u.element().clone(), edge.clone());
            Some(edge)
        }
    }

    /**
     * Return the edge between two verticies if it exists
     */
    pub fn adjacent(&self, u: Ref<Vertex<V, E>>, v: Ref<Vertex<V, E>>) -> Option<Rc<E>> {
        u.outgoing().get(v.element()).cloned()
    }
}
