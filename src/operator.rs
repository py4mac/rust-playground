use std::iter::{ Sum};

#[derive(Debug)]
struct TransactionLine {
    debit: u32
}

// impl Sum<u32> for TransactionLine {
//     fn sum<I>(iter: I) -> Self
//     where
//         I: Iterator<Item = u32>,
//     {
//         let mut result = 0;
//         for v in iter {
//             result += v;
//         }
//         TransactionLine { debit: result }
//     }
// }


// impl Iterator for TransactionLine {
//     type Item= u32;
//     fn next(&mut self) -> Option<u32> {
//         println!("Hello"); 
//         Some(self.debit)
//     }
// }

fn sum_debit(a: &[TransactionLine]) -> u32 {
    a.iter().map(|s| s.debit).sum()
}


pub fn run() {
    let mut transactions = vec![
        TransactionLine{debit: 3},
        TransactionLine{debit: 5}
    ];

    let sum_transactions = sum_debit(&transactions);

    println!("{}", sum_transactions);

    // println!("{:?}", transactions.into_iter().sum());

    // // println!("{:?}", transactions);
    for tran in transactions.into_iter().next() {
        println!("{:?}", tran); 
    }

}