use std::cmp;

// Definição de um nó da árvore
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32, // altura do nó (usado apenas na AVL)
}

impl Node {
    // Cria um novo nó
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    // Calcula a altura de um nó
    fn height(node: &Option<Box<Node>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    // Calcula o fator de balanceamento de um nó
    fn balance_factor(&self) -> i32 {
        Node::height(&self.left) - Node::height(&self.right)
    }

    // Atualiza a altura de um nó
    fn update_height(&mut self) {
        self.height = cmp::max(Node::height(&self.left), Node::height(&self.right)) + 1;
    }

    // Rotação à direita
    fn rotate_right(mut self) -> Box<Node> {
        let mut left_child = self.left.take().unwrap();
        self.left = left_child.right.take();
        self.update_height();
        left_child.right = Some(Box::new(self));
        left_child.update_height();
        Box::new(left_child)
    }

    // Rotação à esquerda
    fn rotate_left(mut self) -> Box<Node> {
        let mut right_child = self.right.take().unwrap();
        self.right = right_child.left.take();
        self.update_height();
        right_child.left = Some(Box::new(self));
        right_child.update_height();
        Box::new(right_child)
    }

    // Balanceia a árvore AVL
    fn balance(mut self) -> Box<Node> {
        self.update_height();
        let balance = self.balance_factor();

        if balance > 1 {
            if let Some(ref mut left) = self.left {
                if left.balance_factor() < 0 {
                    self.left = Some(left.rotate_left());
                }
            }
            return self.rotate_right();
        }

        if balance < -1 {
            if let Some(ref mut right) = self.right {
                if right.balance_factor() > 0 {
                    self.right = Some(right.rotate_right());
                }
            }
            return self.rotate_left();
        }

        Box::new(self)
    }

    // Insere um valor na árvore
    fn insert(mut self, value: i32) -> Box<Node> {
        if value < self.value {
            self.left = match self.left {
                Some(left) => Some(left.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        } else if value > self.value {
            self.right = match self.right {
                Some(right) => Some(right.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        }
        self.balance()
    }

    // Busca um valor na árvore
    fn search(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            self.left.as_ref().map_or(false, |left| left.search(value))
        } else {
            self.right.as_ref().map_or(false, |right| right.search(value))
        }
    }
}

// Definição da árvore
#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = match self.root.take() {
            Some(root) => Some(root.insert(value)),
            None => Some(Box::new(Node::new(value))),
        };
    }

    fn search(&self, value: i32) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}

fn main() {
    let mut tree = Tree::new();

    // Inserção de valores
    tree.insert(10);
    tree.insert(20);
    tree.insert(30);
    tree.insert(40);
    tree.insert(50);
    tree.insert(25);

    // Busca de valores
    println!("Busca por 20: {}", tree.search(20)); // true
    println!("Busca por 15: {}", tree.search(15)); // false
}
