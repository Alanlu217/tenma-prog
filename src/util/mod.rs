pub fn get_range(start: f64, end: f64, step: f64) -> Vec<f64> {
    let num: u32 = ((end - start) / step.abs()).abs().floor() as u32;

    if end < start {
        (0..(num + 1))
            .map(|x| start - x as f64 * step.abs())
            .collect()
    } else {
        (0..(num + 1))
            .map(|x| x as f64 * step.abs() + start)
            .collect()
    }
}

#[test]
fn test_range() {
    assert_eq!(get_range(7.0, 4.0, 2.0), vec![0., 1., 2., 3., 4., 5.0]);
}
