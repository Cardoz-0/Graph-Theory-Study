use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;
use std::collections::VecDeque;



#[derive(Clone, PartialEq)]
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
    edges: Vec<Rc<Edge>>,
}

struct AuxNodeSCC {
    node: VisitableNode,
    on_stack: bool,
    lowest: usize,
}

struct VisitableNode {
    v: Rc<Vertex>,
    visited: bool,
}

struct OrderedNode {
    visitable: VisitableNode,
    pos: usize,
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
    
    fn find_by_pos(&self, pos: usize) -> Rc<Vertex> {
        Rc::clone(&self.verts[pos])
    }
    
    pub fn load(&mut self, path: String) {
        let file = fs::File::open(path).expect("Não foi possível carregar arquivo");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("Não foi possível transformar arquivo em String");
        let split = contents.split('\n');
        let mut vertices_finished = false;

        for line in split {
            if line.contains("*vertices") {
                continue;
            }
            if line.contains("*arcs") {
                vertices_finished = true;
                continue;
            }
            if line.contains("*edges") {
                vertices_finished = true;
                continue;
            }

            if !vertices_finished {
                let vec: Vec<&str> = line.splitn(2, ' ').collect();

                let v = Vertex {
                    id: vec[0].parse::<usize>().unwrap()-1,
                    name: vec[1].to_string(),
                };
                self.add_vertex(Rc::new(v));
            } else {
                let vec: Vec<&str> = line.splitn(3, ' ').collect(); 
                if vec.len() == 1 { //caso o arquivo termine com um newline
                    continue;
                }
                
                let v = self.find_by_pos(vec[0].parse::<usize>().unwrap()-1);
                let u = self.find_by_pos(vec[1].parse::<usize>().unwrap()-1);
                let w = vec[2].parse::<f32>().unwrap();
                self.add_edge(Rc::new(Edge {
                    v: Rc::clone(&v),
                    u: Rc::clone(&u),
                    w,
                }));
            }
        }
    }

    pub fn get_edges(&self, v: Rc<Vertex>) -> Vec<Rc<Edge>> {
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

    pub fn weight(&self, u : Rc<Vertex>, v : Rc<Vertex>) -> f32 {
        let mut w = 0.0;
        if u == v {
            return 0.0;
        }
        for i in &self.edges {
            if v == i.v && u == i.u {
                break;
            }
            w = i.w;
        }
        return w;
    }  


    pub fn EdmondsKarp(&self, s: Rc<Vertex>, t: Rc<Vertex>, Gr: Graph) -> Option<VecDeque<Rc<Vertex>>> {
        let mut C = Rc::new(vec![s]);
        let mut A = Rc::new(Vec::with_capacity(self.verts.len()));

        let mut Q : VecDeque<Rc<Vertex>> = VecDeque::new();
        Q.push_front(t);

        while Q.len() > 0 {
        let u = Q.pop_back().unwrap();
        for v in self.get_neighbours(Rc::clone(&u)).iter() {
            if !C.contains(&v) && self.weight(Rc::clone(&v), u) > 0.0  {
                //A[self.find_by_id] = u;
                A[A.iter().position(|r| r == v).unwrap()] = u;      
               
                if *v == t {
                    let mut p : VecDeque<Rc<Vertex>> = VecDeque::from([t]);
                    let w = Rc::clone(&t);
                    let s0 = Rc::new(s);
                    while w != Rc::clone(&s0) {
                        w = A[self.verts.iter().position(|r| r == w).unwrap()];
                        p.push_front(Rc::clone(&w));
                    }
                return Some(p)
                }
                Q.push_back(Rc::clone(&v));
            }

        }

    }
    None 
}

pub fn FordFulkerson(&self, s: Rc<Vertex>, t: Rc<Vertex>) -> f32 {
    
    let mut MaxFlow : f32 = 0.0;
    let mut Grr : Graph = Graph::new();

    Grr.verts = self.verts;

    for i in self.edges.iter()
    { Grr.add_edge(Rc::clone(i)); }

    let p = self.EdmondsKarp(s, t, Grr).unwrap_or_default();

    while p.len() != 0 {
        let mut smallw : f32 = std::f32::MIN;
        for i in 0..p.len() - 1 {
            let mut actw = Grr.weight(p[i], p[i+1]);
            if actw < smallw
            {smallw = actw};
        }
        
        for i in 0..p.len()-1 {
            for e in Grr.edges {
                if p[1] == e.v && p[1+i] == e.u {
                    e.w = e.w - smallw;
                }
            }   
        }
        MaxFlow = MaxFlow + smallw;
        p = self.EdmondsKarp(s, t, Grr).unwrap_or_default();
    }

    println!("{}", MaxFlow);
    return MaxFlow;
    }
}
fn main() {
    let path = String::from("./../tests/fluxo_maxmo/wiki.gr");
    let mut graph = Graph::new();
    graph.load(path);
    println!("Arquivo carregado com sucesso!");
    
    graph.FordFulkerson(graph.find_by_pos(2), graph.find_by_pos(5));

    }
