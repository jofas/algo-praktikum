use super::*;

#[derive(Clone,Copy,Eq)]
pub struct Vertex {
    pub id:i32,
    pub x: i32,
    pub y: i32,
}

impl AsIndexForGraph for Vertex {
    fn index(&self) -> &i32 {&(self.id)}
}

impl Ord for Vertex {
    fn cmp(&self, other:&Vertex) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self,other:&Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self,other:&Vertex) -> bool {
        self.id == other.id
    }
}

impl Vertex {
    pub fn new(i:i32,xn:i32,yn:i32) -> Vertex {
        Vertex{id:i,x:xn,y:yn}
    }
}
