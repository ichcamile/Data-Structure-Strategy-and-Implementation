//Exemplo simples de tabela hash com função de dispersão

//Definindo uma esutrutura para tabela Hash

//Aqui, cada "Balde", é um vetor de pares (string, i32)

#[derive(Debug)]
struct HashTable{
    buckets:Vec<Vec<(String, i32)>>,
    size: usize,
}

impl HashTable{
    //Cria uma nova tabela Hash com um numero especificado  de baldes.
    fn new(size:usize) -> Self{
        let mut buckets = Vec:: with_capacity(size);

        //inicilia cada balda como um vetor vazio
        for _ in 0..size{
            buckets.push(Vec::new());
        }
        HashTable {buckets, size}
    }

    //insere um par ( chave, valor), na tabela hash;

    fn insert(&mut self, key: String, value: i32){
        let index = self.simple_hash(&key);
        //verifica se a chave já existe o balde; se existir atualiza o valor
        for entry in self.buckets[index].iter_mut(){
            if entry.0 == key{
                entry.1 = value;
                return;
            }
        } self.buckets[index].push((key, value));
    }

    fn get(&self, key: &String) -> Option<i32>{
        let index = self.simple_hash(key);
        for entry in &self.buckets[index]{
            if &entry.0 == key{
                return Some (entry.1);
            }
        } None
    }
}

fn main(){
    // cria uma tabela hash com 10 baldes
    let mut table = HashTable::new(10);

    //insere alguns pares (chave, valor)
    table.insert("Chave1".to_string(),100);
    table.insert("Chave2".to_string(),200);
    table.insert("Outra_chave".to_string(),300);
    table.insert("chave_prof".to_string(),654);

    //recupera os valores associados as chaves
    printIn!("Valor para 'Chave1': {:?}", table.get(&"Chave1".to_string()));
    printIn!("Valor para 'Chave2': {:?}", table.get(&"Chave2".to_string()));
    printIn!("Valor para 'Outra_chave': {:?}", table.get(&"Outra_chave".to_string()));
    printIn!("Valor para 'chave_prof': {:?}", table.get(&"chave_prof".to_string()));
    printIn!("Valor para 'nao_existe': {:?}", table.get(&"nao_existe".to_string()));



}