use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub struct KdeResult {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub bandwidth: f64,
}

fn gaussian_kernel(x: f64) -> f64 {
    (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp()
}

fn scotts_bandwidth(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let std_dev = standard_deviation(data);
    std_dev * n.powf(-1.0 / 5.0)
}

fn standard_deviation(data: &[f64]) -> f64 {
    let n = data.len();
    if n <= 1 {
        return 0.0;
    }
    let mean = data.iter().sum::<f64>() / n as f64;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    variance.sqrt()
}

pub fn kde(
    data: &[f64],
    bandwidth: Option<f64>,
    num_points: usize,
    x_min: Option<f64>,
    x_max: Option<f64>,
) -> KdeResult {
    assert!(!data.is_empty(), "Data cannot be empty");

    let bw = bandwidth.unwrap_or_else(|| scotts_bandwidth(data));

    let data_min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let data_max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let x_min = x_min.unwrap_or(data_min - 3.0 * bw);
    let x_max = x_max.unwrap_or(data_max + 3.0 * bw);

    let step = (x_max - x_min) / (num_points - 1) as f64;
    let x: Vec<f64> = (0..num_points).map(|i| x_min + step * i as f64).collect();

    let n = data.len() as f64;
    let y: Vec<f64> = x
        .iter()
        .map(|&xi| {
            let density = data
                .iter()
                .map(|&di| gaussian_kernel((xi - di) / bw))
                .sum::<f64>();
            density / (n * bw)
        })
        .collect();

    KdeResult {
        x,
        y,
        bandwidth: bw,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_kernel() {
        // At x=0, Gaussian kernel should be 1/sqrt(2*pi)
        let result = gaussian_kernel(0.0);
        let expected = 1.0 / (2.0 * PI).sqrt();
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_scotts_bandwidth() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let bw = scotts_bandwidth(&data);
        // Should return a positive value
        assert!(bw > 0.0);
    }

    #[test]
    fn test_kde_basic() {
        let data = vec![1.0, 2.0, 3.0];
        let result = kde(&data, Some(0.5), 50, None, None);

        // Check that we got the right number of points
        assert_eq!(result.x.len(), 50);
        assert_eq!(result.y.len(), 50);

        // Check that x values are sorted
        for i in 1..result.x.len() {
            assert!(result.x[i] > result.x[i - 1]);
        }

        // Check that density values are non-negative
        for &y in &result.y {
            assert!(y >= 0.0);
        }

        // Density should integrate to approximately 1
        let step = result.x[1] - result.x[0];
        let integral: f64 = result.y.iter().sum::<f64>() * step;
        assert!((integral - 1.0).abs() < 0.1); // Allow some tolerance
    }

    #[test]
    fn test_kde_with_custom_range() {
        let data = vec![5.0, 5.5, 6.0];
        let result = kde(&data, Some(0.3), 20, Some(4.0), Some(7.0));

        assert_eq!(result.x.len(), 20);
        assert_eq!(result.y.len(), 20);
        assert!((result.x[0] - 4.0).abs() < 1e-10);
        assert!((result.x[19] - 7.0).abs() < 1e-10);
        assert!((result.bandwidth - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_scotts_bandwidth_matches_scipy() {
        // Verify that our Scott's bandwidth matches scipy's calculation
        // scipy uses: n^(-1/5) * std(data, ddof=1)
        let data = vec![1.0, 2.0, 3.0];
        let bw = scotts_bandwidth(&data);

        // Expected: std(ddof=1) * n^(-1/5) = 1.0 * 3^(-0.2) = 0.8027415617602307
        let expected = 0.8027415617602307;
        assert!(
            (bw - expected).abs() < 1e-10,
            "Bandwidth mismatch: got {}, expected {}",
            bw,
            expected
        );
    }

    #[test]
    fn test_kde_matches_scipy_output() {
        // Verify KDE output matches scipy.stats.gaussian_kde with Scott's rule
        let data = vec![1.0, 2.0, 3.0];

        // Use the same x values that scipy would generate
        let x = vec![
            -0.9663072216374342,
            -0.30712783905133767,
            0.35205154353475887,
            1.0112309261208554,
            1.670410308706952,
            2.3295896912930485,
            2.988769073879145,
            3.647948456465242,
            4.307127839051338,
            4.966307221637434,
        ];

        // Expected y values from scipy.stats.gaussian_kde(data, bw_method="scott")(x)
        let y_expected = vec![
            0.008428, 0.04669851, 0.14046044, 0.25092226, 0.31117639, 0.31117639, 0.25092226,
            0.14046044, 0.04669851, 0.008428,
        ];

        // Compute with auto bandwidth (Scott's rule)
        let result = kde(&data, None, 10, Some(x[0]), Some(x[9]));

        // Check bandwidth matches expected
        let expected_bw = 0.8027415617602307;
        assert!(
            (result.bandwidth - expected_bw).abs() < 1e-10,
            "Bandwidth mismatch: got {}, expected {}",
            result.bandwidth,
            expected_bw
        );

        // Check y values match scipy output
        for (i, (&y_got, &y_exp)) in result.y.iter().zip(y_expected.iter()).enumerate() {
            assert!(
                (y_got - y_exp).abs() < 1e-6,
                "Y value mismatch at index {}: got {}, expected {}",
                i,
                y_got,
                y_exp
            );
        }
    }
}
