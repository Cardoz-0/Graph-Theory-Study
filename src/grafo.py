import sys
import copy
import networkx as nx
import matplotlib.pyplot as plt
from random import randint

class Graph:
    def __init__(self, directed):
        self.__vertices = []
        self.__weight = []
        self.__directed = directed

    def getEdges(self):
        edges = []
        for vertice in self.__vertices:
            edges += vertice.getEdges()
        return edges

    def qtdVertices(self):
        return len(self.__vertices)

    def qtdEdges(self):
        return len(self.getEdges())

    def isDirected(self):
        return self.__directed

    def degree(self, vertice):
        return len(vertice.getNeighbours)

    def label(self, vertice):
        return vertice.getName()
    
    def getVertices(self):
        return self.__vertices

    def neighbours(self, vertice):
        return vertice.getNeighbours()

    def hasEdge(self, vertice1, vertice2):
        edges = vertice1.getEdges()
        for edge in edges:
            v = edge.vertice2
            if (v == vertice2):
                return (True, edge)

        return (False, None)

    def weight(self, edge):
        edges = vertice1.getEdges()
        for edge in edges:
            v = edge.vertice2
            if (v == vertice2):
                return edge.getWeight()

    def load(self, filename):
        file = open(filename, "r")
        txt = file.readlines()
        file.close()

        self.__qtdVertices = int(txt[0].replace("*vertices ", ""))

        end_of_vertices = txt.index("*edges\n")

        for i in range(1, end_of_vertices): # Vertice loader
            vertice_name = txt[i].replace(str(i)+' ', "").replace('"', "").replace("\n", "")
            self.__vertices.append(Vertice(vertice_name))

        for i in range(end_of_vertices+1, len(txt)): # Edge Loader
            values = txt[i].replace("\n", "").split(" ")
            index1 = int(values[0])-1
            index2 = int(values[1])-1
            weight = str(values[2])
            edge = Edge(self.__vertices[index1], self.__vertices[index2], weight)
            self.__vertices[index1].addEdge(edge, self.isDirected())
            
            # self.__vertices[int(values[0])+1].addEdge(self.__vertices[int(values[1])+1], self.__directed)

    def vertice_to_index(self, vertices):
        returnable = []
        for vertice in vertices:
            index = self.__vertices.index(vertice)
            returnable.append(index)
        return returnable

    def render(self):
        G = nx.Graph()

        for edge in self.getEdges():
            G.add_edge(edge.vertice1.getName(), edge.vertice2.getName(), weight=str(edge.getWeight()))

        options = {
            "font_size": 12,
            "node_size": 5000,
            "node_color": "white",
            "edgecolors": "black",
            "linewidths": 5,
            "width": 5,
        }

        pos = nx.spring_layout(G)
        nx.draw_networkx(G, pos, **options)
        nx.draw_networkx_edge_labels(G,pos)

        # Set margins for the axes so that nodes aren't clipped
        ax = plt.gca()

        ax.margins(0.20)
        plt.axis("off")
        plt.show()

    def getVertice(self, index):
        return self.getVertices()[index]
    
    def vertice_to_index(self, vertice):
        return self.getVertices().index(vertice)

    def vertices_to_index(self, vertices):
        indexes = []
        for vert in vertices:
            indexes.append(self.getVertices().index(vert))
        return indexes

    def queue_to_index_list(self, queue, visited):
        indexes = []
        for item in queue:
            if not (item in visited):
                indexes.append(self.getVertices().index(item))
        return indexes

    # Breadth-First Search
    def searchBFS(self, index):
        visited = [self.getVertice(index)]
        queue = self.getVertices()[index].getNeighbours()
        print("0: " + str(self.queue_to_index_list(queue, [])))
        depth = 1

        while len(queue) != 0:
            vertice = queue.pop(0)
            if not (vertice in visited):
                print(str(depth) + ": " + str(self.queue_to_index_list(vertice.getNeighbours(), visited)))
                depth += 1
                queue.extend(vertice.getNeighbours())
                visited.append(vertice)
        print(str(self.queue_to_index_list(visited, [])))

    # Hierholzer Algorithm
    def detect_eulerian_cicle(self):
        visited = [False for e in self.getEdges()]

        # Vertice selecionado arbitrariamente
        vertice = self.__vertices[0]
        
        (r, Cicle) = self.detect_eulerian_subcicle(vertice, visited)

        if r == False:
            return (False, None)
        else:
            for e in visited:
                if not(e):
                    return(False, None)
            return (True, Cicle)


    def detect_eulerian_subcicle(self, v, C):
        Ciclo = [v]
        t = v

        while True:
            ver_nei = True
            for u in self.neighbours(v):
                (has, edge) = self.hasEdge(u,v)
                index = self.getEdges().index(edge)
                if C[index] == False:
                    ver_nei = False
                    C[index] = True
                    has_op, opposite = self.hasEdge(v,u)
                    if has_op:
                        index_op = self.getEdges().index(opposite)
                        C[index_op] = True
                    v = u
                    Ciclo.append(v)
                    break
                    
            if ver_nei:
                return (False, None)
            if (v == t):
                break

        for x in Ciclo:
            for w in self.neighbours(x):
                (has, edge) = self.hasEdge(x,w)
                index = self.getEdges().index(edge)

                if C[index] == False:
                    (r,Aux_ciclo) = self.detect_eulerian_subcicle(x, C)
                    if r == False:
                        return (False, None)
                    else:
                        aux = Ciclo.index(x)+1
                        Ciclo = Ciclo[:Ciclo.index(x)] + Aux_ciclo + Ciclo[aux:]
        return (True, Ciclo)

    def dijikstra(self, s):
        # Nodos não visitados
        C = list(self.getVertices())
 
        # Salva o custo para visitar um nodo
        D = {}
    
        # Salva o caminho mais rápido para um nodo encontrado até o momento
        A = {}
    
        # "Infinito"
        max_value = sys.maxsize
        for node in C:
            D[node] = max_value

        # Setamos o custo inicial como 0
        D[s] = 0
        
        # Visita todos os nodos
        while C:
            # Encontra o nodo com o menor valor
            current_min_node = None
            for node in C: 
                if current_min_node == None:
                    current_min_node = node
                elif D[node] < D[current_min_node]:
                    current_min_node = node
                    
            # Atualiza o custo de cada aresta de acordo com os vizinhos
            neighbors = self.neighbours(current_min_node)
            for neighbor in neighbors:
                (has, edge) = self.hasEdge(current_min_node,neighbor)
                tentative_value = D[current_min_node] + edge.getWeight()
                if tentative_value < D[neighbor]:
                    D[neighbor] = tentative_value
                    # Atualiza o melhor caminho para o nodo atual
                    A[neighbor] = current_min_node
    
            # Marca o nodo como visitado
            C.remove(current_min_node)
        
        return A, D

    def print_result(self, previous_nodes, shortest_path, start_node, target_node):
        path = []
        node = target_node
        
        while node != start_node:
            path.append(node.getName())
            node = previous_nodes[node]
    
        path.append(start_node.getName())
        

        print(" -> ".join(reversed(path))+", d={}".format(shortest_path[target_node]))

    def floydWarshall(self):
        nVert = self.qtdVertices()
        distances= [[99999 for i in range(nVert)] for j in range(nVert)]

        for vertice in range(nVert):
            distances[vertice][vertice] = 0

        for i in range(len(self.getEdges())):
                print(self.getEdges()[i][1])

