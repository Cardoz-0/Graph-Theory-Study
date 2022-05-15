from src.grafo import Graph


grafo = Graph()
grafo.load("./tests/ciclo_euleriano/SemCicloEuleriano.net")
#grafo.render()

grafo.searchBFS(0)

#Eulerian Circle
result = grafo.EulerianCircle()

print(result)
if result != False:
    print(1)
    print(result)
else:
    print(0)
