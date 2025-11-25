fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", vec);
    vec.remove(2);
    println!("Vector after removing element at index 2: {:?}", vec);
    vec.push(6);
    println!("Vector after adding element 6: {:?}", vec);
}
