//this code was written at 3am, if it ain't broke, don't fix it
pub fn get_points_between(a: (f32, f32), b: (f32, f32)) -> Vec<(f32, f32)> {
    let mut points = vec![];

    let x = b.0 - a.0;
    let y = b.1 - a.1;

    if x.abs() >= y.abs() {
        for i in 1..x.abs() as i32 {
            points.push((
                a.0 + i as f32 * x.signum(),
                (a.1 + i as f32 * (y / x).abs() * y.signum()).round(),
            ));
        }
        return points;
    }
    for i in 1..y.abs() as i32 {
        points.push((
            (a.0 + i as f32 * (x / y).abs() * x.signum()).round(),
            a.1 + i as f32 * y.signum(),
        ))
    }
    points
}
