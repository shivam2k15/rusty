// function takes input vector and returns even number vector
// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     even_nums(&mut vec);
//     println!("{:?}", vec);
// }

// fn even_nums(vec: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < vec.len() {
//         if vec[i] % 2 != 0 {
//             vec.remove(i);
//         }
//         i += 1;
//     }
// }

fn main() {
    let vec = vec![1, 2];
    let rs = even(vec);
    print!("{:?}", rs);
}

fn even(vec: Vec<i32>) -> Vec<i32> {
    let mut rs = Vec::new();
    // for num in vec {
    //     if num % 2 == 0 {
    //         rs.push(num);
    //     }
    // }

    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            rs.push(vec[i]);
        }
    }
    return rs;
}
