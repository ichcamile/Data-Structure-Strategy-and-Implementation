fn main() {
    println!("Hello, world!");
}

pub struct MyVec{
    data: Vec<i32>,
}

impl MyVec{
    pub fn new() -> MyVec{
        MyVec {data: Vec::new()}
    }

    pub fn push (&mut self, value: i32){
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&i32>{
        self.data.get(index)
    }
}




// src /lib.rs
//modulo de testes: esse codigo será compilado e excutado apenas quando rodarmos os testes 
#[cfg(test)]
mod tests {
    //traz as definicações do escopo superior (como a estrurara MyVec) para o módulo de testes
    use super ::*;

    #[test]
    fn test_push_and_get(){
        //let mut vec = MyVec :: new()
        //
        // let é usado para declarar uma variavel 
        //mut indica ue a variavel é mutavel
        //MyVec :: new(); chama a função associada new da estrutura MyVec para criar uma nota instância do vetor
        let mut vec = MyVec:: new();

        // adiciona dois elementos
        vec.push(10);
        vec.push(20);

        //verifica se os elementos foram inseridos corretamente.
        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&20));

        //Um índice que não existe deve retornar None.

        assert_eq!(vec.get(2), None);


    }
}