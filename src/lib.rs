pub mod clip;

#[cfg(test)]
mod tests {
    use clip;
    #[test]
    fn is_in_line_works() {
        let p = clip::Point { x: 5, y: 10 };


        let line = clip::Line {
            start: clip::Point { x: 5, y: 5 },
            end: clip::Point { x: 5, y: 20 },
        };

        let result = p.is_in_line(&line);

        assert!(result);

        let p_f = clip::Point { x: 3, y: 4 };


        let line_f = clip::Line {
            start: clip::Point { x: 5, y: 5 },
            end: clip::Point { x: 5, y: 20 },
        };

        let result_f = p_f.is_in_line(&line_f);

        assert!(!result_f);
    }

    #[test]
    fn clip_works() {
        let poly = clip::Polygon {
            points: vec![
                clip::Point {x: 168, y: 408},
                clip::Point {x: 168, y: 108},
                clip::Point {x: 508, y: 108},
                clip::Point {x: 508, y: 408},
                clip::Point {x: 408, y: 408},
                clip::Point {x: 308, y: 208}
            ],
        };
        let inter_polygon = clip::Polygon {
            points: vec![
                clip::Point {x: 72, y: 232},
                clip::Point {x: 342, y: 132},
                clip::Point {x: 422, y: 302},
                clip::Point {x: 92, y: 492},
                clip::Point {x: 292, y: 292}
            ],
        };

        if let Some(polygons) = poly.clip(&inter_polygon) {
            println!("{:?}", polygons);
            assert!(polygons.len() > 0);
        } else {
            println!("here");
            assert!(false)
        }
    }
}
