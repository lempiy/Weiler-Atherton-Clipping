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
                clip::Point { x: 16, y: 40 },
                clip::Point { x: 16, y: 10 },
                clip::Point { x: 50, y: 10 },
                clip::Point { x: 50, y: 40 },
                clip::Point { x: 40, y: 40 },
                clip::Point { x: 30, y: 20 },
            ],
        };
        let inter_polygon = clip::Polygon {
            points: vec![
                clip::Point { x: 8, y: 24 },
                clip::Point { x: 35, y: 14 },
                clip::Point { x: 43, y: 31 },
                clip::Point { x: 44, y: 50 },
                clip::Point { x: 30, y: 30 },
            ],
        };

        if let Some(polygons) = poly.clip(&inter_polygon) {
            assert!(polygons.len() > 0);
        } else {
            assert!(false)
        }
    }
}
