fn main() {
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
