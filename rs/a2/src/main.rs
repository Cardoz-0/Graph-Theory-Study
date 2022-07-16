use std::cmp::min;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Deref;
use std::rc::Rc;
use std::collections::VecDeque;
use std::io;
use std::io::Write;

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

    pub fn detect_scc(&self) -> () {
        let mut stack: Vec<AuxNodeSCC> = Vec::new();

        for v in &self.verts {
            stack.push(AuxNodeSCC {
                node: VisitableNode{v: Rc::clone(&v), visited: false},
                on_stack: false,
                lowest: 1, // Arquivos de testes utilizam 1 para indexar o primeiro elemento
            })
        }
        for i in 0..stack.len() {
            if !stack[i].node.visited {
                self.dfs_scc(i, &mut stack);
            }
        }
        for i in 0..stack.len() {
            println!("{}", stack[i].lowest);
        }
    }

    fn dfs_scc(&self, at: usize, stack: &mut Vec<AuxNodeSCC>) {
        let mut current = &mut stack[at];
        if current.node.visited == false {
            current.node.visited = true;
            current.on_stack = true;
            current.lowest = current.node.v.id;

            for neighbour in self.get_neighbours(Rc::clone(&stack[at].node.v)) {
                for to in 0..stack.len() {
                    if neighbour.id == stack.get(to).unwrap().deref().node.v.id {
                        if !stack[to].node.visited {
                            self.dfs_scc(to, stack);
                        }
                        if stack[to].on_stack {
                            stack[at].lowest = min(stack[at].lowest, stack[to].lowest);
                        }
                        break;
                    }
                }
            }

            if stack[at].lowest == stack[at].node.v.id {
                for i in 0..stack.len() {
                    if stack[i].on_stack {
                        stack[i].on_stack = false;
                        stack[i].lowest = stack[at].node.v.id;
                        if i == at {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn dfs_toporder(&self, at: usize, nodes: &mut Vec<OrderedNode>, mut last_available: usize) -> usize {
        nodes[at].visitable.visited = true;
        for neighbour in self.get_neighbours(Rc::clone(&nodes[at].visitable.v)) {
            let orderable_neighbour = nodes.iter().find(|x| x.visitable.v.id == neighbour.id).unwrap();
            if !orderable_neighbour.visitable.visited {
                last_available = self.dfs_toporder(orderable_neighbour.visitable.v.id, nodes, last_available); 
            }   
        }
        nodes[at].pos = last_available;
        return last_available-1;
    }

    pub fn topological_order(&self) -> Vec<OrderedNode> {
        let mut stack: Vec<OrderedNode> = Vec::new();

        for v in &self.verts {
            stack.push(OrderedNode {
                visitable: VisitableNode {v: Rc::clone(&v),visited: false},
                pos: 0
                }
            )
        }
        let last_available = stack.len();
        
        for at in 0..stack.len() {
            if !stack[at].visitable.visited {
                    self.dfs_toporder(at, &mut stack, last_available);
            }
        }
        stack
    }

    pub fn prims(&self, s: usize) -> (f32, Vec<Rc<Edge>>) {
        let mut stack: Vec<VisitableNode> = Vec::new();

        for v in &self.verts {
            stack.push(VisitableNode {v: Rc::clone(&v),visited: false});
        }

        let mut mst_edges: Vec<Rc<Edge>> = Vec::new();
        let mut pq: VecDeque<Rc<Edge>> = VecDeque::new();
        stack[0].visited = true;
        self.get_edges(self.find_by_id(0)).iter().for_each(
            |e| if !stack[e.u.id].visited {pq.push_back(Rc::clone(e))}); 

        let total_edges = &self.verts.len() - 1;
        let mut mst_cost = 0.0;
        while !pq.is_empty() && mst_edges.len() != total_edges {
            let edge = pq.pop_front().unwrap();
            if !stack[edge.u.id].visited {
                mst_edges.push(Rc::clone(&edge));
                mst_cost += edge.w;

                stack[edge.u.id].visited = true;
                self.get_edges(self.find_by_id(0)).iter().for_each(
                    |e| if !stack[e.u.id].visited {pq.push_back(Rc::clone(e))});

            }
        }
        (mst_cost, mst_edges)
    }    
}

fn main() {
    let path = String::from("./../../tests/dirigidos/manha.net");
    let mut graph = Graph::new();
    graph.load(path);
    println!("Arquivo carregado com sucesso!");
    let some_v = graph.find_by_id(0);
    let neighbours = graph.get_neighbours(some_v.clone());
    for v in neighbours {
        println!("Vertex: {}, Neighbors{}", some_v.id, v.id)
    }
    graph.detect_scc();
    
    let _toporder = graph.topological_order();
    for t in &_toporder {
        if t.pos == _toporder.len() - 1 { 
            println!("{} ", t.visitable.v.name);
            }
        else {
            print!("{} -> ", t.visitable.v.name);
            io::stdout().flush().unwrap(); 
    }
    }

    let path = String::from("./../../tests/arvore_geradora_minima/agm_tiny.net");
    let mut tree = Graph::new();
    tree.load(path);
    println!("Arquivo carregado com sucesso!");
 
    let (cost, mst) = tree.prims(0);
    println!("{}", cost);
    for i in &mst {
        print!("{} - {}, ", i.u.id, i.v.id);
        io::stdout().flush().unwrap(); 
    }

}
