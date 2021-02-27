use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointStyle};
pub fn main() {
    let data = [0,1,2,3,4,5];
    let indices = (0..data.len()).collect::<Vec<_>>();
    let x = [20,25,30,35,40,46];
    let converted_data: Vec<_> = data.iter().map(|i| *i as f64).collect();
    let converted_x: Vec<_> = x.iter().map(|i| *i as f64).collect();
    println!("Mean: {}", avg(&data));
    println!("Standard Deviation: {}", standard_deviation(&data));
    println!("Correlation Coefficient: {}", coefficient(&indices, &x, &data));
    let slope: f64 = (coefficient(&indices, &x, &data) * standard_deviation(&data))/(standard_deviation(&x));
    match slope as i32 {
        -1 => println!("Line of Best Fit in Standard Form: y - {} = -(x - {})", avg(&data), avg(&x)),
        1 => println!("Line of Best Fit in Standard Form: y - {} = x - {}", avg(&data), avg(&x)),
        _ => println!("Line of Best Fit in Standard Form: y - {} = {}(x - {})", avg(&data), slope, avg(&x))
    }
    let dataset = converted_x.into_iter().zip(converted_data.into_iter()).collect::<Vec<_>>();
    let plot_one: Plot = Plot::new(dataset).point_style(
        PointStyle::new()
            .colour("#000000")
    );
    let view = ContinuousView::new()
        .add(plot_one)
        .x_label("X-Axis")
        .y_label("Data");
    Page::single(&view).save("plot.svg").unwrap();
}
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
/*
pub fn plug(slope: &[usize], x: &[usize], y: &[usize], ) -> f64 {
    
}*/