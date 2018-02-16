
use std::cmp::Ordering;


fn main() {
    // main for test purposes, will be turned to lib in a future
    let p = Point{x: 10, y: 10};
    let poly = Polygon{
        points: vec!(Point{x: 5, y: 5},Point{x: 15,y: 5},Point{x: 15,y: 15},Point{x: 5, y: 15}),
    };
    let inter_polygon = Polygon{
        points: vec!(Point{x: 10, y: 10},Point{x: 20,y: 10},Point{x: 20,y: 20},Point{x: 10, y: 20}),
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

    if let Some(i) = poly.get_first_outside_vertex_index(&inter_polygon) {
        println!("found outside point at index {} point {:?}", i, poly.points[i]);
    } else {
        println!("didn't find the outside point")
    }

    if let Some(i) = poly.get_first_inside_vertex_index(&inter_polygon) {
        println!("found inside point at index {} point {:?}", i, poly.points[i]);
    } else {
        println!("didn't find the inside point")
    }

    let intersect = Line{
        start: Point{x: 7, y: 2},
        end: Point{x: 25, y: 10},
    };

    //poly.get_inter_vertex_list(&inter_polygon);
    poly.get_intersections_with_line(&intersect);
}

#[derive(Clone,Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone,Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone)]
struct Polygon {
    points: Vec<Point>
}

impl Point {
    fn is_in_polygon(&self, poly: &Polygon)->bool {
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

    fn distance_cmp(&self, first: &Point, second: &Point) -> Ordering {
        let dst_first = (self.x - first.x).abs() + (self.y - first.y).abs();
        let dst_second = (self.x - second.x).abs() + (self.y - second.y).abs();

        if dst_first < dst_second {
            Ordering::Less
        } else if dst_first > dst_second{
            Ordering::Greater
        } else {
            Ordering::Equal
        }
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
    fn get_intersection(self, line: &Line) -> Option<Point> {
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

enum InterVertex {
    InsideVertex(Point),
    OutsideVertex(Point),
    InIntersection(Point),
    OutIntersection(Point),
}

impl Polygon {
    fn is_clockwise(&self)-> bool {
        let mut sum = 0;
        for (i, p) in self.points.iter().enumerate() {
            let mut j = if i != self.points.len() - 1 {i + 1} else {0};
            sum += (self.points[j].x - p.x)*(self.points[j].y + p.y)
        }
        sum < 0
    }
    fn get_reversed(&self)-> Polygon {
        let points: Vec<Point> = self.points.iter().rev().map(|n| n.clone()).collect();
        Polygon{
            points
        }
    }

//    fn get_inter_vertex_list(&self, poly: &Polygon)->bool {
//        let mut subject = self.clone();
//        let mut result = Vec::new();
//        if !subject.is_clockwise() {
//            subject = self.get_reversed();
//        }
//        if let Some(start_index) = subject.get_first_outside_vertex_index(poly) {
//            if let None = subject.get_first_inside_vertex_index(poly) {
//                // if no inside vertexes return poly
//            }
//            subject.points
//                .iter()
//                .enumerate()
//                .skip(start_index)
//                .fold(result, |acc, x| {
//                    let (i, start) = x;
//
//                    // check vertex
//                    if i != start_index && start.is_in_polygon(poly) {
//                        result.push(InsideVertex(start));
//                    } else {
//                        result.push(OutsideVertex(start));
//                    }
//
//                    // check intersection
//                    let next_i = if i == subject.points.len() - 1 {
//                        0
//                    } else {
//                        i + 1
//                    };
//
//                    let end = subject.points[next_i];
//                    let line = Line{
//                        start,
//                        end,
//                    };
//
//                })
//        } else {
//            // if no outside vertexes return self
//        }
//    }

    fn get_intersections_with_line(&self, line: &Line)->bool {
        let mut lines:Vec<Point> = self.points
            .iter()
            .enumerate()
            .filter_map(|x| {
                let (i, start) = x;
                let next_i = if i == self.points.len() - 1 {
                    0
                } else {
                    i + 1
                };
                let end = self.points[next_i].clone();
                let l = Line{
                    start: start.clone(),
                    end,
                };
                l.get_intersection(line)
            })
            .collect();
        println!("{:?}\n", lines);
        lines.sort_by(|a, b|{
            line.start.distance_cmp(a,b)
        });
        println!("{:?}\n", lines);
        true
    }

    fn get_first_outside_vertex_index(&self, poly: &Polygon)-> Option<usize> {
        self.points.iter().position(|ref x|{
            !x.is_in_polygon(poly)
        })
    }

    fn get_first_inside_vertex_index(&self, poly: &Polygon)-> Option<usize> {
        self.points.iter().position(|ref x|{
            x.is_in_polygon(poly)
        })
    }
}