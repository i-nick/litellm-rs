//! Helper functions for metrics calculations

use std::collections::VecDeque;

/// Calculate percentile from sorted values
pub(super) fn calculate_percentile(sorted_values: &[f64], percentile: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }

    if percentile >= 1.0 {
        // Safe: we checked is_empty() above
        return sorted_values.last().copied().unwrap_or(0.0);
    }

    let index = percentile * (sorted_values.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = (index.ceil() as usize).min(sorted_values.len() - 1);

    if lower == upper || lower >= sorted_values.len() {
        sorted_values.get(lower).copied().unwrap_or(0.0)
    } else {
        let weight = index - lower as f64;
        let lower_val = sorted_values.get(lower).copied().unwrap_or(0.0);
        let upper_val = sorted_values.get(upper).copied().unwrap_or(0.0);
        lower_val * (1.0 - weight) + upper_val * weight
    }
}

/// Calculate average of f64 values from any iterable
pub(super) fn calculate_average(values: &VecDeque<f64>) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

/// Calculate average of u64 values from any iterable
pub(super) fn calculate_average_u64(values: &VecDeque<u64>) -> u64 {
    if values.is_empty() {
        0
    } else {
        values.iter().sum::<u64>() / values.len() as u64
    }
}

/// Calculate average of u32 values from any iterable
pub(super) fn calculate_average_u32(values: &VecDeque<u32>) -> u32 {
    if values.is_empty() {
        0
    } else {
        values.iter().sum::<u32>() / values.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== calculate_percentile Tests ====================

    #[test]
    fn test_calculate_percentile_empty() {
        let values: Vec<f64> = vec![];
        assert_eq!(calculate_percentile(&values, 0.5), 0.0);
    }

    #[test]
    fn test_calculate_percentile_single_value() {
        let values = vec![42.0];
        assert_eq!(calculate_percentile(&values, 0.0), 42.0);
        assert_eq!(calculate_percentile(&values, 0.5), 42.0);
        assert_eq!(calculate_percentile(&values, 1.0), 42.0);
    }

    #[test]
    fn test_calculate_percentile_two_values() {
        let values = vec![10.0, 20.0];
        assert_eq!(calculate_percentile(&values, 0.0), 10.0);
        assert_eq!(calculate_percentile(&values, 0.5), 15.0);
        assert_eq!(calculate_percentile(&values, 1.0), 20.0);
    }

    #[test]
    fn test_calculate_percentile_p50() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(calculate_percentile(&values, 0.5), 3.0);
    }

    #[test]
    fn test_calculate_percentile_p0() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        assert_eq!(calculate_percentile(&values, 0.0), 10.0);
    }

    #[test]
    fn test_calculate_percentile_p100() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        assert_eq!(calculate_percentile(&values, 1.0), 50.0);
    }

    #[test]
    fn test_calculate_percentile_p95() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let p95 = calculate_percentile(&values, 0.95);
        // P95 of 1-100 should be around 95.05
        assert!((p95 - 95.05).abs() < 0.1);
    }

    #[test]
    fn test_calculate_percentile_p99() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let p99 = calculate_percentile(&values, 0.99);
        // P99 of 1-100 should be around 99.01
        assert!((p99 - 99.01).abs() < 0.1);
    }

    #[test]
    fn test_calculate_percentile_interpolation() {
        let values = vec![0.0, 10.0];
        // 25th percentile should be 2.5 (interpolated)
        let p25 = calculate_percentile(&values, 0.25);
        assert!((p25 - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_calculate_percentile_above_one() {
        let values = vec![1.0, 2.0, 3.0];
        // Percentile >= 1.0 should return last value
        assert_eq!(calculate_percentile(&values, 1.5), 3.0);
        assert_eq!(calculate_percentile(&values, 10.0), 3.0);
    }

    #[test]
    fn test_calculate_percentile_identical_values() {
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        assert_eq!(calculate_percentile(&values, 0.0), 5.0);
        assert_eq!(calculate_percentile(&values, 0.5), 5.0);
        assert_eq!(calculate_percentile(&values, 1.0), 5.0);
    }

    #[test]
    fn test_calculate_percentile_large_values() {
        let values = vec![1e10, 2e10, 3e10];
        assert_eq!(calculate_percentile(&values, 0.5), 2e10);
    }

    #[test]
    fn test_calculate_percentile_small_values() {
        let values = vec![0.001, 0.002, 0.003];
        let p50 = calculate_percentile(&values, 0.5);
        assert!((p50 - 0.002).abs() < 0.0001);
    }

    // ==================== calculate_average Tests ====================

    #[test]
    fn test_calculate_average_empty() {
        let values: VecDeque<f64> = VecDeque::new();
        assert_eq!(calculate_average(&values), 0.0);
    }

    #[test]
    fn test_calculate_average_single_value() {
        let values: VecDeque<f64> = VecDeque::from([42.0]);
        assert_eq!(calculate_average(&values), 42.0);
    }

    #[test]
    fn test_calculate_average_multiple_values() {
        let values: VecDeque<f64> = VecDeque::from([10.0, 20.0, 30.0]);
        assert_eq!(calculate_average(&values), 20.0);
    }

    #[test]
    fn test_calculate_average_with_decimals() {
        let values: VecDeque<f64> = VecDeque::from([1.5, 2.5, 3.0]);
        let avg = calculate_average(&values);
        assert!((avg - 2.333333).abs() < 0.001);
    }

    #[test]
    fn test_calculate_average_large_values() {
        let values: VecDeque<f64> = VecDeque::from([1e10, 2e10, 3e10]);
        assert_eq!(calculate_average(&values), 2e10);
    }

    #[test]
    fn test_calculate_average_with_negatives() {
        let values: VecDeque<f64> = VecDeque::from([-10.0, 0.0, 10.0]);
        assert_eq!(calculate_average(&values), 0.0);
    }

    #[test]
    fn test_calculate_average_identical_values() {
        let values: VecDeque<f64> = VecDeque::from([5.0, 5.0, 5.0, 5.0]);
        assert_eq!(calculate_average(&values), 5.0);
    }

    // ==================== calculate_average_u64 Tests ====================

    #[test]
    fn test_calculate_average_u64_empty() {
        let values: VecDeque<u64> = VecDeque::new();
        assert_eq!(calculate_average_u64(&values), 0);
    }

    #[test]
    fn test_calculate_average_u64_single_value() {
        let values: VecDeque<u64> = VecDeque::from([100]);
        assert_eq!(calculate_average_u64(&values), 100);
    }

    #[test]
    fn test_calculate_average_u64_multiple_values() {
        let values: VecDeque<u64> = VecDeque::from([10, 20, 30]);
        assert_eq!(calculate_average_u64(&values), 20);
    }

    #[test]
    fn test_calculate_average_u64_rounds_down() {
        // Integer division rounds down
        let values: VecDeque<u64> = VecDeque::from([1, 2]);
        assert_eq!(calculate_average_u64(&values), 1); // (1+2)/2 = 1 (integer division)
    }

    #[test]
    fn test_calculate_average_u64_large_values() {
        let values: VecDeque<u64> = VecDeque::from([1_000_000, 2_000_000, 3_000_000]);
        assert_eq!(calculate_average_u64(&values), 2_000_000);
    }

    #[test]
    fn test_calculate_average_u64_with_zero() {
        let values: VecDeque<u64> = VecDeque::from([0, 100, 200]);
        assert_eq!(calculate_average_u64(&values), 100);
    }

    // ==================== calculate_average_u32 Tests ====================

    #[test]
    fn test_calculate_average_u32_empty() {
        let values: VecDeque<u32> = VecDeque::new();
        assert_eq!(calculate_average_u32(&values), 0);
    }

    #[test]
    fn test_calculate_average_u32_single_value() {
        let values: VecDeque<u32> = VecDeque::from([50]);
        assert_eq!(calculate_average_u32(&values), 50);
    }

    #[test]
    fn test_calculate_average_u32_multiple_values() {
        let values: VecDeque<u32> = VecDeque::from([100, 200, 300]);
        assert_eq!(calculate_average_u32(&values), 200);
    }

    #[test]
    fn test_calculate_average_u32_rounds_down() {
        let values: VecDeque<u32> = VecDeque::from([1, 2, 3]);
        assert_eq!(calculate_average_u32(&values), 2);
    }

    #[test]
    fn test_calculate_average_u32_all_zeros() {
        let values: VecDeque<u32> = VecDeque::from([0, 0, 0]);
        assert_eq!(calculate_average_u32(&values), 0);
    }

    #[test]
    fn test_calculate_average_u32_large_values() {
        let values: VecDeque<u32> = VecDeque::from([1000, 2000, 3000]);
        assert_eq!(calculate_average_u32(&values), 2000);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_calculate_percentile_negative_values() {
        let values = vec![-30.0, -20.0, -10.0, 0.0, 10.0];
        assert_eq!(calculate_percentile(&values, 0.5), -10.0);
    }

    #[test]
    fn test_calculate_average_very_small_difference() {
        let values: VecDeque<f64> = VecDeque::from([1.0000001, 1.0000002, 1.0000003]);
        let avg = calculate_average(&values);
        assert!((avg - 1.0000002).abs() < 1e-7);
    }

    #[test]
    fn test_calculate_percentile_p25_p75() {
        let values: Vec<f64> = (0..100).map(|x| x as f64).collect();
        let p25 = calculate_percentile(&values, 0.25);
        let p75 = calculate_percentile(&values, 0.75);

        // P25 should be around 24.75, P75 around 74.25
        assert!((p25 - 24.75).abs() < 0.1);
        assert!((p75 - 74.25).abs() < 0.1);
    }
}
