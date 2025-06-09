// Função que realiza a busca por interpolação
fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    // Verifica se o array está vazio
    if arr.is_empty() {
        return None;
    }

    // Define os índices inicial (low) e final (high) da busca
    let mut low = 0;
    let mut high = arr.len() - 1;

    // Enquanto os limites forem válidos e o target estiver dentro do intervalo
    while low <= high && target >= arr[low] && target <= arr[high] {
        // Evita divisão por zero se todos os elementos entre low e high forem iguais
        if arr[high] == arr[low] {
            return if arr[low] == target {
                Some(low)
            } else {
                None
            };
        }

        // Calcula a posição provável do target usando interpolação
        let pos = low + (((target - arr[low]) as usize * (high - low))
            / ((arr[high] - arr[low]) as usize));

        // Verifica se a posição está dentro dos limites do array
        if pos >= arr.len() {
            break;
        }

        // Verifica se encontrou o valor na posição estimada
        if arr[pos] == target {
            return Some(pos); // Valor encontrado
        } else if arr[pos] < target {
            // Atualiza o limite inferior se o valor estiver mais à direita
            low = pos + 1;
        } else {
            // Atualiza o limite superior se o valor estiver mais à esquerda
            // Garante que não haja underflow se pos for 0
            if pos == 0 {
                break;
            }
            high = pos - 1;
        }
    }

    // Retorna None se o valor não for encontrado
    None
}

// Função principal para testar a busca
fn main() {
    // Define um array ordenado (pré-requisito da busca por interpolação)
    let arr = [10, 20, 30, 40, 50, 60, 70, 80];
    let target = 50;

    // Executa a busca e imprime o resultado
    match interpolation_search(&arr, target) {
        Some(index) => println!("Valor encontrado na posição {}", index),
        None => println!("Valor não encontrado"),
    }
}
