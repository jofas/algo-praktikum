use super::*;

use std::collections::{HashMap,BinaryHeap};
use std::iter::FromIterator;
use std::cmp::Ordering;

use stdweb::unstable::TryInto;


mod vertex;
pub use self::vertex::Vertex;

mod edge;
pub use self::edge::Edge;

pub trait AsIndexForGraph {
    fn index(&self) -> &i32;
}

impl AsIndexForGraph for i32 {
    fn index(&self) -> &i32 {&self}
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Graph {
    vs   : HashMap<i32,Vertex>,
    es   : HashMap<i32,Edge>,
    pub d : Vec<JDraw>,
}

js_serializable!(Graph);
js_deserializable!(Graph);

impl From<(HashMap<i32,Vertex>,HashMap<i32,Edge>,Vec<JDraw>)> for Graph {
    fn from(x:(HashMap<i32,Vertex>,HashMap<i32,Edge>,Vec<JDraw>)) -> Graph {
        Graph{vs:x.0,es:x.1,d:x.2}
    }
}

impl From<stdweb::Value> for Graph {
    fn from(g:stdweb::Value) -> Graph {
        let d_len:i32 = js! {return @{&g}["d"].length}
            .try_into().unwrap();

        let mut nd:Vec<JDraw> = Vec::new();
        for i in 0..d_len {
            nd.push(js! {return @{&g}["d"][@{i}];}
                .try_into().unwrap());
        }

        let vs_len:i32 = js! {
            var i = 0;
            for (var x in @{&g}["vs"]) {i += 1;}
            return i;
        }.try_into().unwrap();

        let mut nvs:HashMap<i32,Vertex> = HashMap::new();
        for i in 0..vs_len {
            let v:Vertex = js! {
                return @{&g}["vs"][@{i}];
            }.try_into().unwrap();
            nvs.insert(v.id,v);
        }

        let es_len:i32 = js! {
            var i = 0;
            for (var x in @{&g}["es"]) {i += 1;}
            return i;
        }.try_into().unwrap();

        let mut nes:HashMap<i32,Edge> = HashMap::new();
        for i in 0..es_len {
            let e:Edge = js!{return @{&g}["es"][@{i}];}
                .try_into().unwrap();
            nes.insert(e.id,e);
        }

        Graph{vs:nvs,es:nes,d:nd}
    }
}

impl Graph {

    // Graph()

    pub fn new() -> Graph {
        Graph{
            vs:HashMap::new(),
            es:HashMap::new(),
            d:Vec::new(),
        }
    }

    // Vertex Operations

    fn contains_vertex<I:AsIndexForGraph>(&self,i:&I) -> bool {
        match self.vs.get(i.index()) {
            Some(_) => true,
            None    => false,
        }
    }

    pub fn add_vertex(&mut self,v:Vertex) {
        self.vs.insert(v.id,v);
    }
    /*
    fn remove_vertex<I:AsIndexForGraph>(&mut self,i:&I) {
        let c = self.vs.clone();
        match c.get(i.index()) {
            Some(v) => {
                let c = self.clone();
                if let Some(n) = c.neighbours(v) {
                    n.iter().for_each(|x| self.remove_edge_from_to(v,x));
                };
                self.vs.remove(i.index());
            },
            None    => println!("No Vertex: {}",i.index()),
        }
        // KANTEN LOESCHEN
    }
    */
    pub fn get_vertex<I:AsIndexForGraph>(&self,i:&I) -> Option<&Vertex> {
        self.vs.get(i.index())
    }

    pub fn get_vertex_mut<I:AsIndexForGraph>(&mut self,i:&I) -> Option<&mut Vertex> {
        self.vs.get_mut(i.index())
    }

    pub fn get_vertices(&self) -> Vec<&Vertex> {
        Vec::from_iter(self.vs.values())
    }

    pub fn get_vertices_mut(&mut self) -> Vec<&mut Vertex> {
        Vec::from_iter(self.vs.values_mut())
    }

    pub fn get_vertices_owned(&self) -> Vec<Vertex> {
        self.get_vertices().iter().map(|x| **x).collect()
    }

