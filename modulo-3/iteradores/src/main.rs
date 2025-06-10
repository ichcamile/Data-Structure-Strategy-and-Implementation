fn main() {

    let numbers = vec![1,2,3,4,5,6,7,8,9,10];
    //Usando iteradors para calcular a soma dos quadrados dos numeros pares:
    let sum_even_squares: i32 = numbers
    .iter()  //Cria um iterador sobre os elementos
    .filter(|&&x| x% 2 == 0) //FIltra somente os numeros pares
    .map(|&x| x* x) //Eleva cada numero ao quadrado
    .sum(); //soma todos os resultados

    println!("Soma dos quadrado dos numeros pares: {}", sum_even_squares)

    //Usando iteradores para encontrar o primeiro numeros maior que 5
    let first_gt_five = numbers.iter().find(|&&x| x > 5);

    match first_gt_five{
        Some(x) => println("Primeiro número maior que 5: {}", x),
        None => println("Nenhum numero maior que 5 encontrado."),
    }


}
