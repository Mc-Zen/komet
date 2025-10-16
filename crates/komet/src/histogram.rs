pub fn histogram(values: &[f64], edges: &[f64]) -> Vec<u64> {
    let mut counts = vec![0u64; edges.len() - 1];
    for &value in values {
        if value < edges[0] {
            continue;
        }

        for (i, &edge) in edges[1..].iter().enumerate() {
            if value < edge || (value == edge && i == edges.len() - 2) {
                counts[i] += 1;
                break;
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram() {
        assert_eq!(histogram(&[2., 4., 4.], &[1., 3., 5.]), [1, 2]);
    }

    #[test]
    fn test_upper_edge() {
        assert_eq!(histogram(&[0., 2., 4.], &[0., 2., 4.]), [1, 2]);
    }
    #[test]
    fn test_different_lengths() {
        assert_eq!(
            histogram(&[1.0, 2.0, 3.0], &[1.0, 1.666, 2.3333, 3.0]),
            [1, 1, 1]
        );
    }
}
