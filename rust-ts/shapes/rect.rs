use std::fmt::Display;
use std::str::FromStr;

use super::area::Area;
use super::dumb_collision::{Contains, Points};
use anyhow::Result;


pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        return Rect {
            x: 0.0,
            y: 0.0,
            width: 1.0,
            height: 1.0,
        };
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle:({}, {})  {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

impl Contains for Rect {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height;
    }
}

impl Points for Rect {
    fn points(&self) -> super::dumb_collision::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Invalid rect: {}", s));
        }
        return Ok(Rect {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        })
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;
        return self.points.get(idx).map(|x| *x);
    }
}

impl IntoIterator for Rect {
    type IntoIter = RectIter;
    type Item = (f64, f64);
    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}

impl IntoIterator for &Rect {
    type IntoIter = RectIter;
    type Item = (f64, f64);
    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}

impl From<&Rect> for RectIter {
    fn from(rect: &Rect) -> Self {
        return RectIter {
            points: vec![
                (rect.x, rect.y),
                (rect.x + rect.width, rect.y),
                (rect.x, rect.y + rect.height),
                (rect.x + rect.width, rect.y + rect.height),
            ],
            idx: 0,
        };
    }
}
