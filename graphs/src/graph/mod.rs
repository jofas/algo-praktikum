extern crate ui;
use ui::JDraw;

use std::collections::HashMap;
use std::iter::FromIterator;
pub use std::cmp::Ordering;

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

#[derive(Clone)]
pub struct Graph {
    vs   : HashMap<i32,Vertex>,
    es   : HashMap<i32,Edge>,
    pub draw : Vec<JDraw>,
    e_id: i32,
}

impl Graph {

    // Graph()

    pub fn new() -> Graph {
        Graph{
            vs:HashMap::new(),
            es:HashMap::new(),
            draw:Vec::new(),
            e_id:0

        }
    }

    // Vertex Operations

    /*
    fn contains_vertex<I:AsIndexForGraph>(&self,i:&I) -> bool {
        match self.vs.get(i.index()) {
            Some(_) => true,
            None    => false,
        }
    }
    */

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

    pub fn get_vertices(&self) -> Vec<&Vertex> {
        Vec::from_iter(self.vs.values())
    }

    // Edge Operations
    /*
    fn contains_edge<I:AsIndexForGraph>(&self,i:&I) -> bool {
        match self.es.get(i.index()) {
            Some(_) => true,
            None    => false,
        }
    }

    pub fn get_edge<I:AsIndexForGraph>(&self,i:&I) -> Option<&Edge> {
        self.es.get(i.index())
    }

    fn get_edge_mut<I:AsIndexForGraph>(&mut self,i:&I) -> Option<&mut Edge> {
        self.es.get_mut(i.index())
    }
    */
    /*
    fn find_edge<I:AsIndexForGraph,J:AsIndexForGraph>(&self,from:&I,to:&J) -> Option<&Edge> {
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
    pub fn add_edge(&mut self,e:Edge) {
        self.es.insert(e.id,e);
        let s = self.get_vertex(&e.start).unwrap();
        let e = self.get_vertex(&e.end).unwrap();
        self.draw.push(JDraw::new(s.x,s.y,e.x,e.y));
    }

    /*
    fn add_edge_from_to<I:AsIndexForGraph>(&mut self, from:&I, to:&I, id:Option<i32>) {
        self.add_edge_from_to_weight(from,to,id,0);
    }

    fn add_edge_from_to_weight<I:AsIndexForGraph>(&mut self, from:&I, to:&I, id:Option<i32>, weight:i32) {
        if self.contains_vertex(from) && self.contains_vertex(to) {
            let id = match id {
                Some(i) => i,
                None    => self.e_id
            };
            let n = Edge::new(id,*from.index(),*to.index(),weight);
            self.e_id = id + 1;
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
    /*
    fn adjacent<I:AsIndexForGraph,J:AsIndexForGraph>(&self, x:&I, y:&J) -> bool {
        if let Some(_) = self.find_edge(x,y) { return true;}
        false
    }

    fn neighbours<I:AsIndexForGraph>(&self, i:&I) -> Option<Vec<&Vertex>> {
        if self.contains_vertex(i.index()) {
            let mut ret:Vec<&Vertex> = Vec::new();
            for v in self.get_vertices() {
                if self.adjacent(i,v) {
                    ret.push(v)
                }
            }
            return Some(ret);
        }
        None
    }
    */
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
    /*
    pub fn to_drawable(&self) -> Vec<(i32,i32,i32,i32)> {
        let edges = self.get_edges();
        edges.iter().map(|x| {
            let s = self.get_vertex(&x.start).unwrap();
            let e = self.get_vertex(&x.end).unwrap();
            (s.x,s.y,e.x,e.y)
        }).collect()
    }
    */
}

