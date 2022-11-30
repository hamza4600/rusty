// Slice ad Strucs

fn slice_con() {
    let n1 = "I am Learning Rust".to_string();
    println!("String is {}", n1.len());
    let c = &n1[0..5];
    println!("Slice is :: {}", c);
}
// can also mutat slice

fn use_slice(data: &mut [i32]) {
    println!("Length of slice is {}", data.len());
    println!("{:?}", data);
    data[0] = 1080;
}
fn mut_slice() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data);
    println!("{:?}", data);
}

//  sructs
struct Employ {
    name: String,
    age: i32,
    salary: f32,
}

// let mut Student {  //
//     name: String::from("USA"),
//     class: 20,
//     place: 10000.0,
// }

//  return stru from a fucnction

fn create_employ() -> Employ {
    Employ {
        name: String::from("Rust"),
        age: 20,
        salary: 10000.0,
    }
}

//  conpare
fn compare(em: Employ, em1: Employ) -> Employ {
    if em.age == em1.age {
        // println!("Both are same");
        return em;
    } else {
        // println!("Both are not same");
        return em1;
    }
}
fn display(emp: Employ) {
    println!("Name is {}", emp.name);
    println!("Age is {}", emp.age);
    println!("Salary is {}", emp.salary);
}
fn struc_rust() {
    let emp1 = Employ {
        name: "Hamza".to_string(),
        age: 20,
        salary: 10000.0,
    };
    println!(
        "Name is {} age {} salary {}",
        emp1.name, emp1.age, emp1.salary
    );
    let emp2 = create_employ();
    println!(
        "Name is {} age {} salary {}",
        emp2.name, emp2.age, emp2.salary
    );
    let mut emp3 = create_employ();
    emp3.name = "Kubra".to_string();
    println!(
        "Name is {} age {} salary {}",
        emp3.name, emp3.age, emp3.salary
    );

    let elder = compare(emp1, emp2);
    display(elder);
}

//  adding methods to struc
struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

//  static methods
struct Poinst {
    x: i32,
    y: i32,
    z: i32,
}
impl Poinst {
    fn get_point(x: i32 , y: i32 , z: i32) -> Poinst {
        Poinst {
            x: x,
            y: y,
            z: z,
        }
    }
    fn display (&self) {
        println!("X = {} Y = {} Z = {}", self.x, self.y, self.z);
    }
}

fn main() {
    slice_con();
    mut_slice();
    struc_rust();

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area is {}", rect.area());

    let pot = Poinst::get_point(10, 20, 30);
    pot.display();

}
