#![allow(clippy::result_unit_err)]

use plotters::prelude::*;
use std::path::{Path, PathBuf};

use crate::{data::DataSet, utils::save_contents_to_file};

pub struct LinearRegression {
    pub data: DataSet,
    pub m: usize,
    alpha: f64,
    theta_zero: f64,
    theta_one: f64,
    gradient_descent_iterations: usize,
    graph_path: PathBuf,
}

impl LinearRegression {
    pub fn new(
        data: DataSet,
        alpha: Option<f64>,
        gradient_descent_iterations: usize,
        graph_path: PathBuf,
    ) -> Self {
        Self {
            m: data.len(),
            data,
            alpha: if let Some(alpha) = alpha {
                alpha
            } else {
                0.00000000001
            },
            theta_zero: 10000.0,
            theta_one: -0.055,
            gradient_descent_iterations,
            graph_path,
        }
    }

    pub fn save_thetas_to_file(&self, path: &Path) -> Result<(), ()> {
        let file_contents = format!(
            "theta_zero: {}\ntheta_one: {}",
            self.theta_zero, self.theta_one
        );
        save_contents_to_file(path, &file_contents)?;
        Ok(())
    }

    pub fn set_thetas(&mut self, theta_zero: f64, theta_one: f64) {
        self.theta_zero = theta_zero;
        self.theta_one = theta_one;
    }

    pub fn predict(&self, x: f64) -> f64 {
        let res = self.hypothesis(x);
        tracing::info!("x: {x}, hypothesis: {}", res);
        res
    }

    pub fn train(&mut self) {
        self.gradient_descent();
    }

    fn hypothesis(&self, x: f64) -> f64 {
        self.theta_zero + self.theta_one * x
    }

    fn cost(&mut self, is_theta_zero: bool) -> f64 {
        let mut sum = 0.0;
        for record in self.data.clone() {
            let (x, y) = (record.km, record.price);
            let x_hypothesis = self.hypothesis(x);
            if is_theta_zero {
                sum += x_hypothesis - y;
            } else {
                sum += (x_hypothesis - y) * x;
            }
        }
        (1.0 / (self.m as f64)) * sum
    }

    fn gradient_descent(&mut self) {
        for i in 0..self.gradient_descent_iterations {
            let temp_zero = self.theta_zero - self.alpha * self.cost(true);
            let temp_one = self.theta_one - self.alpha * self.cost(false);
            self.theta_zero = temp_zero;
            self.theta_one = temp_one;
            tracing::info!(
                "iteration: {i}\ntheta_zero: {}\ntheta_one: {}",
                self.theta_zero,
                self.theta_one
            );
        }
        tracing::info!(
            "theta_zero_final_value: {}\ntheta_one_final_value: {}",
            self.theta_zero,
            self.theta_one
        )
    }

    pub fn plot_data(&self) -> Result<(), ()> {
        let root = BitMapBackend::new(&self.graph_path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| tracing::error!("Error filling plot: {e}"))?;

        let mut chart = ChartBuilder::on(&root)
            .caption("x=km, y=price", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(100)
            .y_label_area_size(100)
            .build_cartesian_2d(
                self.data.get_min_value(true)..(self.data.get_max_value(true) * 1.2),
                self.data.get_min_value(false)..(self.data.get_max_value(false) * 1.2),
            )
            .map_err(|e| tracing::error!("Error setting plot data boundaries: {e}"))?;

        chart
            .configure_mesh()
            .draw()
            .map_err(|e| tracing::error!("Error drawing to plot: {e}"))?;

        chart
            .draw_series(PointSeries::of_element(
                self.data.get_vector_of_tuples(),
                2,
                RED,
                &|c, s, st| {
					#[allow(clippy::needless_return)]
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ))
            .map_err(|e| tracing::error!("Error: {e}"))?;

        chart
            .draw_series(LineSeries::new(
                self.data
                    .get_vector_of_tuples()
                    .iter()
                    .map(|(x, _)| (*x, self.predict(*x))),
                BLUE,
            ))
            .map_err(|e| tracing::error!("Error: {e}"))?;
        root.present()
            .map_err(|e| tracing::error!("Error printing graph: {e}"))?;

        Ok(())
    }
}
