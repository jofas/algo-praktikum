//extern crate regex;
//use self::regex::Regex;

extern crate serde_json;
use self::serde_json::Value;

//use std::fs::{self,File};
//use std::io::Read;
//use std::path::Path;

use super::*;
/*
static PATH_TO_GRAPHS: &'static str = "./www/graphs";
static IMPORTABLE: &'static str = r"graph(?P<n>(\d+|\d+_\d+_))_(?P<s>\d+)\.json";

pub fn list_all_graphs() -> Option<Vec<String>> {
    let p = Path::new(PATH_TO_GRAPHS);
    let imp = Regex::new(IMPORTABLE).unwrap();
    let mut gn:Vec<String> = Vec::new();
    for entry in fs::read_dir(p).ok()? {
        let file_path = entry.ok()?.path();
        let file_name = file_path.to_str()?;
        if imp.is_match(file_name) {
            let capture = imp.captures(file_name).unwrap();
            let name = String::from(&capture["n"]) + " " + &capture["s"];
            gn.push(name);
        }
    }
    Some(gn)
}

pub fn build_graphs() -> Option<Vec<Graph>>{
    let p = Path::new(PATH_TO_GRAPHS);
    let importable = Regex::new(r"graph(?P<n>(\d+|\d+_\d+_))_.+\.json").unwrap();

    let mut graphs:Vec<Graph> = Vec::new();

    for entry in fs::read_dir(p).ok()? {
        let file_path = entry.ok()?.path();
        let file_name = file_path.to_str()?;

        if importable.is_match(file_name) {
            let mut file = File::open(file_name).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let capture = importable.captures(file_name).unwrap();

            if let Some(g) = json_to_graph(String::from(&capture["n"]),data.as_str()) {
                graphs.push(g);
            }
            println!("{}",file_name);
        }
    }
    Some(graphs)
}
*/
pub fn json_to_graph(data:&str) -> Option<Graph> {
    let val:Value = serde_json::from_str(data).ok()?;
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
        let start = e["start"].as_i64()?;
        let end   = e["end"].as_i64()?;
        g.add_edge(Edge::new(id as i32, start as i32, end as i32, 0));
    }

    Some(g)
}
