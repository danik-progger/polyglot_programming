#[derive(Debug)]
struct Counter {
    count: usize,
}

fn add_one(counter: &mut Counter) {
    counter.count += 1;
}

fn print_all(items: &Vec<Counter>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    println!("{:?}", counter);
    add_one(&mut counter);
    println!("{:?}", counter);

    let mut vec = vec![Counter { count: 0 }];
    let first = vec.get_mut(0);
    let second = vec.get_mut(1);
    println!("{:?}", second);
    print_all(&vec);


    // ----- ITERATOR -----
    let data = vec![1, 2, 3];
    let mut arr = data.iter().map(|x| x + 1);

    let mut new_vec: Vec<i32> = vec![];

    while let Some(x) = arr.next() {
        new_vec.push(x);
    }

    println!("{:?}", new_vec);

    // ----- READ FILE -----
    let file = std::fs::read_to_string("src/lines.txt").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| {
            println!("{}", line);
        });

    // ----- ENUM -----
    print_color(Color::Green);
    let color = Color::Blue;
    println!("{:?}", color.is_green());

    let mut items: Vec<Item> = vec![];
    append(&mut items);

    // let mut items: Vec<usize> = vec![1, 2, 3];
    // items.push(append(&mut items); -- error

    // ----- ERROR -----

    let agr = std::env::args()
        .nth(1)
        .expect("the file name must be passed in");

    let file = std::fs::read_to_string(agr).expect("unable to read file");

    file.lines().for_each(|line| {
        if let Ok(v) = line.parse::<usize>() {
            println!("{:?}", v);
        } else {
            println!("Line not a number");
        }
    });
}

// ----- ENUM -----
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_part(&self) -> bool {
        match self {
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
            Color::Red => false,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}

struct Man {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    Man(Man),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("hello fem!".to_string()));
}

// ---- OPTION -----
fn foo(n: Option<usize>) -> Option<usize> {
    return Some(n? * 5);
}

fn practice(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}

// --- 3 RULES ---
/*
1. Only 1 owner
2. 1 mutable reference with no immutable
3. Multiply immutable references with mutable

- Reference cannot outlive it's value
*/
