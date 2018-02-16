fn main() {
    // main for test purposes, will be turned to lib in a future
    let p = Point{x: 10, y: 10};
    let poly = Polygon{
        points: vec!(Point{x: 5, y: 5},Point{x: 15,y: 5},Point{x: 15,y: 15},Point{x: 5, y: 15}),
    };
    let result = p.is_in_polygon(&poly);
    println!("result is_in_polygon is: {}", result);

    let line = Line{
        start: Point{x: 5, y: 5},
        end: Point{x: 20, y: 20},
    };

    let intersect_line = Line{
        start: Point{x: 20, y: 5},
        end: Point{x: 5, y: 20},
    };

    match line.get_intersection(&intersect_line) {
        Some(point) => println!("result get_intersection is: x: {} y: {}", point.x, point.y),
        None => {}
    }

    let p = Point{
        x: 5,
        y: 15,
    };

    let line = Line{
        start: Point{x: 5, y: 5},
        end: Point{x: 5, y: 20},
    };

    let result = p.is_in_line(&line);
    println!("result is_in_line is: {}", result);

    println!("result before get_reversed is: {:?}", poly);
    let rev_poly = poly.get_reversed();
    println!("result after get_reversed is: {:?}", rev_poly);
}

#[derive(Clone,Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Polygon {
    points: Vec<Point>
}

impl Point {
    fn is_in_polygon(self, poly: &Polygon)->bool {
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

    fn is_in_line(self, line: &Line)->bool {
        let dxc = self.x - line.start.x;
        let dyc = self.y - line.start.y;

        let dxl = line.end.x - line.start.x;
        let dyl = line.end.y - line.start.y;

        let cross = dxc * dyl - dyc * dxl;

        if cross != 0 {
            return false
        }

        if dxl.abs() >= dyl.abs() {
            if dxl > 0 {
                line.start.x <= self.x && self.x <= line.end.x
            } else {
                line.end.x <= self.x && self.x <= line.start.x
            }
        } else {
            if dyl > 0 {
                line.start.y <= self.y && self.y <= line.end.y
            } else {
                line.end.y <= self.y && self.y <= line.start.y
            }
        }
    }
}

impl Line {
    fn get_intersection<'a>(self, line: &Line) -> Option<Point> {
        let (line_1_start, line_1_end) = (self.start, self.end);
        let (line_2_start, line_2_end) = (line.start.clone(), line.end.clone());

        let den = ((line_2_end.y - line_2_start.y) * (line_1_end.x - line_1_start.x)) -
            ((line_2_end.x - line_2_start.x) * (line_1_end.y - line_1_start.y));

        if den == 0 {
            return None
        }

        let a = line_1_start.y - line_2_start.y;
        let b = line_1_start.x - line_2_start.x;

        let num_1 = ((line_2_end.x - line_2_start.x) * a) -
            ((line_2_end.y - line_2_start.y) * b);
        let num_2 = ((line_1_end.x - line_1_start.x) * a) -
            ((line_1_end.y - line_1_start.y) * b);

        let a = num_1 as f64 / den as f64;
        let b = num_2 as f64 / den as f64;

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

impl Polygon {
    fn is_clockwise(self)-> bool {
        let mut sum = 0;
        for (i, p) in self.points.iter().enumerate() {
            let mut j = if i != self.points.len() - 1 {i + 1} else {0};
            sum += (self.points[j].x - p.x)*(self.points[j].y + p.y)
        }
        sum > 0
    }
    fn get_reversed(self)->Polygon {
        let points: Vec<Point> = self.points.iter().rev().map(|n| n.clone()).collect();
        Polygon{
            points
        }
    }
}