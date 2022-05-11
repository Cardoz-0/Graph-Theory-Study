from src.grafo import Graph


grafo = Graph()
grafo.load("./tests/ciclo_euleriano/ContemCicloEuleriano.net")
# grafo.render()

grafo.searchBFS(0)
