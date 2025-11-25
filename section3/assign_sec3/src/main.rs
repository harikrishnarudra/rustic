fn main() {
    let mut vec = vec![1,3,5,7];
    println!("{:?}", &mut vec);
    let ret_val = mutate_vec(&mut vec);
    println!("{:?}", ret_val);
    vec.push(15);
    println!("{:?}", &mut vec);


    println!("--- second question -- ");
    let x = 5;
    let y = add_two(x);
    println!("x after adding two: {}", x);
    println!("result of adding with two: {}", y);
        
}

fn mutate_vec(v : &mut Vec<i32>) -> bool {
    println!("Mutating vector={:?}", v);
    if v[0] == 1 {
        return true
    } 
    return false
}

fn add_two (val: i8) -> i8 {
    val + 2
}