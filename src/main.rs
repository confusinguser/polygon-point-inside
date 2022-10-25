use std::ops::Add;

#[deny(clippy::all, clippy::pedantic)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub const ZERO: Self = Point::new(0., 0.);
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    #[must_use]
    pub fn atan2(self) -> f64 {
        self.y.atan2(self.x)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
#[derive(Debug, Clone, PartialEq)]
struct Points {
    pub list: Vec<Point>,
}
impl<'a> IntoIterator for &'a Points {
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Point>>;
    type Item = Point;
    fn into_iter(self) -> Self::IntoIter {
        self.list.iter().copied()
    }
}
impl FromIterator<Point> for Points {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Self {
            list: iter.into_iter().collect(),
        }
    }
}

macro_rules! points {
    ($(($x: expr, $y: expr)),+) => {
        {
            Points { list: vec![$(Point { x: $x, y: $y }),+] }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    slope: f64,
    offset: f64,
    is_reciprocal: bool,
}
#[derive(Debug, Clone, PartialEq)]
struct Lines(pub Vec<Line>);

fn main() {
    let mut points = points![(-1., 4.), (3., 3.), (-3., -2.)];
    let testcases = points![(-2., 2.), (-1., 2.), (2., 3.36)];
    let midpoint = points.get_mean_point();
    points.sort_points_for_lines(midpoint);
    let lines = points.get_lines();

    for test in &testcases {
        let midpoint_over_under_lines = lines.point_over_under_lines(midpoint);
        let point_inside = lines.point_is_inside_polygon(test, midpoint_over_under_lines);
        println!("{test:?} is inside {point_inside}");
    }
}

impl Points {
    fn sort_points_for_lines(&mut self, midpoint: Point) {
        // maybe do the angle calculations in `sort_by`?
        // Then, we'd call atan2 multiple times, but we don't need to allocate 2 more Vecs
        let mut angles = Vec::with_capacity(self.list.len());
        for point in &*self {
            let offset_point = Point::new(point.x - midpoint.x, point.y - midpoint.y);
            angles.push((point, offset_point.atan2()));
        }
        angles.sort_by(|a, b| b.1.total_cmp(&a.1));
        *self = angles.into_iter().map(|a| a.0).collect();
    }

    fn get_mean_point(&self) -> Point {
        let sum = self.into_iter().fold(Point::ZERO, |a, b| a + b);
        let denominator = self.list.len() as f64;
        Point::new(sum.x / denominator, sum.y / denominator)
    }

    fn get_lines(&self) -> Lines {
        Lines(
            self.into_iter()
                .enumerate()
                .map(|(i, point)| {
                    let next_point = self.list[(i + 1) % self.list.len()];
                    let dy = point.y - next_point.y;
                    let dx = point.x - next_point.x;
                    let slope;
                    let is_reciprocal;
                    let offset;

                    if dy.abs() > dx.abs() {
                        slope = dy / dx;
                        is_reciprocal = false;
                        offset = point.y - slope * point.x;
                    } else {
                        slope = dx / dy;
                        is_reciprocal = true;
                        offset = point.x - slope * point.y;
                    }
                    Line {
                        slope,
                        offset,
                        is_reciprocal,
                    }
                })
                .collect(),
        )
    }
}

impl Lines {
    // Iterator to avoid allocation on each check
    fn point_over_under_lines(&self, point: Point) -> impl Iterator<Item = bool> + '_ {
        self.0.iter().map(move |line| {
            if line.is_reciprocal {
                (point.x - line.slope * point.y - line.offset).is_sign_positive()
            } else {
                (point.y - line.slope * point.x - line.offset).is_sign_positive()
            }
        })
    }

    fn point_is_inside_polygon(
        &self,
        point: Point,
        midpoint_over_under_lines: impl IntoIterator<Item = bool>,
    ) -> bool {
        let point_over_under_lines = self.point_over_under_lines(point);
        // if iterator equals
        point_over_under_lines.eq(midpoint_over_under_lines.into_iter())
    }
}
