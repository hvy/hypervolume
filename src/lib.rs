// TODO: Error handling.
// TODO: Reduce heap allocations.

fn get_hypervolume(pts: &Vec<Vec<f64>>, ref_pt: &Vec<f64>) -> f64 {
    get_hypervolume_recursive(pts, ref_pt)
}

fn get_hypervolume_recursive(pts: &[Vec<f64>], ref_pt: &Vec<f64>) -> f64 {
    match pts.len() {
        1 => get_hypervolume_two_points(&pts[0], &ref_pt),
        2 => {
            get_hypervolume_two_points(&pts[0], &ref_pt)
                + get_hypervolume_two_points(&pts[1], &ref_pt)
                - get_hypervolume_two_points(&get_max_coordinates(&pts[0], &pts[1]), &ref_pt)
        }
        _ => {
            // get_exclusive_hypervolume depends on the points being sorted by the first dimension.
            let mut pts = pts.to_vec();
            pts.sort_by(|pt0, pt1| pt0[0].partial_cmp(&pt1[0]).unwrap());

            pts.iter()
                .enumerate()
                .map(|(i, pt)| get_exclusive_hypervolume(pt, &pts[i + 1..], ref_pt))
                .sum()
        }
    }
}

fn get_hypervolume_two_points(pt0: &Vec<f64>, pt1: &Vec<f64>) -> f64 {
    assert_eq!(pt0.len(), pt1.len());
    assert!(!pt0.is_empty());

    pt0.iter()
        .zip(pt1.iter())
        .fold(1.0, |prod, (&crd0, &crd1)| prod * (crd0 - crd1).abs())
}

fn get_max_coordinates(pt0: &Vec<f64>, pt1: &Vec<f64>) -> Vec<f64> {
    assert_eq!(pt0.len(), pt1.len());
    assert!(!pt0.is_empty());

    pt0.iter()
        .zip(pt1.iter())
        .map(|(&crd0, &crd1)| crd0.max(crd1))
        .collect()
}

fn get_exclusive_hypervolume(pt: &Vec<f64>, pts: &[Vec<f64>], ref_pt: &Vec<f64>) -> f64 {
    let mut limited_pts: Vec<Vec<f64>> = Vec::new();

    if pts.len() > 0 {
        let intersection_pts: Vec<Vec<f64>> = (0..pts.len())
            .map(|i| get_max_coordinates(&pts[i], &pt))
            .collect();

        limited_pts.push(intersection_pts[0].to_vec());

        // Assert that `pts` is sorted by the first dimension.
        let mut left = 0;
        let mut right = 1;

        while right < pts.len() {
            if intersection_pts[left]
                .iter()
                .zip(intersection_pts[right].iter())
                .any(|(&crd0, &crd1)| crd0 > crd1)
            {
                left = right;
                limited_pts.push(intersection_pts[left].to_vec());
            }
            right += 1;
        }
    }

    get_hypervolume_two_points(pt, ref_pt)
        - match limited_pts.len() {
            0 => 0.0,
            1 => get_hypervolume_two_points(&limited_pts[0], ref_pt),
            _ => get_hypervolume_recursive(&limited_pts, ref_pt),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_float_close(v0: f64, v1: f64) -> bool {
        let rtol: f64 = 1e-5;
        let atol: f64 = 1e-8;
        (v0 - v1).abs() <= atol + rtol * v1.abs()
    }

    #[test]
    fn test_from_csv() {
        let filename = "tests/test.csv";

        let contents = std::fs::read_to_string(filename).unwrap();

        for (i, line) in contents.lines().enumerate() {
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
            let ref_pt = ref_pt_str
                .trim_start_matches("[")
                .trim_end_matches("]")
                .split(",")
                .map(|cor| cor.parse().unwrap())
                .collect();

            // Parse hypervolume.
            let hv: f64 = hv_str.parse().unwrap();

            let hv_computed = get_hypervolume(&pts, &ref_pt);

            assert!(
                is_float_close(hv, hv_computed),
                "Computed: {}, Expected: {}, Err: {}",
                hv_computed,
                hv,
                hv - hv_computed
            );
        }
    }
}
