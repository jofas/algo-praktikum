#![feature(nll)]
#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
use self::serde_json::Value;

mod graph;
use graph::{Graph,Vertex,Edge,DijkstraState,AStarState};

static SVG_HEAD: &'static str = "
<svg version='1.1'
     xmlns='http://www.w3.org/2000/svg'
     width='1024' height='768'
     id='graph'>";
static SVG_TAIL: &'static str = "</svg>";

fn json_to_graph(data:String) -> Option<Graph> {
    let val:Value = serde_json::from_str(&data).ok()?;
    let mut g = Graph::new();

    // Vertices
    for v in val["vertices"].as_array()? {
        let id = v["id"].as_i64()?;
        let x  = v["position"]["x"].as_i64()?;
        let y  = v["position"]["y"].as_i64()?;
        g.add_vertex(Vertex::new(id as i32, x as i32, y as i32));
    }

    // Edges
    for e in val["edges"].as_array()? {
        let id    = e["id"].as_i64()?;
        let start = if let Some(s) = e["start"].as_i64() {s}
                    else if let Some(s) = e["start"]["id"].as_i64(){s}
                    else {return None;};
        let end   = if let Some(e) = e["end"].as_i64() {e}
                    else if let Some(e) = e["end"]["id"].as_i64() {e}
                    else {return None;};
        let e = Edge::new(id as i32,start as i32, end as i32,0);
        if g.valid_edge(&e) {
            g.add_edge(e);
        } else {return None;}
    }
    Some(g)
}

#[derive(Serialize,Deserialize,Clone)]
pub struct JDraw {
    id:i32,
    ax:i32,
    ay:i32,
    bx:i32,
    by:i32
}

impl JDraw {
    pub fn new(nid:i32,nax:i32,nay:i32,nbx:i32,nby:i32) -> JDraw {
        JDraw{id:nid,ax:nax,ay:nay,bx:nbx,by:nby}
    }

    pub fn to_svg(&self) -> String {
        format!("<line
            id='{}'
            x1='{}'
            y1='{}'
            x2='{}'
            y2='{}'
            style='stroke:black'/>",
        self.id,self.ax,self.ay,self.bx,self.by)
    }
}

js_serializable!(JDraw);
js_deserializable!(JDraw);

fn draw(g:stdweb::Value) -> String {
    let g = Graph::from(g);
    let svg_c = g.d.iter().fold(SVG_HEAD.to_owned(),|acc,x| {
        acc + &x.to_svg()
    });
    let svg_c = g.get_vertices_owned().iter().fold(svg_c,|acc,x| {
        acc + &x.to_svg()
    });
    String::from(svg_c + SVG_TAIL)
}

fn a_star_init(g:stdweb::Value,start:i32,end:i32) -> AStarState {
    let mut g = Graph::from(g);
    g.get_vertex_mut(&start).unwrap().set_dist(0);

    let c = g.clone();

    AStarState::from((c,vec![g.get_vertex(&start).unwrap().clone()],Vec::new(),start,end))
}

fn a_star_next(ass:stdweb::Value) -> AStarState {
    let mut ass = AStarState::from(ass);

    let ret = ass.a_star_to_draw();
    if let Some((x,Some(xp),last)) = ret {
        let line = ass.graph.find_edge(&xp,&x).unwrap().id;
        for l in &ass.graph.d {
            if l.id == line {
                let new_line = format!("<line
                    id='{}_new'
                    x1='{}'
                    y1='{}'
                    x2='{}'
                    y2='{}'
                    style='stroke:green;stroke-width:3'/>",
                    l.id,l.ax,l.ay,l.bx,l.by);
                js!{
                    var svg = document.getElementById("graph");
                    svg.innerHTML += @{new_line};
                }
                break;
            }
        }
        if last {
            js!{
                document.getElementById("a_star_next").remove();
                var inf = document.getElementById("a_star_info");
                inf.innerHTML = "Verbindung gefunden."
            }
        }
    } else if let None = ret {
        js!{
            document.getElementById("a_star_next").remove();
            var inf = document.getElementById("a_star_info");
            inf.innerHTML = "Es gibt keine Verbindung von "+@{ass.start}+" zu "+@{ass.end};
        }
    }
    ass
}


fn dijkstra_init(g:stdweb::Value,start:i32) -> DijkstraState {
    let mut g = Graph::from(g);
    g.get_vertex_mut(&start).unwrap().set_dist(0);

    let mut print = g.get_vertices();
    print.sort_by(|a,b| a.id.cmp(&b.id));
    print.iter().for_each(|x| {
        let d = if x.dist == <i32>::min_value() {String::from("&#8734;")} else {x.dist.to_string()};
        let p = format!("<p id='p_{}'>ID: {} Distanz: {}</p>",x.id,x.id,d);
        js! {document.getElementById("vertex_dist").innerHTML += @{p};}
    });

    let c = g.clone();

    DijkstraState::from((g,c.get_vertices_owned()))
}

fn dijkstra_next(ds:stdweb::Value) -> DijkstraState {
    let mut ds = DijkstraState::from(ds);

    let ret = ds.dijkstra_to_draw();
    if let (x,Some(xp),last) = ret {
        let line = ds.graph.find_edge(&xp,&x).unwrap().id;
        for l in &ds.graph.d {
            if l.id == line {
                let new_line = format!("<line
                    id='{}_new'
                    x1='{}'
                    y1='{}'
                    x2='{}'
                    y2='{}'
                    style='stroke:green;stroke-width:3'/>",
                    l.id,l.ax,l.ay,l.bx,l.by);
                let v = ds.graph.get_vertex(&x).unwrap();
                js!{
                    var p = document.getElementById("p_"+@{&x}.toString());
                    p.innerHTML="ID: "+@{&x}.toString()+" Distanz: "+@{&v.dist.abs()}.toString();

                    var svg = document.getElementById("graph");
                    svg.innerHTML += @{new_line};
                }
                break;
            }
        }
        if last {
            js!{document.getElementById("dijkstra_next").remove();}
        }
    }
    ds
}

fn main() {
    stdweb::initialize();
    js! {
        Module.exports.dijkstra_init = @{dijkstra_init};
        Module.exports.dijkstra_next = @{dijkstra_next};
        Module.exports.a_star_init = @{a_star_init};
        Module.exports.a_star_next = @{a_star_next};
        Module.exports.new = @{json_to_graph};
        Module.exports.draw = @{draw};
    }
}
