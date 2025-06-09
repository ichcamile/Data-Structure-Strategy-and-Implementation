#[derive(Debug)]
struct HashTable {
    buckets: Vec<Vec<(String, i32)>>,
    size: usize,
}

impl HashTable {
    // Cria uma nova tabela hash com um número especificado de baldes
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);

        // Inicializa cada balde como um vetor vazio
        for _ in 0..size {
            buckets.push(Vec::new());
        }

        HashTable { buckets, size }
    }

    // Função de dispersão simples:
    // Soma os valores dos bytes da chave e calcula o módulo pelo tamanho da tabela
    fn simple_hash(&self, key: &String) -> usize {
        let sum: usize = key.bytes().map(|b| b as usize).sum();
        sum % self.size
    }

    // Insere um par (chave, valor) na tabela hash
    fn insert(&mut self, key: String, value: i32) {
        let index = self.simple_hash(&key);
        // Verifica se a chave já existe no balde; se existir, atualiza o valor
        for entry in self.buckets[index].iter_mut() {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }

    // Recupera o valor associado à chave, se existir
    fn get(&self, key: &String) -> Option<i32> {
        let index = self.simple_hash(key);
        for entry in &self.buckets[index] {
            if &entry.0 == key {
                return Some(entry.1);
            }
        }
        None
    }
}

fn main() {
    let mut table = HashTable::new(10);

    table.insert("Chave1".to_string(), 100);
    table.insert("Chave2".to_string(), 200);
    table.insert("Outra_chave".to_string(), 300);
    table.insert("chave_prof".to_string(), 654);

    println!("Valor para 'Chave1': {:?}", table.get(&"Chave1".to_string()));
    println!("Valor para 'Chave2': {:?}", table.get(&"Chave2".to_string()));
    println!("Valor para 'Outra_chave': {:?}", table.get(&"Outra_chave".to_string()));
    println!("Valor para 'chave_prof': {:?}", table.get(&"chave_prof".to_string()));
    println!("Valor para 'nao_existe': {:?}", table.get(&"nao_existe".to_string()));
}