    // Edge Operations
    /*
    fn contains_edge<I:AsIndexForGraph>(&self,i:&I) -> bool {
        match self.es.get(i.index()) {
            Some(_) => true,
            None    => false,
        }
    }
    */
    pub fn get_edge<I:AsIndexForGraph>(&self,i:&I) -> Option<&Edge> {
        self.es.get(i.index())
    }
    /*
    fn get_edge_mut<I:AsIndexForGraph>(&mut self,i:&I) -> Option<&mut Edge> {
        self.es.get_mut(i.index())
    }
    */
    pub fn find_edge<I:AsIndexForGraph,J:AsIndexForGraph>(&self,from:&I,to:&J) -> Option<&Edge> {
        if self.contains_vertex(from) && self.contains_vertex(to) {
            let ret:Vec<&Edge> = self.es.values().filter(
                |edge| edge.start.index() == from.index() && edge.end.index() == to.index()
            ).collect();
            if !(ret.is_empty()) {
                return self.get_edge(ret[0]);
            }
        }
        None
    }
    /*
    fn find_edge_mut<I:AsIndexForGraph,J:AsIndexForGraph>(&mut self,from:&I,to:&J) -> Option<&mut Edge> {
        if self.contains_vertex(from) && self.contains_vertex(to) {
            let c = self.es.clone();
            let ret:Vec<&Edge> = c.values().filter(
                |edge| edge.start.index() == from.index() && edge.end.index() == to.index()
            ).collect();
            if !(ret.is_empty()) {
                return self.get_edge_mut(ret[0]);
            }
        }
        None
    }
    */
    pub fn valid_edge(&self,e:&Edge) -> bool {
        self.contains_vertex(&e.start) && self.contains_vertex(&e.end)
    }
    pub fn add_edge(&mut self,e:Edge) {
        self.es.insert(e.id,e);
        let s = self.get_vertex(&e.start).unwrap();
        let end = self.get_vertex(&e.end).unwrap();
        self.d.push(JDraw::new(e.id,s.x,s.y,end.x,end.y));
    }

    /*
    fn add_edge_from_to<I:AsIndexForGraph>(&mut self, from:&I, to:&I, id:i32) {
        self.add_edge_from_to_weight(from,to,id,0);
    }

    fn add_edge_from_to_weight<I:AsIndexForGraph>(&mut self, from:&I, to:&I, id:i32, weight:i32) {
        if self.contains_vertex(from) && self.contains_vertex(to) {
            let n = Edge::new(id,*from.index(),*to.index(),weight);
            self.es.insert(id,n);
        }
    }

    fn remove_edge<I:AsIndexForGraph>(&mut self,i:&I) {
        let c = self.es.clone();
        match c.get(i.index()) {
            Some(e) => {self.es.remove(i.index());},
            None    => println!("No Edge: {}",i.index()),
        }
    }

    fn remove_edge_from_to<I:AsIndexForGraph>(&mut self, from:&I, to:&I) {
        let c = self.clone();
        if let Some(e) = c.find_edge(from,to) {
            self.remove_edge(e);
        }
    }

    pub fn get_edges(&self) -> Vec<&Edge> {
        Vec::from_iter(self.es.values())
    }
    */
    /*
    fn get_edge_weight<I:AsIndexForGraph>(&self, i:&I) -> Option<i32> {
        if let Some(e) = self.get_edge(i) {
            return Some(e.weight);
        }
        None
    }

    fn get_edge_weight_from_to<I:AsIndexForGraph>(&self, from:&I, to:&I) -> Option<i32> {
        if let Some(e) = self.find_edge(from,to) {
            return Some(e.weight);
        }
        None
    }

    fn set_edge_weight<I:AsIndexForGraph>(&mut self, i:&I, w:i32) {
        if let Some(e) = self.get_edge_mut(i) {
            e.set_weight(w);
        }
    }

    fn set_edge_weight_from_to<I:AsIndexForGraph>(&mut self, from:&I, to:&I, w:i32) {
        if let Some(e) = self.find_edge_mut(from,to) {
            e.set_weight(w);
        }
    }
    */
    // Anfragen
    fn adjacent<I:AsIndexForGraph,J:AsIndexForGraph>(&self, x:&I, y:&J) -> bool {
        if let Some(_) = self.find_edge(x,y) { return true;}
        false
    }

