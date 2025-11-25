fn main() {
    //copy trait
    let x = 1;
    let y = x;
    let z = y;
    println!("x: {}, y: {}, z: {}", x, y, z);

    //copy trait for reference types
    let a = "string".to_string();
    println!("a: {}", a);
    let b = a;
    println!("b: {}", b);
    // println!("a: {}", a);

    //borrow of moved value. Below p value is used after it's moved to q.
    //The below commented line will cause a compiler error.
    let p = vec!["hello".to_string()];
    let q = p;
    // println!("p: {:?}", p);
    println!("q: {:?}", q);

    //clone
    let c = vec!["hello".to_string()];
    let d = c.clone();
    println!("c: {:?}", c);
    println!("d: {:?}", d);


    let s = String::from("takes");
    takes_ownership(s);
    // println!("s: {}", s); - Errors out as s has been moved to the function

    let t = 5;
    make_copy(t);

    let e: String = give_ownership();
    println!("e: {}", e);

    let str3: String = take_and_give_back(e);
    println!("str3: {}", str3);
    // println!("e: {}", e); - Already ownership given to str3

    if true {
        let str4:String = str3;
        println!("str4: {}", str4);
    } else {
        let str5: String = str3;
        println!("str5: {}", str5); // No error in compile time or even run time
    }
    // println!("str3:{}", str3); //Error as str3 already moved on


    let mut pass_ref:String = String::from("hello");
    mutate_string(&mut pass_ref);
    println!("pass_ref: {}", &mut pass_ref);

    fn mutate_string(some_str: &mut String) {
        some_str.push_str(", world")
    }




    fn takes_ownership(s: String) {
        let stri = s;
        println!("s: {}", stri);
    }

    fn make_copy(x: i32) {
        let y = x;
        println!("y: {}", y);
    }

    fn give_ownership() -> String {
        "given ownership".to_string()
    }

    fn take_and_give_back(s: String) -> String {
        s
    }
}
