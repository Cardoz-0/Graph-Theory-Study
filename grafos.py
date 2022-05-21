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
print(grafo.vertice_to_index(cicle))

#print(result)
#if result != False:
   # print(1)
  #  print(result)
#else:
 #   print(0)

# grafo.floydWarshall()
