use std::collections::HashMap;

pub struct Node<T> {
    id: (i32, i32),
    pub connections: Vec<(i32, i32)>,
    pub data: T
}

impl<T> Node<T> {
    fn new(position: (i32, i32), connection: (i32, i32), data: T) -> Self {
        Self { id: position, connections: vec![connection], data }
    }
}

pub struct Graph<T> {
    pub nodes: HashMap<(i32, i32), Node<T>>
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }
}

impl<T> Graph<T> {
    pub fn add(&mut self, connection: (i32, i32), position: (i32, i32), data: T) {
        if !self.nodes.contains_key(&connection) {
            return;
        }

        self.nodes.insert(position, Node::new(position, connection, data));
        self.nodes.get_mut(&connection).unwrap().connections.push(position);
    }

    pub fn get(&mut self, position: (i32, i32)) -> Option<&T> {
        if !self.nodes.contains_key(&position) {
            return None
        } 
        
        Some(&self.nodes.get_mut(&position).unwrap().data)
    }

    pub fn get_mut(&mut self, position: (i32, i32)) -> Option<&mut T> {
        if !self.nodes.contains_key(&position) {
            return None
        } 
        
        Some(&mut self.nodes.get_mut(&position).unwrap().data)
    }
}
