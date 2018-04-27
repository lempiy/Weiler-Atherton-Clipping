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
                clip::Point {x: 180, y: 420},
                clip::Point {x: 180, y: 120},
                clip::Point {x: 520, y: 120},
                clip::Point {x: 520, y: 420},
                clip::Point {x: 420, y: 420},
                clip::Point {x: 320, y: 220}
            ],
        };
        let inter_polygon = clip::Polygon {
            points: vec![
                clip::Point {x: 60, y: 220},
                clip::Point {x: 330, y: 120},
                clip::Point {x: 410, y: 290},
                clip::Point {x: 80, y: 480},
                clip::Point {x: 280, y: 280}
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
