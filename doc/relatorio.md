## ATIVIDADE 1 Atividade Pr ́atica A1 – Grafos (INE5413)
#Ciências da Computacao – Universidade Federal de Santa Catarina
#Prof. Rafael de Santiago

Estudantes:
Gabriel da Silva Cardoso (20100524)
Hans Buss Heidemann (19100528)

Estudantes:

**Algoritmo 1**
        No algoritmo um usamos as três classes que criamos para essa atividade, as classes Grafos, Vertices e Edges. A classe grafos possui duas listas com os vertices e o edges que estão nesse grafo. A classe  Vertices possui  uma lista para os Edges presentes nessa instancia. E a classe Edge é composta por dois vertices 1 e 2, e o peso que esse edge possui. 
        Além das funções pedidas implementamos também a função vertice_to_index() para garantir uma melhor visibilidade.

**Algoritmo 2**
        No algoritmo do ciclo euleriano precisamos estar utilizando um grafo para gerarmos uma lista de vertices vázios e selecionar o primeiro elemento dessa lista. 
        Após isso precisamos passar para a função implementada detect_eulerian_subcircle() que recebe o grafo, o primeiro vertice selecionado e uma lista com os edges. 
                Neste algoritmo não precisamos estar implementando nenhuma função além das solicitadas.

**Algoritmo 3** ##algoritmo do djalma


**Algoritmo 4** 
        O algoritmo de FloydWarshall é um algoritmo relativamente simples, não há necessidade de utilizar nenhuma função além das solicitadas. Passamos um grafo como argumento e criamos um matriz de valores utilizando dicionarios bscando diminuir a complexidade.
        Após a criação da matriz o algoritmo então executada sobre a mesma e printa a cada linha a distância na ordem crescente para cada par de vertices.
