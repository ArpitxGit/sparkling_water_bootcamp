use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

fn lagrange_interpolation(points: &[Point], target_x: f64) -> f64 {
    let mut result = 0.0;
    for i in 0..points.len() {
        let mut term = points[i].y;
        for j in 0..points.len() {
            if j != i {
                term *= (target_x - points[j].x) / (points[i].x - points[j].x);
            }
        }
        result += term;
    }
    result
}

fn read_points_from_file(file_path: &str) -> Result<Vec<Point>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let points: Vec<Point> = serde_json::from_reader(reader)?;
    Ok(points)
}

fn main() {
    println!("Enter the path to the file containing points data:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read input");
    let file_path = file_path.trim();

    let points = match read_points_from_file(&file_path) {
        Ok(points) => points,
        Err(err) => {
            eprintln!("Error reading points from file: {}", err);
            return;
        }
    };

    println!("Enter the value of x for interpolation:");
    let mut target_x = String::new();
    io::stdin().read_line(&mut target_x).expect("Failed to read input");
    let target_x: f64 = match target_x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input, please enter a valid number");
            return;
        }
    };

    let interpolated_y = lagrange_interpolation(&points, target_x);
    println!("Interpolated y value: {}", interpolated_y);
}
