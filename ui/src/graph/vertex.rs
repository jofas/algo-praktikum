use super::*;

#[derive(Clone,Copy,Eq,Serialize,Deserialize)]
pub struct Vertex {
    pub id:i32,
    pub x: i32,
    pub y: i32,
    // fuer Dijkstra
    pub dist:i32,
    pub parent:Option<i32>,
}

js_serializable!(Vertex);
js_deserializable!(Vertex);

impl AsIndexForGraph for Vertex {
    fn index(&self) -> &i32 {&(self.id)}
}

impl Ord for Vertex {
    fn cmp(&self, other:&Vertex) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self,other:&Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertex {
    fn eq(&self,other:&Vertex) -> bool {
        self.dist == other.dist
    }
}

impl Vertex {
    pub fn new(i:i32,xn:i32,yn:i32) -> Vertex {
        Vertex{id:i,x:xn,y:yn,dist:<i32>::min_value(),parent:None}
    }

    pub fn set_dist(&mut self,nd:i32) {
        self.dist = nd;
    }

    pub fn set_parent(&mut self,np:i32) {
        self.parent = Some(np);
    }

    pub fn to_svg(&self) -> String {
        format!("<circle
            id='{}'
            cx='{}'
            cy='{}'
            r ='5'
            fill='green'/>",
            self.id,self.x,self.y)
    }
}
