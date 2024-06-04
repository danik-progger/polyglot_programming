mod shapes;
use std::{fmt::Display, str::FromStr};

use shapes::{area::Area, circle::Circle, dumb_collision::{Contains, Points, Collidable}, rect::Rect};
use anyhow::Result;

enum Shape {
    Rect(Rect),
    Circle(Circle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => Ok(Shape::Rect(data.parse()?)),
            "circle" => Ok(Shape::Circle(data.parse()?)),
            _ => Err(anyhow::anyhow!("Invalid shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::dumb_collision::PointIter {
        match self {
            Shape::Rect(rect) => rect.points(),
            Shape::Circle(cir) => cir.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        match self {
            Shape::Rect(rect) => rect.contains_point((x, y)),
            Shape::Circle(cir) => cir.contains_point((x, y)),
        }
    }
}

impl Display for &Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rect(rect) => write!(f, "{}", rect),
            Shape::Circle(cir) => write!(f, "{}", cir),
        }
    }
}

fn main() -> Result<()> {
    let rec = Rect::default();
    let cir = Circle {
        x: 0.0,
        y: 0.0,
        radius: 30.0,
    };
    println!("Rectangle area: {}", rec.area());
    println!("Circle area: {}", cir.area());
    println!("{}", rec);

    for point in rec {}


    let shapes = std::fs::read_to_string("src/shapes.txt")?
    .lines()
    .filter_map(|x| x.parse::<Shape>().ok())
    .collect::<Vec<_>>();

    let iter = shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1));

    iter.filter(|(s, b)| s.collide(b))
    .for_each(|(s, b)| println!("{} {}", s, b));

    return Ok(());
}
