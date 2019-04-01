use super::*;

#[derive(Clone,Copy,Eq,Serialize,Deserialize)]
pub struct Edge {
    pub id:    i32,
    pub start: i32,
    pub end:   i32,
    pub weight:i32,

}

js_serializable!(Edge);
js_deserializable!(Edge);

impl AsIndexForGraph for Edge {
    fn index(&self) -> &i32 {&(self.id)}
}

impl Ord for Edge {
    fn cmp(&self, other:&Edge) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self,other:&Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self,other:&Edge) -> bool {
        self.id == other.id
    }
}

impl Edge {
    pub fn new(nid:i32,ns:i32,ne:i32,nw:i32) -> Edge {
        Edge{id:nid,start:ns,end:ne,weight:nw}
    }

    pub fn set_weight(&mut self, w:i32) {
        self.weight = w;
    }
}
