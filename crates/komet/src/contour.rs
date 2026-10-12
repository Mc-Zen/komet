fn min(values: &[f64]) -> f64 {
    values.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}
fn max(values: &[f64]) -> f64 {
    values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
use std::ops::{Add, Sub};

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

/// Computes the intersection of the line between two points with a plane $z=z$.
/// Returns the point of intersection, and none, if there is no intersection
/// between the two given points.
fn intersect_z_plane(p1: &Point, p2: &Point, z: f64) -> Option<Point> {
    let diff = *p2 - *p1;
    if diff.z == 0.0 {
        None
    } else {
        let ratio = (z - p1.z) / diff.z;
        if !(0.0..=1.0).contains(&ratio) {
            None
        } else {
            Some(Point {
                x: p1.x + ratio * diff.x,
                y: p1.y + ratio * diff.y,
                z,
            })
        }
    }
}

/// Computes the intersection case 0..15
fn compute_case(zs: &[f64; 4], level: f64) -> usize {
    let case = zs
        .iter()
        .fold(0, |acc, &z| (acc << 1) | if z > level { 1 } else { 0 });
    if case != 0 || zs.iter().filter(|&&z| z == level).count() != 2 {
        return case;
    }
    zs.iter()
        .fold(0, |acc, &z| (acc << 1) | if z >= level { 1 } else { 0 })
}

fn compute_block_intersection(
    corners: &[Point; 4],
    level: f64,
    x_eps: f64,
    z_eps: f64,
) -> Vec<(Point, Point)> {
    let get_edge_intersection = |edge| -> Point {
        let mut indices = [edge, (edge + 1) & 0b11];
        if edge < 2 {
            indices.reverse();
        }
        let o: Point = corners[indices[0]];
        let p: Point = corners[indices[1]];
        intersect_z_plane(&o, &p, level).unwrap()
    };

    let get_line_segment = |edge1, edge2| -> Vec<(Point, Point)> {
        vec![(get_edge_intersection(edge2), get_edge_intersection(edge1))]
    };

    let case = compute_case(&corners.map(|p| p.z), level);

    match case {
        0 => Vec::new(),
        1 => get_line_segment(3, 2),
        2 => get_line_segment(2, 1),
        3 => get_line_segment(3, 1),
        4 => get_line_segment(1, 0),
        5 => {
            let average_z = 0.25 * corners.map(|p| p.z).iter().sum::<f64>();
            if (average_z - level).abs() < z_eps {
                let is = [0, 1, 2, 3].map(get_edge_intersection);
                let center = Point::new(is[0].x, is[1].y, 0.);
                let offset = Point::new(x_eps, 0., 0.);

                let center1 = center + offset;
                let center2 = center - offset;
                vec![
                    (is[0], center2),
                    (center2, is[3]),
                    (is[2], center1),
                    (center1, is[1]),
                ]
            } else if average_z > level {
                [get_line_segment(3, 0), get_line_segment(1, 2)].concat()
            } else {
                [get_line_segment(1, 0), get_line_segment(3, 2)].concat()
            }
        }
        6 => get_line_segment(2, 0),
        7 => get_line_segment(3, 0),
        8 => get_line_segment(0, 3),
        9 => get_line_segment(0, 2),
        10 => {
            let average_z = 0.25 * corners.map(|p| p.z).iter().sum::<f64>();
            if (average_z - level).abs() < z_eps {
                let is = [0, 1, 2, 3].map(get_edge_intersection);
                let center = Point::new(is[0].x, is[1].y, 0.);
                let offset = Point::new(x_eps, 0., 0.);

                let center1 = center - offset;
                let center2 = center + offset;
                vec![
                    (is[1], center2),
                    (center2, is[0]),
                    (is[3], center1),
                    (center1, is[2]),
                ]
            } else if average_z > level {
                [get_line_segment(0, 1), get_line_segment(2, 3)].concat()
            } else {
                [get_line_segment(0, 3), get_line_segment(2, 1)].concat()
            }
        }
        11 => get_line_segment(0, 1),
        12 => get_line_segment(1, 3),
        13 => get_line_segment(1, 2),
        14 => get_line_segment(2, 3),
        15 => Vec::new(),
        _ => panic!("Impossible"),
    }
}

fn group_segments(mut segments: Vec<(Point, Point)>) -> Vec<Vec<Point>> {
    let mut cycles = Vec::<Vec<Point>>::new();
    while let Some(segment) = segments.pop() {
        let mut cycle = vec![segment.0, segment.1];
        let mut p2 = segment.1;

        while let Some(pos) = segments.iter().position(|seg| seg.0 == p2) {
            p2 = segments.remove(pos).1;
            cycle.push(p2);
        }
        cycle.reverse();
        let mut p1 = segment.0;

        while let Some(pos) = segments.iter().position(|seg| seg.1 == p1) {
            p1 = segments.remove(pos).0;
            cycle.push(p1);
        }
        cycles.push(cycle);
    }
    cycles
}

pub type ContourLine = Vec<Point>; // A single contour line.

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LabelPosition {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contour {
    pub lines: Vec<ContourLine>,
    pub label: Option<LabelPosition>,
}

fn point_at_distance(line: &[Point], distances: &[f64], distance: f64) -> Point {
    if distance <= 0.0 {
        return line[0];
    }
    if distance >= *distances.last().unwrap() {
        return *line.last().unwrap();
    }

    let end = distances.partition_point(|&d| d < distance);
    let start = end - 1;
    let ratio = (distance - distances[start]) / (distances[end] - distances[start]);
    line[start]
        + Point::new(
            ratio * (line[end].x - line[start].x),
            ratio * (line[end].y - line[start].y),
            ratio * (line[end].z - line[start].z),
        )
}

fn slice_line(line: &[Point], distances: &[f64], start: f64, end: f64) -> Vec<Point> {
    if end <= start {
        return Vec::new();
    }

    let mut points = vec![point_at_distance(line, distances, start)];
    for (point, &distance) in line.iter().zip(distances) {
        if start < distance && distance < end {
            points.push(*point);
        }
    }
    points.push(point_at_distance(line, distances, end));
    if points.len() >= 2 {
        points
    } else {
        Vec::new()
    }
}

fn find_label_position(
    lines: &[ContourLine],
    label_length: f64,
    previous_labels: &[LabelPosition],
) -> Option<(usize, f64, f64, LabelPosition)> {
    let label_length = label_length.abs();
    if !label_length.is_finite() || label_length == 0.0 {
        return None;
    }
    let half_length = label_length / 2.0;
    let min_distance_squared = (1.2 * label_length).powi(2);
    let mut best: Option<(usize, f64, f64, f64, LabelPosition)> = None;

    for (line_index, line) in lines.iter().enumerate() {
        if line.len() < 2 {
            continue;
        }

        let mut distances = Vec::with_capacity(line.len());
        distances.push(0.0);
        for points in line.windows(2) {
            let dx = points[1].x - points[0].x;
            let dy = points[1].y - points[0].y;
            distances.push(distances.last().unwrap() + dx.hypot(dy));
        }
        let total_length = *distances.last().unwrap();
        if total_length < label_length || total_length == 0.0 {
            continue;
        }

        let first_center = half_length;
        let last_center = total_length - half_length;
        let mut centers = vec![(first_center + last_center) / 2.0];
        for distance in &distances {
            centers.push(distance.clamp(first_center, last_center));
        }
        for pair in distances.windows(2) {
            centers.push(((pair[0] + pair[1]) / 2.0).clamp(first_center, last_center));
        }

        for center in centers {
            let start = center - half_length;
            let end = center + half_length;
            let start_point = point_at_distance(line, &distances, start);
            let end_point = point_at_distance(line, &distances, end);
            let chord_length = (end_point.x - start_point.x).hypot(end_point.y - start_point.y);
            let straightness = chord_length / label_length;
            let angle = (end_point.y - start_point.y)
                .atan2(end_point.x - start_point.x)
                .to_degrees();
            let angle = if angle > 90.0 {
                angle - 180.0
            } else if angle < -90.0 {
                angle + 180.0
            } else {
                angle
            };
            let center_point = point_at_distance(line, &distances, center);
            let label = LabelPosition {
                x: center_point.x,
                y: center_point.y,
                angle,
            };
            if previous_labels.iter().any(|previous| {
                let dx = label.x - previous.x;
                let dy = label.y - previous.y;
                dx * dx + dy * dy < min_distance_squared
            }) {
                continue;
            }

            let is_better = match &best {
                None => true,
                Some((_, _, _, best_straightness, _)) => straightness > *best_straightness,
            };
            if is_better {
                best = Some((line_index, start, end, straightness, label));
            }
        }
    }

    best.map(|(line_index, start, end, _, label)| (line_index, start, end, label))
}

fn single_contour(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    level: f64,
    z_range: f64,
    label_length: f64,
    previous_labels: &[LabelPosition],
) -> Contour {
    assert_eq!(x.len() * y.len(), z.len());
    let cols = x.len();
    let mut blocks = Vec::<(usize, usize)>::new();

    let x_range = x.last().unwrap() - x.first().unwrap();
    let z_eps = 1e-6 * z_range;
    let x_eps = 1e-17 * x_range;

    for i in 0..x.len() - 1 {
        for j in 0..y.len() - 1 {
            let zs = [
                z[j * cols + i],
                z[j * cols + i + 1],
                z[(j + 1) * cols + i],
                z[(j + 1) * cols + i + 1],
            ];
            if min(&zs) <= level && max(&zs) >= level {
                blocks.push((i, j));
            }
        }
    }
    let offsets = [(0, 0), (1, 0), (1, 1), (0, 1)];

    let segments = blocks
        .iter()
        .map(|(i, j)| {
            compute_block_intersection(
                &offsets.map(|(a, b)| Point::new(x[i + a], y[j + b], z[(j + b) * cols + i + a])),
                level,
                x_eps,
                z_eps,
            )
        })
        .fold(Vec::new(), |mut acc, x| {
            acc.extend(x);
            acc
        });
    let mut lines = group_segments(segments);
    let label = if label_length != 0.0 {
        find_label_position(&lines, label_length, previous_labels).map(
            |(line_index, start, end, label)| {
                let line = lines.remove(line_index);
                let mut distances = Vec::with_capacity(line.len());
                distances.push(0.0);
                for points in line.windows(2) {
                    distances.push(
                        distances.last().unwrap()
                            + (points[1].x - points[0].x).hypot(points[1].y - points[0].y),
                    );
                }
                let before = slice_line(&line, &distances, 0.0, start);
                let after = slice_line(&line, &distances, end, *distances.last().unwrap());
                if !before.is_empty() {
                    lines.push(before);
                }
                if !after.is_empty() {
                    lines.push(after);
                }
                label
            },
        )
    } else {
        None
    };
    Contour { lines, label }
}

/// Generates contours from intersecting a function on a 2d rectangular mesh
/// with a planes parallel to the z-plane at `z=level`.
///
/// The z values should be arranged in a flat array as the following
/// ```rs
/// (
///   z(x0, y0), z(x1, y0), ...,
///   z(x0, y1), z(x1, y1), ...,
///  ...
/// )
/// ```
pub fn contour(
    x: &[f64],
    y: &[f64],
    z: &[f64],
    levels: &[f64],
    label_lengths: &[f64],
) -> Vec<Contour> {
    // assert_eq!(levels.len(), label_lengths.len());
    let mut contours = Vec::with_capacity(levels.len().min(label_lengths.len()));
    let mut previous_labels = Vec::new();
    for (&level, &label_length) in levels.iter().zip(label_lengths.iter()) {
        let result = single_contour(
            x,
            y,
            z,
            level,
            max(z) - min(z),
            label_length,
            &previous_labels,
        );
        if let Some(label) = result.label {
            previous_labels.push(label);
        }
        contours.push(result);
    }
    contours
}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_contour(z: [f64; 4], level: f64) -> Vec<ContourLine> {
        single_contour(&[0., 1.], &[0., 1.], &z, level, max(&z) - min(&z), 0.0, &[]).lines
    }

    #[test]
    fn test_xy_different_sizes() {
        let _contour = single_contour(
            &[0., 1., 2.],
            &[0., 1.],
            &[3., 4., 7., 0., 2., 1.],
            0.4,
            7.,
            0.0,
            &[],
        );
    }

    #[test]
    fn test_contour_label_position() {
        let contour = single_contour(&[0., 4.], &[0., 1.], &[1., 1., -1., -1.], 0., 2., 2., &[]);
        assert_eq!(
            contour.label,
            Some(LabelPosition {
                x: 2.,
                y: 0.5,
                angle: 0.,
            })
        );
        assert_eq!(contour.lines.len(), 2);
        assert!(find_label_position(&contour.lines, 3., &[]).is_none());

        let unlabeled = single_contour(&[0., 4.], &[0., 1.], &[1., 1., -1., -1.], 0., 2., 0., &[]);
        assert_eq!(unlabeled.label, None);
        assert_eq!(unlabeled.lines.len(), 1);

        let too_long = single_contour(&[0., 4.], &[0., 1.], &[1., 1., -1., -1.], 0., 2., 5., &[]);
        assert_eq!(too_long.label, None);
        assert_eq!(too_long.lines.len(), 1);
    }

    #[test]
    fn test_contour_labels_are_spaced_from_previous_labels() {
        let contours = contour(
            &[0., 10.],
            &[0., 1.],
            &[1., 1., -1., -1.],
            &[0., 0., 0., 0.],
            &[2., 2., 2., 2.],
        );
        let labels: Vec<_> = contours
            .iter()
            .filter_map(|contour| contour.label)
            .collect();

        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0].x, 5.);
        for (index, label) in labels.iter().enumerate() {
            for previous in &labels[..index] {
                assert!((label.x - previous.x).hypot(label.y - previous.y) >= 2.4);
            }
        }
        assert_eq!(contours[3].label, None);
    }

    #[test]
    fn test_contour_label_angle() {
        let position = find_label_position(
            &[vec![Point::new(0., 0., 0.), Point::new(3., 4., 0.)]],
            5.,
            &[],
        )
        .unwrap()
        .3;
        assert_eq!(position.x, 1.5);
        assert_eq!(position.y, 2.);
        assert!((position.angle - 53.13010235415598).abs() < 1e-12);
    }

    #[test]
    fn test_intersect_z_plane() {
        assert_eq!(
            intersect_z_plane(&Point::new(0., 0., 0.), &Point::new(2., 0., 1.), 0.2).unwrap(),
            Point::new(0.4, 0., 0.2)
        );
        assert_eq!(
            intersect_z_plane(&Point::new(0., 0., 0.), &Point::new(2., 0., 1.), 1.).unwrap(),
            Point::new(2., 0., 1.)
        );
        assert_eq!(
            intersect_z_plane(&Point::new(0., 0., 0.), &Point::new(2., 0., 1.), 1.1),
            None
        );
        assert_eq!(
            intersect_z_plane(&Point::new(0., 0., 0.), &Point::new(2., 0., 1.), -0.01),
            None
        );
    }

    #[test]
    fn test_contour_no_intersection() {
        // All on level
        assert_eq!(
            debug_contour([0., 0., 0., 0.], 0.),
            Vec::<Vec<Point>>::new()
        );
        // All below level
        assert_eq!(
            debug_contour([20., -20., -20., 20.], 50.),
            Vec::<Vec<Point>>::new()
        );
        // All above level
        assert_eq!(
            debug_contour([20., -20., -20., 20.], -50.),
            Vec::<Vec<Point>>::new()
        );
    }

    #[test]
    fn test_contour_saddles() {
        // Right on
        assert_eq!(
            debug_contour([-1., 1., 1., -1.], 0.),
            vec![
                vec![
                    Point::new(1., 0.5, 0.),
                    Point::new(0.5, 0.5, 0.),
                    Point::new(0.5, 0., 0.)
                ],
                vec![
                    Point::new(0., 0.5, 0.),
                    Point::new(0.5, 0.5, 0.),
                    Point::new(0.5, 1., 0.)
                ]
            ]
        );
        // Slightly above
        assert_eq!(
            debug_contour([-1., 1., 1., -1.], 0.5),
            vec![
                vec![Point::new(0., 0.75, 0.5), Point::new(0.25, 1., 0.5)],
                vec![Point::new(1., 0.25, 0.5), Point::new(0.75, 0., 0.5)]
            ]
        );
        // Slightly below
        assert_eq!(
            debug_contour([-1., 1., 1., -1.], -0.5),
            vec![
                vec![Point::new(1., 0.75, -0.5), Point::new(0.75, 1., -0.5)],
                vec![Point::new(0., 0.25, -0.5), Point::new(0.25, 0., -0.5)]
            ]
        );

        // Right on
        assert_eq!(
            debug_contour([1., -1., -1., 1.], 0.),
            vec![
                vec![
                    Point::new(0.5, 1., 0.),
                    Point::new(0.5, 0.5, 0.),
                    Point::new(1., 0.5, 0.)
                ],
                vec![
                    Point::new(0.5, 0., 0.),
                    Point::new(0.5, 0.5, 0.),
                    Point::new(0., 0.5, 0.)
                ]
            ]
        );
        // Slightly above
        assert_eq!(
            debug_contour([1., -1., -1., 1.], 0.5),
            vec![
                vec![Point::new(0.75, 1., 0.5), Point::new(1., 0.75, 0.5)],
                vec![Point::new(0.25, 0., 0.5), Point::new(0., 0.25, 0.5)]
            ]
        );
        // Slightly below
        assert_eq!(
            debug_contour([1., -1., -1., 1.], -0.5),
            vec![
                vec![Point::new(0.25, 1., -0.5), Point::new(0., 0.75, -0.5)],
                vec![Point::new(0.75, 0., -0.5), Point::new(1., 0.25, -0.5)]
            ]
        );
    }
    #[test]
    fn test_contour_corners() {
        // Bottom to left
        // x   x
        // ←-|
        // o | x  (o above level)
        assert_eq!(
            debug_contour([6., 2., 0., 0.], 3.),
            vec![vec![Point::new(0.75, 0., 3.), Point::new(0., 0.5, 3.)]]
        );
        // Left to bottom
        // o   o
        // --|
        // x ↓ o  (x below level)
        assert_eq!(
            debug_contour([-4., 2., 0., 0.], -1.),
            vec![vec![Point::new(0., 0.75, -1.), Point::new(0.5, 0., -1.),]]
        );
        // Right to bottom
        // x   x
        //   |--
        // x ↓ o
        assert_eq!(
            debug_contour([-4., 4., -1., -1.], 0.),
            vec![vec![Point::new(1., 0.8, 0.), Point::new(0.5, 0., 0.),]]
        );
        // Bottom to right
        // o   o
        //   |-→
        // o | x
        assert_eq!(
            debug_contour([4., -4., 1., 1.], 0.),
            vec![vec![Point::new(0.5, 0., 0.), Point::new(1., 0.8, 0.),]]
        );
        // Top to right
        // x | o
        //   |-→
        // x   x
        assert_eq!(
            debug_contour([-4., -4., -4., 4.], 0.),
            vec![vec![Point::new(0.5, 1., 0.), Point::new(1., 0.5, 0.),]]
        );
        // Right to top
        // o ↑ x
        //   |--
        // o   o
        assert_eq!(
            debug_contour([4., 4., 4., -4.], 0.),
            vec![vec![Point::new(1., 0.5, 0.), Point::new(0.5, 1., 0.),]]
        );
        // Left to top
        // o ↑ x
        // --|
        // x   x
        assert_eq!(
            debug_contour([-4., -4., 4., -4.], 0.),
            vec![vec![Point::new(0., 0.5, 0.), Point::new(0.5, 1., 0.),]]
        );
        // Top to left
        // x | o
        // ←-|
        // o   o
        assert_eq!(
            debug_contour([4., 4., -4., 4.], 0.),
            vec![vec![Point::new(0.5, 1., 0.), Point::new(0., 0.5, 0.),]]
        );
    }

    #[test]
    fn test_contour_opposite() {
        // Left to right
        // o  o
        // ---→
        // x  x
        assert_eq!(
            debug_contour([1., 0., 2., 6.], 1.5),
            vec![vec![Point::new(0., 0.5, 1.5), Point::new(1., 0.25, 1.5)]]
        );
        // Right to left
        // x  x
        // ←---
        // o  o
        assert_eq!(
            debug_contour([2., 6., 1., 0.], 1.5),
            vec![vec![Point::new(1., 0.75, 1.5), Point::new(0., 0.5, 1.5)]]
        );
        // Top to bottom
        // x | o
        //   |
        // x ↓ o
        assert_eq!(
            debug_contour([1., 2., 0., 6.], 1.5),
            vec![vec![Point::new(0.25, 1., 1.5), Point::new(0.5, 0., 1.5)]]
        );
        // Bottom to top
        // o ↑ x
        //   |
        // o | x
        assert_eq!(
            debug_contour([2., 0., 6., 1.], 1.5),
            vec![vec![Point::new(0.25, 0., 1.5), Point::new(0.9, 1., 1.5)]]
        );
    }

    #[test]
    fn test_contour_edge_cases() {
        // top below
        assert_eq!(
            debug_contour([20., -20., -20., 20.], -50.),
            Vec::<Vec<Point>>::new()
        );

        // ·←--·
        //
        // o   o
        assert_eq!(
            debug_contour([3., 4., 0., 0.], 0.),
            vec![vec![Point::new(1., 1., 0.), Point::new(0., 1., 0.)]]
        );

        // ·--→·
        //
        // x   x
        assert_eq!(
            debug_contour([-3., -4., 0., 0.], 0.),
            vec![vec![Point::new(0., 1., 0.), Point::new(1., 1., 0.)]]
        );

        // o   o
        //
        // ·--→·
        assert_eq!(
            debug_contour([0., 0., 3., 4.], 0.),
            vec![vec![Point::new(0., 0., 0.), Point::new(1., 0., 0.)]]
        );

        // x   x
        //
        // ·←--·
        assert_eq!(
            debug_contour([0., 0., -3., -4.], 0.),
            vec![vec![Point::new(1., 0., 0.), Point::new(0., 0., 0.)]]
        );

        // ·   o
        // ↓
        // ·   o
        assert_eq!(
            debug_contour([0., 3., 0., 4.], 0.),
            vec![vec![Point::new(0., 1., 0.), Point::new(0., 0., 0.)]]
        );

        // ·   x
        // ↑
        // ·   x
        assert_eq!(
            debug_contour([0., -3., 0., -4.], 0.),
            vec![vec![Point::new(0., 0., 0.), Point::new(0., 1., 0.)]]
        );

        // o   ·
        //     ↑
        // o   ·
        assert_eq!(
            debug_contour([3., 0., 4., 0.], 0.),
            vec![vec![Point::new(1., 0., 0.), Point::new(1., 1., 0.)]]
        );

        // x   ·
        //     ↓
        // x   ·
        assert_eq!(
            debug_contour([-3., 0., -4., 0.], 0.),
            vec![vec![Point::new(1., 1., 0.), Point::new(1., 0., 0.)]]
        );
    }

    #[test]
    fn test_case_computation() {
        assert_eq!(compute_case(&[-1., -1., -1., -1.], 0.), 0);
        assert_eq!(compute_case(&[1., 1., 1., 1.], 0.), 15);
        assert_eq!(compute_case(&[-1., -1., -1., 1.], 0.), 1);
        assert_eq!(compute_case(&[-1., -1., 1., -1.], 0.), 2);
        assert_eq!(compute_case(&[-1., -1., 1., 1.], 0.), 3);
        assert_eq!(compute_case(&[-1., 1., -1., -1.], 0.), 4);
        assert_eq!(compute_case(&[-1., -1., -1., 1.], 0.), 1);
        assert_eq!(compute_case(&[0., 0., -1., -1.], 0.), 12);
    }
}
