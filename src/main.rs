fn is_sum_even(sequence: &mut Vec<u64>) -> bool {
   let seq_sum: u64 = sequence.iter().sum();
   seq_sum % 2 == 0
}

fn is_graphic(mut sequence: Vec<u64>) -> bool {
    // Se a soma não for par, então a sequência não é gŕafica
    if !is_sum_even(&mut sequence) {
        return false;    
    }
    
    let n = sequence.len() as u64;

    // Soma de d_i de i = 1 até k
    let mut sum_di = 0_u64;

    for (k, &di) in sequence.iter().enumerate() {
        let k = k as u64 + 1;
        sum_di += di;
        
        // Soma de min(k, d_i) de i = k+1 até n
        let mut sum_min = 0_u64;

        for j in k+1 ..= n {
            let dj = sequence[j as usize - 1];
            sum_min += k.min(dj);
        }

        if sum_di > (k*(k-1) + sum_min) {
            return false;
        }
    }

    true
}

fn main() {
    dbg!(is_graphic(vec![3,3,2,1,1,0]));
    dbg!(is_graphic(vec![7, 6, 5, 4, 3, 3, 2]));
}