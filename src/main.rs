fn main() {

    let data = vec![1, 2, 3];
    let mut arr = data
        .iter()
        .map(|x| x + 1);

    let mut new_vec: Vec<i32> = vec![];

    while let Some(x) = arr.next() {
        new_vec.push(x);
    }

    println!("{:?}", new_vec);
}
