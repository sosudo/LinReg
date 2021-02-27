pub fn avg(data: &[usize]) -> f64 {
    data.iter().sum::<usize>() as f64/data.len() as f64
}
pub fn standard_deviation(data: &[usize]) -> f64 {
    (variance(data)/(data.len() as f64 - 1.)).sqrt()
}
pub fn coefficient(indices: &[usize], x: &[usize], y: &[usize]) -> f64 {
    let denominator = (variance(&x) * variance( &y)).sqrt();
    let numerator:f64 = indices.iter().map(|i|(x[*i] as f64 - avg(x))*(y[*i] as f64 - avg(y))).sum();
    numerator/denominator
}
pub fn variance(data: &[usize]) -> f64 {
    let avg = avg(data);
    data.iter().map(|i|(*i as f64-avg) * (*i as f64-avg)).sum()
}
pub fn plug(slope: f64, x_avg: f64, y_avg: f64, x: &[usize]) -> Vec<(f64, f64)> {
    let y_values: Vec<_> = x.iter().map(|i| slope * (*i as f64 - x_avg) + y_avg).collect();
    let converted_x: Vec<_> = x.iter().map(|i| *i as f64).collect();
    converted_x.into_iter().zip(y_values.into_iter()).collect::<Vec<_>>()
}