use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;

#[derive(PartialEq)]
struct Vertex {
    id: usize,
    name: String,
}

struct Edge {
    w: f32,
    v: Rc<Vertex>,
    u: Rc<Vertex>,
}

struct Graph {
    verts: Vec<Rc<Vertex>>,
    edges: Vec<Rc<Edge>>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            verts: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, v: Rc<Vertex>) {
        self.verts.push(v);
    }

    fn add_edge(&mut self, e: Rc<Edge>) {
        self.edges.push(e)
    }
    
    fn find_by_id(&mut self, id: usize) -> Rc<Vertex> {
        self.verts.iter().find(|v| v.id == id).cloned().unwrap()
    }

    pub fn load(&mut self, path: String) {
        let file = fs::File::open(path).expect("Não foi possível carregar arquivo");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Não foi possível transformar arquivo em String");
        let split = contents.split("\n");
        let mut vertices_finished = false;
        let mut is_directed = true;

        for line in split {
            if line.contains("*vertices") {
                continue;
            }
            if line.contains("*arcs") {
                is_directed = false;
                vertices_finished = true;
                continue;
            }
            if line.contains("*edges") {
                vertices_finished = true;
                continue;
            }

            if !vertices_finished {
                let mut vec: Vec<&str> = line.splitn(2, " ").collect();
                let mut v = Vertex{id: vec[0].parse::<usize>().unwrap(), name: vec[1].to_string()};
                self.add_vertex(Rc::new(v));
            } else {
                let vec: Vec<&str> = line.splitn(3, " ").collect();
                let v = self.find_by_id(vec[0].parse::<usize>().unwrap());
                let u = self.find_by_id(vec[1].parse::<usize>().unwrap());
                let w = vec[2].parse::<f32>().unwrap();
                self.add_edge(Rc::new(Edge{v: Rc::clone(&v), u: Rc::clone(&u), w}));
                if !is_directed { self.add_edge(Rc::new(Edge{v: Rc::clone(&u), u: Rc::clone(&v), w: w })) }
            }
        }
    }

    pub fn get_edges(&self, v: Rc<Vertex>, result: &mut Vec<Rc<Edge>>) {   
        for edge in &self.edges {
            if edge.v == v {
                result.push(Rc::clone(&edge));
            }
        }
    }

    pub fn get_neighbours(&self, v: Rc<Vertex>, result: &mut Vec<Rc<Vertex>>) {
        let mut collector = Vec::new();
        self.get_edges(v, &mut collector);
        for edge in collector {
            result.push(Rc::clone(&edge.u));
        }
    }
}

fn main() {
    let path = String::from("./../tests/dirigidos/manha.net");
    let mut graph = Graph::new();
    graph.load(path);
    println!("Arquivo carregado com sucesso!");
    let mut neighbours: Vec<Edge> = Vec::new();
    //test.get_neighbours(0,&mut neighbours);
    //if let Some(edge) = neighbours.pop() {
     //   println!("Achou vizinho {}", edge.u_pos);
    //}
}
