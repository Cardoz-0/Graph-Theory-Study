import sys
import networkx as nx
import matplotlib.pyplot as plt

class Graph:
    def __init__(self, directed=False):
        self.__vertices = []
        self.__edges = []
        self.__weight = []
        self.__directed = directed


    def qtdVertices(self):
        return len(self.__vertices)

    def qtdEdges(self):
        return len(self.__edges)

    def isDirected(self):
        return self.__directed

    def degree(self, vertice):
        return len(ertice.getNeighbours)

    def label(self, vertice):
        pass

    def neighbours(self, vertice):
        return vertice.getNeighbours()

    def hasEdge(self, vertice1, vertice2):
        edges = vertice1.getEdges()
        for edge in edges:
            v = edge.vertice2
            if (v == vertice2):
                return true

        return false

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
            self.__vertices[index1].addEdge(edge, self.isDirected)
            self.__edges.append(edge)
            # self.__vertices[int(values[0])+1].addEdge(self.__vertices[int(values[1])+1], self.__directed)
            
    def render(self):
        G = nx.Graph()

        for edge in self.__edges:
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
        return self.__vertices[index]

    # Breadth-First Search
    def searchBFS(self, index):
        visited = [self.getVertice(index)]
        queue = self.__vertices[index].getNeighbours()
        print("Buscando elemento " + self.__vertices[index].getName())
        depth = 0

        while len(queue) != 0:
            vertice = queue.pop(0)
            if not (vertice in visited):
                print(str(depth) + "-Buscando elemento " + vertice.getName())
                depth += 1
                queue.extend(vertice.getNeighbours())
                visited.append(vertice)

    # Hierholzer Algorithm
    def detectEulerianCircle(self):



class Vertice:
    def __init__(self, name) -> None:
        self.__name = name
        self.__edges = []

    def getName(self):
        return self.__name

    def getEdges(self):
        return self.__edges

    def getNeighbours(self):
        neighbours = []
        for edge in self.__edges:
            neighbours.append(edge.vertice2)
        return neighbours 

    def addEdge(self, edge, directed):
        self.__edges.append(edge)
        if not directed:
            edge.vertice2.addEdge(self, Edge(edge.vertice2, edge.vertice1), True) # Adiciona sem se readicionar
        
    def removeEdge(self, edge):
        if edge in self.__edges: self.__edges.remove(edge) # Condição para que essa função possa ser usada para grafos ordenados ou não ordenados


class Edge:
    def __init__(self, vertice1, vertice2, weight=None) -> None:
        self.vertice1 = vertice1
        self.vertice2 = vertice2
        self.__weight = weight

    def getWeight(self):
        return self.__weight
    
    def destroyEdge(self): #Pode ser usado se for ordenado ou não
        self.__vertice1.removeEdge(self.__vertice2)
        self.__vertice2.removeEdge(self.__vertice1)
