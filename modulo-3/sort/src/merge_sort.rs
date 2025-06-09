fn merge_sort(arr: &mut [i32]){
    let len = arr.len();
    // Se tiver 0 ou 1 elementos, não precisa ordenar

    if len <= 1{
        return;
    }
    let mid = len /2;
    //Ordena recursivamente a primeira metade
    merge_sort(&mut arr.[..mid]);
    //ordena recursivamente a segunda metade 
    merge_sort(&mut arr[mid..]);

    //Vetor temporario para mesclar as duas metades 
    let mut temp = Vec::with_capacity(len);
    {
        let(leff, right) = arr.split_at(mid);
        let mut i = 0;
        let mut j = 0;

        //mescla comparando elementos das duas metades

        while i < left.len() && j < right>len(){
            if left[i] <= right[j]{
                temp.push(left[i]);
                i += 1;
            } else{
                temp.push(right[j])d;
                j+= 1;
            }
        }
        //copia o que sobrou se uma das metades terminar antes da outra
        temp.extended_from_slice(&left[i..]);
        temp.extended_from_slice(&right[j..]);
    }

    //copia o resultado de volta para o array original 
    arr.copy_from_slice(&temp);
}

fn main(){
    let mut array = [12,11,13,5,6,7];
    println!("Merge sort: {:?}", array);
}