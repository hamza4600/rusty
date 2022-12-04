// array in rust
//
pub fn array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    // 2nd way
    let arr = ["Hzam", "Hassan", "Hassan", "Hassan", "Hassan"];
    println!("arr = {:?}", arr);
    // 3rd way
    let mut arr = [0; 5];
    arr[0] = 100;
    arr[1] = 200;
    arr[2] = 300;
    arr[3] = 400;
    arr[4] = 500;
    println!("arr = {:?}", arr);
}

pub fn array_vector() {
    let mut arra: Vec<i32> = Vec::new(); // it is dynamic array
    arra.push(10);
    arra.push(20);
    arra.push(30);
    arra.push(40);
    arra.push(50);
    println!("arra = {:?}", arra);
    // 2nd way

    let mut names: Vec<String> = Vec::new();
    names.push("Hza345m".to_string());
    names.push("Has456+san".to_string());
    names.push("Has1412san".to_string());
    names.push("Hassan".to_string());
    names.push("H13as*san".to_string());
    println!("names = {:?}", names);

    let arra = vec![10, 20, 30, 40, 50];
    println!("arra = {:?}", arra);
    // 3rd way
    let mut arra = Vec::with_capacity(10);
    arra.push(100);
    arra.push(200);
    arra.push(300);
    arra.push(400);
    arra.push(500);
    println!("arra = {:?}", arra);

    // 4th way
    // let mut arra= vec![0; 5];
}
