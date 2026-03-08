struct Vec2(f64, f64);

enum LinearRegressionError {
    EmptyList,
    UndefinedGradient,
}

struct Line {
    gradient: f64,
    y_intercept: f64,
}

impl Line {
    fn format_as_equation(&self) -> String {
        format!("y = {}x + {}", self.gradient, self.y_intercept)
    }
}

impl<T> From<(T, T)> for Vec2
where T: Into<f64>
{
    fn from((x, y): (T, T)) -> Self {
        Vec2(x.into(), y.into())
    }
}

fn calculate_linear_regression(points: &[Vec2]) -> Result<Line, LinearRegressionError> {
    let number_of_points = points.len();
    if number_of_points == 0 {
        return Err(LinearRegressionError::EmptyList);
    }

    let mean_x: f64 = points.iter().map(|Vec2(x, _)| x).sum::<f64>() / number_of_points as f64;

    let denominator: f64 = points.iter()
        .map(|Vec2(x, _)| (x - mean_x).powi(2))
        .sum();
    if denominator == 0f64 {
        return Err(LinearRegressionError::UndefinedGradient);
    }

    let mean_y: f64 = points.iter().map(|Vec2(_, y)| y).sum::<f64>() / number_of_points as f64;
    let numerator: f64 = points.iter()
        .map(|Vec2(x, y)| (x - mean_x) * (y - mean_y))
        .sum();

    let gradient = numerator / denominator;

    Ok(Line {
        gradient,
        y_intercept: mean_y - gradient * mean_x,
    })
}

fn main() {
    let points = [(2, 1425), (5, 1014), (8, 900), (11, 548), (14, 381), (17, 274), (19, 248), (21, 181), (23, 26), (25, 16)]
        .map(Vec2::from);
    match calculate_linear_regression(&points) {
        Ok(res)  => println!("{}", res.format_as_equation()),
        Err(err) => match err {
            LinearRegressionError::EmptyList         => println!("No points provided."),
            LinearRegressionError::UndefinedGradient => println!("Gradient is undefined (vertical regression line)."),
        },
    }
}
