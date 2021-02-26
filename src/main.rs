fn main() {
    let data = [1,2,3,43892];
    let x = (0..data.len()).collect::<Vec<_>>();
    let _x_avg = avg(&x);
    println!("{:?} {:?}",standard_deviation(&x),coefficient(&x,&data));
}
fn avg(data: &[usize]) -> f64 {
    data.iter().sum::<usize>() as f64/data.len() as f64
}
fn standard_deviation(data: &[usize]) -> f64 {
    let avg = avg(&data);
    let sum:f64 = data.iter().map(|i|(*i as f64-avg)*(*i as f64-avg)).sum();
    (sum/(data.len() as f64 - 1.)).sqrt()
}
fn coefficient(x: &[usize], y: &[usize]) -> f64 {
    let x_avg = avg(&x);
    let y_avg = avg(&y);
    let x_sum:f64 = x.iter().map(|i|(*i as f64-x_avg)*(*i as f64-x_avg)).sum();
    let y_sum:f64 = y.iter().map(|i|(*i as f64-y_avg)*(*i as f64-y_avg)).sum();
    let denominator = (x_sum * y_sum).sqrt();
    let numerator:f64 = x.iter().map(|i|(*i as f64-x_avg)*(y[*i] as f64-y_avg)).sum();
    numerator/denominator as f64
}