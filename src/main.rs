use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointStyle};
mod math;
use math::*;
pub fn main() {
    let x = [20,25,30,35,40,46];
    let data = [0,1,2,3,4,5];
    let indices = (0..data.len()).collect::<Vec<_>>();
    let converted_data: Vec<_> = data.iter().map(|i| *i as f64).collect();
    let converted_x: Vec<_> = x.iter().map(|i| *i as f64).collect();
    println!("Mean: {}", avg(&data));
    println!("Standard Deviation: {}", standard_deviation(&data));
    println!("Correlation Coefficient: {}", coefficient(&indices, &x, &data));
    println!("Variance: {}", variance(&data));
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
    let line = plug(slope, avg(&x), avg(&data), &x);
    println!("{:?}",line);
    let plot_two: Plot = Plot::new(line).point_style(
        PointStyle::new()
            .colour("#0000ff")
    );
    let view = ContinuousView::new()
        .add(plot_one)
        .add(plot_two)
        .x_label("X-Axis")
        .y_label("Data");
    Page::single(&view).save("plot.svg").unwrap();
}