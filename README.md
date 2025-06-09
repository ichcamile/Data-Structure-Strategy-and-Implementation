# Data-Structure-Strategy-and-Implementation
# 📚 Estrutura de Dados

Repositório dedicado aos principais conceitos, estruturas e algoritmos fundamentais estudados na disciplina **Estrutura de Dados**.

---

## 🔍 O que são Estruturas de Dados?

Estruturas de dados são formas de **organizar, armazenar e manipular informações** na memória do computador, de forma eficiente e estruturada. Elas servem como a base para a construção de algoritmos rápidos e eficazes, sendo fundamentais em praticamente todos os sistemas computacionais.

---

## 🧱 Principais Estruturas de Dados

### ➤ Estruturas Lineares

| Estrutura | Descrição | Características |
|----------|------------|-----------------|
| **Array** (vetor) | Coleção de elementos de mesmo tipo com tamanho fixo | Acesso rápido por índice, tamanho imutável |
| **Lista Ligada** | Elementos encadeados por ponteiros | Tamanho dinâmico, inserções/remoções eficientes |
| **Pilha (Stack)** | Último a entrar é o primeiro a sair (LIFO) | Usada para desfazer ações, percorrer estruturas |
| **Fila (Queue)** | Primeiro a entrar é o primeiro a sair (FIFO) | Ideal para processamento em ordem |

---

### ➤ Estruturas Não Lineares

| Estrutura | Descrição | Aplicações |
|-----------|-----------|------------|
| **Árvore** | Estrutura hierárquica com nós filhos e raiz | Representação de diretórios, árvores de decisão |
| **Árvore Binária de Busca (BST)** | Árvore onde à esquerda estão valores menores, à direita, maiores | Busca, inserção e remoção eficientes |
| **Árvore AVL / Red-Black** | Árvores balanceadas para manter desempenho | Garante tempo logarítmico em operações |
| **Heap** | Árvore quase completa usada para encontrar máximo/mínimo rapidamente | Filas de prioridade |
| **Grafo** | Conjunto de vértices e arestas | Redes sociais, rotas de GPS, análise de redes |

---

### ➤ Estruturas de Acesso Rápido

| Estrutura | Descrição | Benefícios |
|-----------|-----------|------------|
| **Tabela Hash** | Usa uma função hash para mapear chaves a valores | Acesso e inserção em tempo constante (idealmente) |
| **Conjuntos (Set)** | Armazena elementos únicos | Rápido para verificar existência e evitar duplicatas |

---

## ⚙️ Categorias de Algoritmos

- **Busca**: localizar elementos (busca linear, binária)
- **Ordenação**: reorganizar dados (bubblesort, mergesort, quicksort)
- **Manipulação de estruturas**: inserção, remoção, balanceamento
- **Algoritmos em grafos**: BFS, DFS, Dijkstra, Kruskal, etc.
- **Recursão e divisão e conquista**: abordagem eficiente para resolver problemas grandes

---

## 📈 Complexidade de Algoritmos

Usamos a notação **Big-O** para descrever o desempenho dos algoritmos:

| Notação | Nome | Desempenho |
|---------|------|------------|
| O(1) | Constante | Independente do tamanho |
| O(log n) | Logarítmica | Muito rápida em grandes volumes |
| O(n) | Linear | Cresce proporcionalmente aos dados |
| O(n log n) | Quase-linear | Ótima para ordenações |
| O(n²) | Quadrática | Lenta para grandes conjuntos |
| O(2ⁿ), O(n!) | Exponencial/Fatorial | Inviável com grandes entradas |

---

## 🛠️ Aplicações no Mundo Real

- **Redes sociais**: grafos para conexões entre usuários
- **Buscas e rankings**: árvores, heaps e tabelas hash
- **Sistemas operacionais**: filas e pilhas para agendamento
- **Jogos**: árvores de decisão e grafos para IA
- **E-commerce**: arrays, árvores e listas para produtos, categorias e estoques

---

## 📌 Organização do Repositório

```bash
📁 estrutura-de-dados/
├── arrays/
├── listas-ligadas/
├── pilhas/
├── filas/
├── arvores/
├── grafos/
├── tabelas-hash/
├── algoritmos/
└── README.md  ← este arquivo

