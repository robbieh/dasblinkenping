#[allow(dead_code)]
#[allow(unused)]

use std::collections::HashMap;

const L: &str  = "+RF-LFL-FR+";
const R: &str  = "-LF+RFR+FL-";

#[derive(Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn apply_hilbert(rules: String, iterations: isize, dir: &mut isize, 
                     points:&mut Vec<Point>, current: &mut Point) {
    println!("{:#?} {:#?} {:?}", current, rules, iterations);
    for c in rules.chars() {
        match c {
            'R' => if iterations != 0 
                { apply_hilbert(R.to_string(), iterations - 1, dir, points, current)},
            'L' => if iterations != 0 
                { apply_hilbert(L.to_string(), iterations - 1, dir, points, current)},
            '+' => *dir += 1,
            '-' => *dir -= 1,
            'F' => {
                    let x = current.x.clone();
                    let y = current.y.clone();
                    points.push(current.clone());
                    match *dir % 4 {
                        0 => *current = Point{x: x, y: y - 1 },
                        1 => *current = Point{x: x + 1, y: y },
                        2 => *current = Point{x: x, y: y + 1 },
                        3 => *current = Point{x: x - 1, y: y },
                        _ => {},
                    };
                },
            _ => {},
        };
    }
}


pub fn hilbert(iterations: isize ) -> Vec<Point>{
    let mut points = Vec::<Point>::new();
    let mut point = Point{x: 1,y: 1};
    apply_hilbert(L.to_string(), iterations, &mut 5, &mut points, &mut point);
    points.push(point.clone());
    points
}

