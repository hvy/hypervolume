// TODO: Reduce heap allocations.

pub fn compute(pts: &[Vec<f64>], ref_pt: &[f64]) -> f64 {
    if ref_pt.is_empty() {
        panic!("Reference point must have at least one dimension");
    }
    get_hypervolume_recursive(pts, ref_pt)
}

fn get_hypervolume_recursive(pts: &[Vec<f64>], ref_pt: &[f64]) -> f64 {
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

fn get_hypervolume_two_points(pt0: &[f64], pt1: &[f64]) -> f64 {
    assert_eq!(pt0.len(), pt1.len());
    assert!(!pt0.is_empty());

    pt0.iter()
        .zip(pt1.iter())
        .fold(1.0, |prod, (&crd0, &crd1)| prod * (crd0 - crd1).abs())
}

fn get_max_coordinates(pt0: &[f64], pt1: &[f64]) -> Vec<f64> {
    assert_eq!(pt0.len(), pt1.len());
    assert!(!pt0.is_empty());

    pt0.iter()
        .zip(pt1.iter())
        .map(|(&crd0, &crd1)| crd0.max(crd1))
        .collect()
}

fn get_exclusive_hypervolume(pt: &[f64], pts: &[Vec<f64>], ref_pt: &[f64]) -> f64 {
    let mut limited_pts: Vec<Vec<f64>> = Vec::new();

    if !pts.is_empty() {
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
