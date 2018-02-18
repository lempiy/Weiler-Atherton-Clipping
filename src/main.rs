use std::cmp::Ordering;


fn main() {
    // main for test purposes, will be turned to lib in a future
    let poly = Polygon{
        points: vec!(Point{x: 1, y: 1},Point{x: 5,y: 2},Point{x: 6,y: 6},Point{x: 2, y: 8},Point{x: 3, y: 5}),
    };
    let inter_polygon = Polygon{
        points: vec!(Point{x: 4, y: 4},Point{x: 8,y: 5},Point{x: 5,y: 8},Point{x: 4, y: 6}, Point{x: 1, y: 5}),
    };

    let p = Point{
        x: 5,
        y: 15,
    };

    let line = Line{
        start: Point{x: 5, y: 5},
        end: Point{x: 5, y: 20},
    };

    let _result = p.is_in_line(&line);

    if let Some(polygons) = poly.clip(&inter_polygon) {
        println!("CLIP RESULT {:?}", polygons);
    } else {
        println!("None");
    }
}

#[derive(Clone,Debug,Copy)]
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

#[derive(Clone,Debug)]
enum PolyListOption {
    List(Vec<InterVertex>),
    InsidePoly(Vec<Point>),
    None
}

#[derive(Clone,Debug,Copy)]
enum InterVertex {
    InsideVertex(Point),
    OutsideVertex(Point),
    InIntersection(Point),
    OutIntersection(Point),
}

impl InterVertex {
    fn get_point(&self) -> Point {
        match *self {
            InterVertex::InIntersection(ref p) => p.clone(),
            InterVertex::OutIntersection(ref p) => p.clone(),
            InterVertex::InsideVertex(ref p) => p.clone(),
            InterVertex::OutsideVertex(ref p) => p.clone(),
        }
    }

