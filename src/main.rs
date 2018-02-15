fn main() {
    // main for test purposes, will be turned to lib in a future
    let p = Point{x: 10, y: 10};
    let poly = Polygon{
        points: vec!(Point{x: 5, y: 5},Point{x: 15,y: 5},Point{x: 15,y: 15},Point{x: 5, y: 15}),
    };
    let result = p.is_in_polygon(poly);
    println!("result is_in_polygon is: {}", result);

    let line = Line{
        start: Point{x: 5, y: 5},
        end: Point{x: 20, y: 20},
    };

    let intersect_line = Line{
        start: Point{x: 20, y: 5},
        end: Point{x: 5, y: 20},
    };

    match line.get_intersection(intersect_line) {
        Some(point) => println!("result get_intersection is: x: {} y: {}", point.x, point.y),
        None => {}
    }
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

impl Point {
    fn is_in_polygon(self, poly: Polygon)->bool {
        let (x, y) = (self.x, self.y);
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
}

impl Line {
    fn get_intersection(self, line: Line) -> Option<Point> {
        let (line_1_start, line_1_end) = (self.start, self.end);
        let (line_2_start, line_2_end) = (line.start, line.end);

        let denominator = ((line_2_end.y - line_2_start.y) * (line_1_end.x - line_1_start.x)) -
            ((line_2_end.x - line_2_start.x) * (line_1_end.y - line_1_start.y));

        if denominator == 0 {
            return None
        }

        let a = line_1_start.y - line_2_start.y;
        let b = line_1_start.x - line_2_start.x;

        let numerator1 = ((line_2_end.x - line_2_start.x) * a) -
            ((line_2_end.y - line_2_start.y) * b);
        let numerator2 = ((line_1_end.x - line_1_start.x) * a) -
            ((line_1_end.y - line_1_start.y) * b);

        let a = numerator1 as f64 / denominator as f64;
        let b = numerator2 as f64 / denominator as f64;

        if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
            return None
        }

        let result = Point{
            x: line_1_start.x + (a * (line_1_end.x - line_1_start.x) as f64).round() as i32,
            y: line_1_start.y + (a * (line_1_end.y - line_1_start.y) as f64).round() as i32,
        };

        Some(result)
    }
}

