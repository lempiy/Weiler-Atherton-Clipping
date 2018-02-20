use std::cmp::Ordering;


fn main() {
    // main for test purposes, will be turned to lib in a future
    let poly = Polygon{
        points: vec!(
            Point{x: 1, y: 1},
            Point{x: 7,y: 3},
            Point{x: 1,y: 8},
            Point{x: 3, y: 4},
            Point{x: 1, y: 3},
        ),
    };
    let inter_polygon = Polygon{
        points: vec!(
            Point{x: 4, y: 3},
            Point{x: 5,y: 4},
            Point{x: 8,y: 1},
            Point{x: 8, y: 5},
            Point{x: 4, y: 7}),
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

    fn get_first_in_intersection(list: &mut Vec<InterVertex>)->Option<Point> {
        let mut found = 0;
        let mut result = None;
        if let Some(p) = list.iter()
            .enumerate()
            .find(|x| {
                let (i, x) = *x;
                if let InterVertex::InIntersection(_) = *x {
                    found = i;
                    return true
                }
                false
            }) {
            result = Some(p.1.get_point());
        };
        if found > 0 {
            for _ in 0..found {
                list.remove(0);
            };
        }
        result
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
                    PolyListOption::List(mut clip_list) => {
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

    fn get_clip_polygons(mut subject: Vec<InterVertex>, mut clip: Vec<InterVertex>)
                         ->Option<Vec<Vec<Point>>> {
        let mut result: Vec<Vec<Point>> = Vec::new();
        while let Some(start_point) =
            InterVertex::get_first_in_intersection(&mut subject) {
            if let Some(poly) =
                Polygon::get_clip_polygon(&mut subject, &mut clip, start_point.clone()) {
                result.push(poly);
            } else {
                break;
            }
        };
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    fn get_clip_polygon(subject: &mut Vec<InterVertex>, clip: &mut Vec<InterVertex>, initial: Point)
        ->Option<Vec<Point>> {
        let mut result: Vec<Point> = Vec::new();


        let mut subject_as_list = true;
        let mut start_point = initial.clone();
        let mut end_point = subject[subject.len()-1].clone().get_point();
        while initial != end_point {
            if let Some(values) = Polygon::collect_from_list(
                if subject_as_list{subject}else{clip}
                , start_point) {
                let (mut edges, end) = values;
                end_point = end.clone();
                start_point = end.clone();
                if subject_as_list {
                    subject_as_list = false;
                } else {
                    subject_as_list = true;
                }
                result.append(&mut edges);
            } else {
                println!("something went wrong");
                println!("res {:?}", result);
                return None
            }
        };
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    fn collect_from_list(list: &mut Vec<InterVertex>, start_point: Point)
        ->Option<(Vec<Point>, Point)> {
        let mut initial_vertex_not_found = true;
        let mut last_point: Option<Point> = None;
        let (mut start_i, mut end_i) = (0,0);
        let dont_skip = list[0].get_point() == start_point;
        let points:Vec<Point> = list
            .iter()
            .enumerate()
            .skip_while(|x|{
                // need to skip until InIntersection occurs,
                // but include the InIntersection
                if dont_skip {return false};
                let (i, _) = *x;
                let next = if i == list.len() - 1 {
                    0
                } else {
                    i + 1
                };

                let next_point = &list[next];
                if next_point.get_point() == start_point {
                    start_i = next;
                    initial_vertex_not_found = false;
                    return true
                }
                initial_vertex_not_found
            })
            .take_while(|x| {
                let (i, x) = *x;

                if let InterVertex::OutIntersection(ref p) = *x {
                    end_i = i;
                    last_point = Some(p.clone());
                    return false
                }
                true
            })
            .map(|x| {
                let (_, x) = x;

                x.get_point()
            })
            .collect();
        for i in start_i..end_i {
            list.remove(i);
        }
        if points.len() > 0 {
            Some((points, last_point.unwrap()))
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
        let mut intersection_count = 0;
        if let Some(start_index) = subject.get_first_outside_vertex_index(poly) {
            if let None = subject.get_first_inside_vertex_index(poly) {
                if poly.points.iter().all(|x| { x.is_in_polygon(&subject) }) {
                    return PolyListOption::InsidePoly(poly.points.clone());
                }
            };
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
                    let mut intersections = poly
                        .get_intersections_with_line(&line, &mut cursor_inside);
                    intersection_count = intersection_count + intersections.len();
                    acc.append(&mut intersections);
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