    fn get_first_in_intersection(list: &Vec<InterVertex>)->Option<Point> {
        if let Some(p) = list
            .iter()
            .find(|x| {
                if let InterVertex::InIntersection(_) = **x {
                    return true
                }
                false
            }) {
            Some(p.get_point())
        } else {
            None
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other:&Point) ->bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

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

    fn is_in_line(&self, line: &Line)->bool {
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

    fn clip(&self, other: &Polygon) -> Option<Vec<Vec<Point>>> {
        let option = self.get_inter_vertex_list(other);
        let other_option = other.get_inter_vertex_list(self);
        match option {
            PolyListOption::List(subject_list) => {
                match other_option {
                    PolyListOption::List(clip_list) => {
                        Polygon::get_clip_polygons(subject_list, clip_list)
                    },
                    PolyListOption::InsidePoly(list) => Some(vec![list]),
                    PolyListOption::None => None,
                }
            },
            PolyListOption::InsidePoly(list) => Some(vec![list]),
            PolyListOption::None => None,
        }
    }

    fn get_clip_polygons(subject: Vec<InterVertex>, clip: Vec<InterVertex>)
        ->Option<Vec<Vec<Point>>> {
        let mut result: Vec<Vec<Point>> = Vec::new();

        if subject.len() < 2 || clip.len() < 2 {
            return None
        }
        // first point is always outside vertex so we can start from second
        let initial = InterVertex::get_first_in_intersection(&subject).unwrap();
        let mut start = initial.clone();
        println!("START {:?}\n{:?}\n{:?}", start, subject, clip);
        let mut k = 0;
        loop {
            println!("THIS 0 {:?}\n ===> {:?}", start, subject);
            if let Some(value) = Polygon::collect_from_list(&subject, &mut start, true) {
                let (mut subject_edges, end) = value;
                println!("THIS 1 {:?}\n{:?}", clip, end);
                if let Some(value) = Polygon::collect_from_list(&clip, end, false) {
                    println!("THIS 2");
                    let (mut clip_edges, _) = value;
                    subject_edges.append(&mut clip_edges);
                    println!("{:?}", subject_edges);
                    result.push(subject_edges);
                }

                println!("HERE {:?} {:?}", end, initial);
                if *end == initial {
                    break
                }
            }
            k = k+1;
            if k > 3 {
                break
            }
        }
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    fn collect_from_list<'a>(list: &Vec<InterVertex>, last_point: &'a mut Point, is_subject: bool)
        ->Option<(Vec<Point>, &'a mut Point)> {
        let mut enter_vertex_not_found = true;
        let mut initial_vertex_not_found = true;
        let point_value = last_point.clone();
        let points:Vec<Point> = list
            .iter()
            .enumerate()
            .skip_while(|x|{
                // need to skip until InIntersection occurs,
                // but include the InIntersection
                let (i, _) = *x;
                let next = if i == list.len() - 1 {
                    0
                } else {
                    i + 1
                };

                let next_point = &list[next];

                if next_point.get_point() == point_value {
                    initial_vertex_not_found = false;
                    println!("found initial {:?}", next_point);
                    return true
                }
                initial_vertex_not_found
            })
            .skip_while(|x| {
                if !is_subject {
                    return false
                };

                let (i, p) = *x;
                if let &InterVertex::InIntersection(_) = p {
                    return false;
                }
                let next = if i == list.len() - 1 {
                    0
                } else {
                    i + 1
                };

                let next_point = &list[next];
                if let &InterVertex::InIntersection(_) = next_point {
                    enter_vertex_not_found = false;
                    if !enter_vertex_not_found {println!("found enter {:?}", next_point)};
                    return true
                }
                enter_vertex_not_found
            })
            .take_while(|x| {
                let (_, x) = *x;
                if let InterVertex::OutIntersection(ref p) = *x {
                    if is_subject {*last_point = p.clone()};
                    return false
                }
                true
            })
            .map(|x| {
                println!("map {:?}", x);
                let (_, x) = x;
                x.get_point()
            })
            .collect();
        if points.len() > 0 {
            Some((points, last_point))
        } else {
            None
        }
    }

    fn get_inter_vertex_list(&self, poly: &Polygon)->PolyListOption {
        let mut subject = self.clone();
        if !subject.is_clockwise() {
            subject = self.get_reversed();
        }
        let mut cursor_inside = false;
        if let Some(start_index) = subject.get_first_outside_vertex_index(poly) {
            if let None = subject.get_first_inside_vertex_index(poly) {
                if poly.points.iter().all(|x| { x.is_in_polygon(&subject) }) {
                    return PolyListOption::InsidePoly(poly.points.clone());
                }
                return PolyListOption::None;
            }
            let result = subject.points
                .iter()
                .enumerate()
                .skip(start_index)
                .chain(
                    subject
                        .points
                        .iter()
                        .enumerate()
                        .take(start_index)
                )
                .fold(Vec::new(), |mut acc, x| {
                    let (i, start) = x;

                    // check vertex
                    if i != start_index && start.is_in_polygon(poly) {
                        acc.push(InterVertex::InsideVertex(start.clone()));
                    } else {
                        acc.push(InterVertex::OutsideVertex(start.clone()));
                    }

                    // check intersection
                    let next_i = if i == subject.points.len() - 1 {
                        0
                    } else {
                        i + 1
                    };

                    let end = subject.points[next_i].clone();
                    let line = Line{
                        start: start.clone(),
                        end,
                    };

                    acc.append(&mut poly.get_intersections_with_line(&line, &mut cursor_inside));
                    acc
                });
            PolyListOption::List(result)
        } else {
            PolyListOption::InsidePoly(self.points.clone())
        }
    }

    fn get_intersections_with_line(&self, line: &Line, cursor_inside: &mut bool)->Vec<InterVertex> {
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
                let l = Line {
                    start: start.clone(),
                    end,
                };
                l.get_intersection(line)
            })
            .collect();
        lines
            .sort_by(|a, b|{
                line.start.distance_cmp(a,b)
            });
        lines
            .iter()
            .map(|x|{
                if *cursor_inside {
                    *cursor_inside = !*cursor_inside;
                    InterVertex::OutIntersection(x.clone())
                } else {
                    *cursor_inside = !*cursor_inside;
                    InterVertex::InIntersection(x.clone())
                }
            })
            .collect()
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
