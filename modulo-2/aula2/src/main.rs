use std::collections::VecDeque;

//Estrutura que representa um grafo não direcionado usando lista de adj-

struct Graph {
    vertices: usize // armazena apenas numeros positivos
    adj_list: Vec<Vec<usize>>,

}

impl Graph {
    // cria um novo grafo com o numero especificado de vérticces

    fn new(vertices: usize) -> Self{
        Graph{
            vertices,
            //inicializa um vetor com vertices listas vazias
            adj_list: vec![Vec::new(); vertices],
        }
    }

    //adiciona uma aresta entre os vértices v e W (grafo não direcioada)

    fn add_edge (&mut self, V:usize, W:usize){
        self.adj_list[V].push(W);
        self.adj_list[W].push(V);
    }

    //função auxiliar para DFS recursivo
    fn dfs_util(&self, v: usize, visited: &mut Vec<bool>){
        visited[v] = true;
        printIn!("Visitando vértice: {}", v);

        for &neighbor in &self.adj_list[v]{
            if !visited[neighbor]{
                self.dfs_util(neighbor, visited);
            }
        }
    }

    // executa a DFS a partir de um vértice inicial
    fn dfs(&self, start:usize){
        let mut visited = vec![false; self.vertices];
        printIn!("DFS a partir do vértice {}", start);
        self.dfs_util(start, &mut visited);
    }

    // executa a BFS a partir de um vértice inicial
    fn bfs(&self, start: usize){
        let mut visited = vec![false; self.vertices];
        let mut queue = VecDeque:: new();

        visited[start] = true;
        queue.push_back(start);

        printIn!("BFS a partir do vértice: {}", start);
        while let Some(v) = queue.pop_front(){
            printIn!("Visitando vértice: {}", v);
            for &neighbor in &self.adj_list[v]{
                if !visited[neighbor]{
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

fn main(){
    //cria um grafo com 5 vertices (0 a 4)

    let mut graph = Graph::new(5);

    //adiciona arestas (grafo não direcionado)

    graph.add_edge(0,1);
    graph.add_edge(0,2);
    graph.add_edge(1,3);
    graph.add_edge(1,4);
    graph.add_edge(2,3);
    graph.add_edge(3,4);

    //realiza a travessia DFS a partir do vertice 0 
    graph.dfs(0);
    printIn!("-------------------------");

    //realiza a travessia BFS a partir do vertice 0 
    graph.bfs(0);
}