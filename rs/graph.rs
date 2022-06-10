fn main() {
    struct Graph {
        vertices : Vec<Edge>,
        directed : bool, 
    }

    struct Edge {
        vertice1: Vertice,
        vertice2: Vertice,
        weight: i32,
    }

    struct Vertice {
        name: std::String,
        edges: Vec<Edge>,
    }

    
    impl Graph {
        pub fn new() -> Graph {
            Graph {vertices: Vec::new(), edge: Vec::new(), weight: 0 }
        }

        pub fn get_edges(&mut self) -> usize {
            let edges = Vec::new();
            for i in self.vertices.iter_mut() {
                edges += i;
            }
            edges
        }
        pub fn qtdVertices(self) -> usize {
            self.vertices.len()
        }
        pub fn qtdEdges(self) -> usize {
            self.get_edges().len()
        }
        
        pub fn isDirected(self) -> bool {
            self.directed
        }
        
        pub fn degree(self, &vertice) -> //not sure yet {
            vertice.getNeighbours()
        }

        pub fn label(self, vertice) -> std::String {
            vertice.getName()
        }
        
        pub fn getVertices(self) -> Vec<Edge> {
            self.vertices
        }

        pub fn neighbours(self, vertice) -> //not sure yet {
            vertice.getNeighbours()                                     
        }
        
        pub fn hasEdge(self, vertice1, vertice2) -> bool, edge {
            let edges = vertice1.getEdges();
            for edge in edges.iter_mut() {
                let v = edge.vertice2
                if (v == vertice2):
                    return (True, edge)
            }
            
            return (False, None)
        }

        pub fn weight(self, edge) -> i32 {

        let edges = vertice1.getEdges();
        for edge in edges.iter_mut() {
            let v = edge.vertice2
            if (v == vertice2) {return edge.getWeight()}
        }
        
        pub fn getVertice(self, index) {
            self.getVertices()[index]
        }
        
        pub fn vertice_to_index(self, vertice) -> Vec<vertice> {
            let indexes = Vec::new();
            for i in vertices {
                indexes.push(self.getVertices().index(vert))
            }
        indexes
        }
    }
    impl Edge {
        pub fn new() -> Edge {
            Edge {vertice1: Vertice::new(), vertice2: Vertice::new()}
        }
    }
    
    impl Vertice {
        pub fn new() -> Vertice {
            Vertice { weight: 0, edges: Vec::new() }
        }
    }
    let mut graph = Graph::new();
    let edge = Edge::new();
    graph.set_edge(edge);
    let len: usize = graph.get_vertices();
    println!("{}", len);
}
