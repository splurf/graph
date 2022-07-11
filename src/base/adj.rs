use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};

type Vertex<V, E> = RawVertex<Rc<V>, Rc<E>>;

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
}

#[derive(Default, Debug)]
pub struct AdjacencyList<V, E> {
    data: HashMap<Rc<V>, Rc<RefCell<Vertex<V, E>>>>,
}

impl<V: Eq + Hash, E> AdjacencyList<V, E> {
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
        if u.outgoing.contains_key(&v.element) {
            None
        } else {
            let edge = Rc::new(element);
            (&mut u.outgoing).insert(v.element.clone(), edge.clone());
            (&mut v.incoming).insert(u.element.clone(), edge.clone());
            Some(edge)
        }
    }

    /**
     * Return the edge between two verticies if it exists
     */
    pub fn adjacent(&self, u: Ref<Vertex<V, E>>, v: Ref<Vertex<V, E>>) -> Option<Rc<E>> {
        u.outgoing.get(&v.element).cloned()
    }
}
