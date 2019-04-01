#![feature(nll)]
//extern crate libc;
//use libc::{c_char,int32_t,size_t};

//use std::ffi::CString;
//use std::convert::From;

mod graph;
pub use graph::{Graph,Vertex,Edge};

mod build_graphs;
pub use build_graphs::*;


/*
#[repr(C)]
pub struct CArray {
    len: size_t,
    data: *const c_void,
}

impl From<Vec<Vertex>> for CArray {
    fn from(v:Vec<Vertex>) -> CArray {
        let mut cv:Vec<CVertex> = Vec::new();
        for x in v {
            cv.push(CVertex::from(x));
        }
        //v = v.iter().map(|x| CVertex::from(*x)).collect();
        cv.shrink_to_fit();
        CArray{len:cv.len() as size_t, data:cv.as_ptr() as *const c_void}
    }
}

#[repr(C)]
pub struct CVertex {
    id: int32_t,
    x : int32_t,
    y : int32_t
}

impl From<Vertex> for CVertex {
    fn from(v:Vertex) -> CVertex {
        CVertex{id:v.id,x:v.x,y:v.y}
    }
}

#[repr(C)]
pub struct Coordinates {
    x: int32_t,
    y: int32_t,
}

impl From<(i32,i32)> for Coordinates {
    fn from(t: (i32,i32)) -> Coordinates {
        Coordinates{x:t.0,y:t.1}
    }
}
*/
/*
#[no_mangle]
pub extern fn get_graphs() -> CArray {
    let graphs = build_graphs().unwrap();
    let g = &graphs[0];
    let vs = g.get_vertices();
    let mut cvs:Vec<Vertex> = Vec::new();
    for x in vs {
        cvs.push(*x)
    }
    CArray::from(cvs)
    //CVertex::from(cvs[0])

    //let e = g.get_edge(&0).unwrap();
    //let s = g.get_vertex(&e.start).unwrap();
    //let x = Coordinates::from((s.x,s.y));
    //println!("{}", x.x);
    //x
    let gs = graphs.iter().fold(String::new(), |acc,x| {
        acc + &x.name + ","
    });
    let c_gs = CString::new(gs).unwrap();
    c_gs.into_raw()
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}
}
