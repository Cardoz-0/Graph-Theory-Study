use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

struct Edge {
    v_pos: i32,
    u_pos: i32,
    w: f32,
}

struct Vertex {
    name: String
} 

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new()
        }
    }
    
    pub fn load(path: String) -> Graph {
        let file = fs::File::open(path).expect("Não foi possível carregar arquivo");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Não foi possível transformar arquivo em String");
        let mut graph = Graph::new();
        let split = contents.split("\n");
        let mut vertices_finished = false;
        let mut is_directed = true;
        let graph_ref = &mut graph;
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
                let vec: Vec<&str> = line.splitn(2, " ").collect();
                graph_ref.add_vertex(Vertex{name: vec[1].to_string()});
            } else {
                let vec: Vec<&str> = line.splitn(3, " ").collect();
                graph_ref.add_edge(vec[0].parse::<i32>().unwrap(),
                                    vec[1].parse::<i32>().unwrap(),
                                    vec[2].parse::<f32>().unwrap());
                    
                if !is_directed {graph_ref.add_edge(vec[1].parse::<i32>().unwrap(),
                                    vec[0].parse::<i32>().unwrap(),
                                    vec[2].parse::<f32>().unwrap())}
            }
        }
        graph
    }

    pub fn add_vertex(&mut self, v: Vertex) -> () {
        self.vertices.push(v);
    }

    pub fn add_edge(&mut self, v_pos: i32, u_pos: i32, w: f32) -> () {
        self.edges.push(Edge{v_pos: v_pos-1, u_pos: u_pos-1,w});        
    }

    pub fn get_neighbours(self, v_pos: i32, result: &mut Vec<Edge>) -> () {
        for edge in self.edges {
            if edge.v_pos == v_pos {
                result.push(edge)
            }
        }
    }
}

fn main() {
    let path = String::from("./../tests/dirigidos/manha.net");
    let test = Graph::load(path);
    println!("Arquivo carregado com sucesso!");
    let mut neighbours: Vec<Edge> = Vec::new();
    test.get_neighbours(0,&mut neighbours);
    if let Some(edge) = neighbours.pop() {
        println!("Achou vizinho {}", edge.u_pos);
    }
}
