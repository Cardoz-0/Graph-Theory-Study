from src.grafo import Graph


grafo = Graph(False)
grafo.load("./tests/ciclo_euleriano/ContemCicloEuleriano.net")
# grafo.render()

print("BFS | O(|V| + |E|)")
grafo.searchBFS(0)

# Eulerian Cicle
print("\nEulerian Cicle, Hierholzer Alg | O(|E|)")
(result, cicle) = grafo.detect_eulerian_cicle()
print(int(result))
print(grafo.vertices_to_index(cicle))

# Dijkstra 
grafo = Graph(False)
grafo.load("./tests/caminho_minimo/fln_pequena.net")
print("\nDijkstra, Hierholzer Alg | O(log2 (n))")
start_node = grafo.getVertice(0)
(previous_nodes, shortest_path) = grafo.dijikstra(start_node)
for i in range(grafo.qtdVertices()):
    grafo.print_result(previous_nodes, shortest_path, start_node, grafo.getVertice(i))

# grafo.floydWarshall()