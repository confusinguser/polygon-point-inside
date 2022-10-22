fn main() {
    let points = vec![(-1., 4.), (3.,3.), (-3.,-2.)];
    let testcases = vec![(-2.,2.),(-1., 2.), (2.,3.36)];
    let midpoint = find_midpoint(&points);
    let points = sort_points_for_lines(points, midpoint);
    let lines = find_line_values_from_points(&points);
    let midpoint_over_under_lines = point_over_under_lines(&lines, midpoint);
    for test in testcases {
        let point_inside = point_is_inside_polygon(&lines, test, &midpoint_over_under_lines);
        println!("{test:?} is inside {point_inside}");
    }
}

fn sort_points_for_lines(points: Vec<(f32, f32)>, midpoint: (f32, f32)) -> Vec<(f32, f32)> {
    let mut angles = Vec::with_capacity(points.len());
    for point in points.iter() {
        let offset_point = ((point.0-midpoint.0),(point.1-midpoint.1)); 
        angles.push((point.to_owned(), (offset_point.1/offset_point.0).atan()));
    };
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    angles.iter().map(|a| a.0).collect()
}

fn find_midpoint(points: &Vec<(f32, f32)>) -> (f32, f32) {
    let sum = points.iter().fold((0.,0.), |a,b| (a.0+b.0,a.1+b.1));
    let div = points.len() as f32;
    (sum.0/div, sum.1/div)
}

fn find_line_values_from_points(points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut lines = Vec::with_capacity(points.len());
    for (i, point) in points.iter().enumerate() {
        let next_point = points.get((i+1)%points.len()).unwrap();
        let k = (point.1-next_point.1)/(point.0-next_point.0);
        let m = point.1-k*point.0;
        lines.push((k,m));
    }
    lines
}

fn point_over_under_lines(lines: &Vec<(f32, f32)>, point: (f32, f32)) -> Vec<bool> {
    let mut output = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        output.push((point.1-line.0*point.0-line.1).is_sign_positive());
    }
    output
}

fn point_is_inside_polygon(lines: &Vec<(f32, f32)>, point: (f32, f32), midpoint_over_under_lines: &Vec<bool>) -> bool {
    let point_over_under_lines = point_over_under_lines(lines, point);
    return point_over_under_lines == *midpoint_over_under_lines
}
