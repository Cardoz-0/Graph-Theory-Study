from src.grafo import Graph


grafo = Graph()
grafo.load("./tests/caminho_minimo/fln_pequena.net")
#grafo.render()

grafo.searchBFS(0)

#Eulerian Circle
#result = grafo.EulerianCircle()

#print(result)
#if result != False:
   # print(1)
  #  print(result)
#else:
 #   print(0)

# grafo.floydWarshall()
