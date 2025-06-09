// Estrutura de dados: Pilha (Stack) genérica
pub struct Stack<T> {
    elements: Vec<T>, // Usamos um Vec interno para armazenar os elementos da pilha
}

impl<T> Stack<T> {
    // Cria uma nova pilha vazia
    pub fn new() -> Stack<T> {
        Stack {
            elements: Vec::new(),
        }
    }

    // Adiciona um elemento ao topo da pilha
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    // Remove o elemento do topo da pilha e o retorna
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Retorna uma referência ao elemento do topo sem removê-lo
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // Verifica se a pilha está vazia
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(10);
        stack.push(20);

        // Verifica o topo da pilha
        assert_eq!(stack.peek(), Some(&20));

        // Remove elementos do topo
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));

        // A pilha deve estar vazia novamente
        assert!(stack.is_empty());
    }
}
