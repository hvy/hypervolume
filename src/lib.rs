// TODO: Document.

mod hypervolume_core;

pub use hypervolume_core::compute;

#[cfg(test)]
mod tests {
    use super::*;

    fn is_float_close(v0: f64, v1: f64) -> bool {
        let rtol: f64 = 1e-5;
        let atol: f64 = 1e-8;
        (v0 - v1).abs() <= atol + rtol * v1.abs()
    }

    #[test]
    fn test_1d_single_point() {
        let pts = vec![vec![0.3]];
        let ref_pt = vec![1.0];
        let hv = 0.7;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_1d_multiple_points() {
        let pts = vec![vec![0.5], vec![0.3], vec![0.2]];
        let ref_pt = vec![1.0];
        let hv = 0.8;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_1d_no_points() {
        let pts = vec![];
        let ref_pt = vec![1.0];
        let hv = 0.0;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_2d_single_point() {
        let pts = vec![vec![0.3, 0.5]];
        let ref_pt = vec![1.0, 1.0];
        let hv = 0.35;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_2d_multiple_points() {
        let mut pts = vec![vec![0.3, 0.5], vec![0.6, 0.2]];
        let ref_pt = vec![1.0, 1.0];
        let hv = 0.47;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));

        // Non-Pareto optimal points do not contribute to the hypervolume.
        pts.push(vec![0.8, 0.7]);
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));

        // Points along the Pareto front does similarly not change the hypervolume.
        pts.push(vec![0.3, 0.8]);
        pts.push(vec![0.9, 0.2]);
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_2d_no_points() {
        let pts = vec![];
        let ref_pt = vec![1.0, 1.0];
        let hv = 0.0;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_3d_single_point() {
        let pts = vec![vec![0.5, 0.5, 0.5]];
        let ref_pt = vec![1.0, 1.0, 1.0];
        let hv = 0.125;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_3d_no_points() {
        let pts = vec![];
        let ref_pt = vec![1.0, 1.0, 1.0];
        let hv = 0.0;
        let hv_computed = compute(&pts, &ref_pt);
        assert!(is_float_close(hv_computed, hv));
    }

    #[test]
    fn test_3d_from_csv() {
        let filename = "tests/3d.csv";
        let contents = std::fs::read_to_string(filename).unwrap();

        for line in contents.lines() {
            let mut words = line.split_whitespace();
            let pts_str = words.next().expect("No points");
            let ref_pt_str = words.next().expect("No reference point");
            let hv_str = words.next().expect("No hypervolume");

            // Parse points.
            let mut pts: Vec<Vec<f64>> = Vec::new();
            let re = regex::Regex::new(r"\[([0-9]*\.?[0-9]+(e[+-]?[0-9]+)?,?)*\]").unwrap();
            for pt_str in re.find_iter(pts_str) {
                pts.push(
                    pt_str
                        .as_str()
                        .trim_start_matches("[")
                        .trim_end_matches("]")
                        .split(",")
                        .map(|cor| cor.parse().unwrap())
                        .collect(),
                );
            }

            // Parse reference points.
            let ref_pt: Vec<f64> = ref_pt_str
                .trim_start_matches("[")
                .trim_end_matches("]")
                .split(",")
                .map(|cor| cor.parse().unwrap())
                .collect();

            // Parse hypervolume.
            let hv: f64 = hv_str.parse().unwrap();

            let hv_computed = compute(&pts, &ref_pt);
            assert!(is_float_close(hv_computed, hv));
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_reference_point() {
        let pts = vec![vec![0.5, 0.5]];
        let ref_pt = vec![];
        let _ = compute(&pts, &ref_pt);
    }
}
