// Função de busca binária em um array ordenado
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len(); // usamos exclusive upper bound

    // Enquanto low for menor que high
    while low < high {
        // Calcula o índice do meio
        let mid = low + (high - low) / 2;

        // Verifica se o valor no meio é o target
        if arr[mid] == target {
            return Some(mid); // Encontrado
        } else if arr[mid] < target {
            low = mid + 1; // Busca na metade superior
        } else {
            high = mid; // Busca na metade inferior
        }
    }

    // Valor não encontrado
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target = 7;

    match binary_search(&arr, target) {
        Some(index) => println!("Elemento {} encontrado no índice {}.", target, index),
        None => println!("Elemento {} não encontrado.", target),
    }
}
