use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;
use std::ops::Deref;
use std::cmp::min;

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

struct VisitableNode {
    v: Rc<Vertex>,
    visited: bool,
    on_stack: bool,
    lowest: usize,
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
    
    fn find_by_id(&self, id: usize) -> Rc<Vertex> {
        self.verts.iter().find(|v| v.id == id).cloned().unwrap()
    }

    pub fn load(&mut self, path: String) {
        let file = fs::File::open(path).expect("Não foi possível carregar arquivo");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Não foi possível transformar arquivo em String");
        let split = contents.split('\n');
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
                let vec: Vec<&str> = line.splitn(2, ' ').collect();
                let v = Vertex{id: vec[0].parse::<usize>().unwrap(), name: vec[1].to_string()};
                self.add_vertex(Rc::new(v));
            } else {
                let vec: Vec<&str> = line.splitn(3, ' ').collect();
                let v = self.find_by_id(vec[0].parse::<usize>().unwrap());
                let u = self.find_by_id(vec[1].parse::<usize>().unwrap());
                let w = vec[2].parse::<f32>().unwrap();
                self.add_edge(Rc::new(Edge{v: Rc::clone(&v), u: Rc::clone(&u), w}));
            }
        }
    }

    pub fn get_edges(&self, v: Rc<Vertex>) ->  Vec<Rc<Edge>> {
        let mut edges: Vec<Rc<Edge>> = vec![];
        for edge in &self.edges {
            if edge.v == v {
                edges.push(Rc::clone(edge));
            }
        }
        edges
    }

    pub fn get_neighbours(&self, v: Rc<Vertex>) -> Vec<Rc<Vertex>> {
        let mut result = Vec::new();
        for edge in self.get_edges(v) {
            result.push(Rc::clone(&edge.u));
        }
        result
    }
    
    pub fn detect_scc(&self) -> () {
        let mut stack: Vec<VisitableNode> = Vec::new();
        
        for v in &self.verts {
            stack.push(VisitableNode{
                v: Rc::clone(&v),
                visited: false,
                on_stack: false,
                lowest: 1 // Arquivos de testes utilizam 1 para indexar o primeiro elemento
            })
        }
        for i in 0..stack.len() {
            if !stack[i].visited {
                self.dfs_scc(i, &mut stack);
            }
        }
        for i in 0..stack.len() {
            println!("{}", stack[i].lowest);
        }
    }
    
    fn dfs_scc (&self, at: usize  ,stack:&mut Vec<VisitableNode>) {
        let mut node = &mut stack[at];
        if node.visited == false {
            node.visited = true;
            node.on_stack = true;
            node.lowest = node.v.id; 
            
            for neighbour in self.get_neighbours(Rc::clone(&stack[at].v)) {
                for to in 0..stack.len() {
                    if neighbour.id == stack.get(to).unwrap().deref().v.id {
                        if !stack[to].visited {
                            self.dfs_scc(to, stack);
                        }
                        if stack[to].on_stack { 
                            stack[at].lowest = min(stack[at].lowest, stack[to].lowest);
                        } 
                        break;
                    }
                }
            }

            if stack[at].lowest == stack[at].v.id {
                for i in 0..stack.len() {
                    if stack[i].on_stack {
                        stack[i].on_stack == false;
                        stack[i].lowest = stack[at].v.id;
                        if i == at {
                            break;
                        }
                    }

                }
            }
        }
    }
}

fn main() {
    let path = String::from("./../tests/dirigidos/manha.net");
    let mut graph = Graph::new();
    graph.load(path);
    println!("Arquivo carregado com sucesso!");
    let some_v = graph.find_by_id(1);
    let neighbours = graph.get_neighbours(some_v.clone());
    for v in neighbours {
        println!("Vertex: {}, Neighbors{}", some_v.name, v.name)
    }
    graph.detect_scc();
}