    fn neighbours<I:AsIndexForGraph>(&mut self, i:&I) -> Option<Vec<&mut Vertex>> {
        if self.contains_vertex(i.index()) {
            let mut ret:Vec<&mut Vertex> = Vec::new();
            let c = self.clone();
            for v in self.get_vertices_mut() {
                if c.adjacent(i,v) {
                    ret.push(v)
                }
            }
            return Some(ret);
        }
        None
    }
    // Ausgabe
    /*
    fn cli_out(&self) {
        println!("Knotenliste:");
        self.vs.iter().for_each(|(id,_)| {
            println!("ID: {}",id);
        });

        println!("Kantenliste:");
        self.es.iter().for_each(|(id,e)| {
            println!("ID: {}, S: {} -> E: {}", id, e.start, e.end);
        });
    }
    */
}

fn decrease_key_with_parent(q:&mut BinaryHeap<Vertex>,v:&Vertex,u:&Vertex,nk:i32) {
    let mut nq = q.clone().into_vec();
    for x in &mut nq {
        if x.id == v.id {
            x.dist = nk;
            x.parent = Some(u.id);
            break;
        }
    }
    *q = BinaryHeap::from(nq);
}

fn weight(a:&Vertex,b:&Vertex) -> i32 {
    -((a.x as f32 - b.x as f32).powi(2) + (a.y as f32 - b.y as f32).powi(2)).sqrt() as i32
}

fn heuristik(a:&Vertex,start:&Vertex) -> i32 {
    weight(a,start)
}


#[derive(Serialize,Deserialize)]
pub struct DijkstraState {
    pub graph:Graph,
    pub heap_as_vec:Vec<Vertex>,
}

js_serializable!(DijkstraState);
js_deserializable!(DijkstraState);

impl From<stdweb::Value> for DijkstraState {
    fn from(v:stdweb::Value) -> DijkstraState {
        let g = Graph::from(js!{return @{&v}["graph"]});
        let hav:Vec<Vertex> = js!{return @{&v}["heap_as_vec"];}.try_into().unwrap();
        DijkstraState{graph:g,heap_as_vec:hav}
    }
}

impl From<(Graph,Vec<Vertex>)> for DijkstraState {
    fn from(v:(Graph,Vec<Vertex>)) -> DijkstraState {
        DijkstraState{graph:v.0,heap_as_vec:v.1}
    }
}

impl DijkstraState {
    pub fn dijkstra_to_draw(&mut self) -> (i32,Option<i32>,bool) {
        let mut q = BinaryHeap::from(self.heap_as_vec.clone());
        let u = q.pop().unwrap();

        let adj = self.graph.neighbours(&u.id).unwrap();
        for v in adj {
            let w = weight(&u,&v);
            js!{console.log(@{&v.id});}
            js!{console.log(@{&w});}
            if v.dist < u.dist + w {
                v.set_dist(u.dist + w);
                v.set_parent(u.id);
                decrease_key_with_parent(&mut q,v,&u,u.dist+w);
            }
        }
        let u_p = self.graph.get_vertex(&u).unwrap().parent;

        // Fuer JS Serialization Vec instead of BinaryHeap
        self.heap_as_vec = q.clone().into_vec();

        if let Some(&x) = q.peek() {
            if x.dist == <i32>::min_value() {(u.id,u_p,true)}
            else {(u.id,u_p,false)}
        } else {(u.id,u_p,true)}
    }
}

#[derive(Serialize,Deserialize)]
pub struct AStarState {
    pub graph:Graph,
    pub heap_as_vec:Vec<Vertex>,
    pub closed_list:Vec<i32>,
    pub start:i32,
    pub end:i32,
}

js_serializable!(AStarState);
js_deserializable!(AStarState);

impl From<stdweb::Value> for AStarState {
    fn from(v:stdweb::Value) -> AStarState {
        let g = Graph::from(js!{return @{&v}["graph"]});
        let hav:Vec<Vertex> = js!{return @{&v}["heap_as_vec"];}.try_into().unwrap();
        let cl:Vec<i32> = js!{return @{&v}["closed_list"];}.try_into().unwrap();
        let s:i32 = js!{return @{&v}["start"];}.try_into().unwrap();
        let e:i32 = js!{return @{&v}["end"];}.try_into().unwrap();
        AStarState{graph:g,heap_as_vec:hav,closed_list:cl,start:s,end:e}
    }
}

impl From<(Graph,Vec<Vertex>,Vec<i32>,i32,i32)> for AStarState {
    fn from(v:(Graph,Vec<Vertex>,Vec<i32>,i32,i32)) -> AStarState {
        AStarState{graph:v.0,heap_as_vec:v.1,closed_list:v.2,start:v.3,end:v.4}
    }
}

impl AStarState {
    pub fn a_star_to_draw(&mut self) -> Option<(i32,Option<i32>,bool)> {
        let mut q = BinaryHeap::from(self.heap_as_vec.clone());
        if let Some(u) = q.pop() {
            let u_p = self.graph.get_vertex(&u).unwrap().parent;
            if u.id == self.end {
                Some((u.id,u_p,true))
            } else {
                self.closed_list.push(u.id);
                self.expand_node(&mut q,&u);
                self.heap_as_vec = q.into_vec();
                Some((u.id,u_p,false))
            }
        } else {
            None
        }
    }

    fn expand_node(&mut self,q:&mut BinaryHeap<Vertex>,u:&Vertex) {
        let nq:Vec<i32> = q.clone().into_vec().iter().map(|x| x.id).collect();

        let c = self.graph.clone();
        let start = c.get_vertex(&self.start).unwrap();

        let adj = self.graph.neighbours(&u.id).unwrap();
        for v in adj {
            if self.closed_list.contains(&v.id) {continue;}

            let g = u.dist + weight(u,&v);

            if nq.contains(&v.id) && g <= v.dist {continue;}

            v.set_dist(g);
            v.set_parent(u.id);
            let f = g + heuristik(&v,start);

            if nq.contains(&v.id) {
                decrease_key_with_parent(q,v,u,f);
            } else {
                v.set_dist(f);
                q.push(*v);
            }
        }
    }
}
