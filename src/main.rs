fn main() {
    let p = Point{x: 10, y: 10};
    let poly = Polygon{
        points: vec!(Point{x: 5, y: 5},Point{x: 15,y: 5},Point{x: 15,y: 15},Point{x: 5, y: 15}),
    };
    let result = is_pip(p, poly);
    println!("result is: {}", result);
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

struct Polygon {
    points: Vec<Point>
}

fn is_pip(point: Point, poly: Polygon)->bool {
    let (x, y) = (point.x, point.y);
    let mut inside: bool = false;
    let mut j = poly.points.len() - 1;
    for (i, p) in poly.points.iter().enumerate() {
        let (xi, yi) = (p.x, p.y);
        let (xj, yj) = (poly.points[j].x, poly.points[j].y);

        let intersect = ((yi > y) != (yj > y)) &&
            (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

//fn get_lines_intersection(line_a: ((i32, i32), (i32, i32)), line_b: ((i32, i32), (i32, i32)))
//    -> Option<(i32, i32)> {
//    let (line_a_start, line_a_end) = line_a;
//    let (line_b_start, line_b_end) = line_b;
//    let denominator = (line_b_start.2 - line_b_end)
//}