#            distances[edge.vertice1 -1][edge.vertice2-1] = edge[2]
#            distances[edge.vertice2-1][edge.vertice1-1] = edge[2]

        for i in range(len(distances)):
            for row in range(len(distances)):
                for collumn in range(len(distances[row])):
                    distances[row][collumn] = min(
                                            distances[row][collumn],
                                            distances[row][i] +
                                            distances[i][collumn]
                    )

        Tlines = 1
        for i in distances:
            print(str(Tlines) + ":", str(i)[1:+1])
            Tlines += 1


class Vertice:
    def __init__(self, name) -> None:
        self.__name = name
        self.__edges = []

    def getName(self):
        return self.__name

    def getEdges(self):
        return self.__edges

    def __str__(self):
        return self.getName()

    def getNeighbours(self):
        neighbours = []
        for edge in self.__edges:
            neighbours.append(edge.vertice2)
        return neighbours

    def addEdge(self, edge, directed):
        self.__edges.append(edge)

        if not directed:
            edge.vertice2.addEdge(Edge(edge.vertice2, edge.vertice1, edge.getWeight()), True) # Adiciona sem se readicionar

    def removeEdge(self, edge):
        if edge in self.__edges: self.__edges.remove(edge) # Condição para que essa função possa ser usada para grafos ordenados ou não ordenados


class Edge:
    def __init__(self, vertice1, vertice2, weight=1.0) -> None:
        self.vertice1 = vertice1
        self.vertice2 = vertice2
        self.__weight = weight

    def getWeight(self):
        return float(self.__weight)

    def destroyEdge(self): #Pode ser usado se for ordenado ou não
        self.__vertice1.removeEdge(self.__vertice2)
        self.__vertice2.removeEdge(self.__vertice1)
