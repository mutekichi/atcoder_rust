#![allow(dead_code)]

/// --- SNAP START ---

/// Geometry Utils: Point and Convex Hull (Monotone Chain)
///
/// # Examples
///
/// ```
/// use atcoder_rust::template::geometry::{Point, convex_hull};
///
/// let points = vec![
///     Point::new(0, 0),
///     Point::new(2, 2),
///     Point::new(0, 2),
///     Point::new(2, 0),
///     Point::new(1, 1), // Inside
/// ];
///
/// let hull = convex_hull(&points);
///
/// // The result will be the 4 corners in counter-clockwise order.
/// // Note: The starting point depends on sorting, usually the bottom-left-most point.
/// assert_eq!(hull.len(), 4);
/// println!("{:?}", hull);
/// ```

/// Represents a point in 2D space.
/// Derives `Ord` to allow sorting (sorts by x, then y).
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Creates a new Point.
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

// Operator overloading for vector arithmetic (optional but useful)
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

/// Calculates the cross product (determinant) of vectors OA and OB.
///
/// # Returns
/// - Positive: O->A->B is a Left Turn (Counter-Clockwise)
/// - Negative: O->A->B is a Right Turn (Clockwise)
/// - Zero: Collinear
fn cross_product(o: Point, a: Point, b: Point) -> i64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

/// Computes the Convex Hull of a set of points using the Monotone Chain algorithm.
///
/// # Arguments
/// - `points`: A slice of `Point`s.
///
/// # Returns
/// - A `Vec<Point>` representing the vertices of the convex hull in **Counter-Clockwise (CCW)** order.
/// - The start point is the lexicographically smallest point (min x, then min y).
///
/// # Notes
/// - This implementation removes collinear points on the edges of the hull (strictly convex).
/// - If you need to keep collinear points, change `<=` to `<` in the while loops.
/// - Time Complexity: O(N log N) due to sorting.
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let n = points.len();
    if n <= 2 {
        return points.to_vec();
    }

    // 1. Sort points lexicographically (x, then y)
    let mut sorted_points = points.to_vec();
    sorted_points.sort();

    // 2. Build the lower hull
    let mut lower = Vec::new();
    for &p in &sorted_points {
        while lower.len() >= 2 {
            let p1 = lower[lower.len() - 2];
            let p2 = lower[lower.len() - 1];
            // If cross_product <= 0, it's a right turn or collinear -> pop
            if cross_product(p1, p2, p) <= 0 {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(p);
    }

    // 3. Build the upper hull
    let mut upper = Vec::new();
    for &p in sorted_points.iter().rev() {
        while upper.len() >= 2 {
            let p1 = upper[upper.len() - 2];
            let p2 = upper[upper.len() - 1];
            // If cross_product <= 0, it's a right turn or collinear -> pop
            if cross_product(p1, p2, p) <= 0 {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(p);
    }

    // 4. Concatenate hulls
    // The last point of lower is the same as the first of upper.
    // The last point of upper is the same as the first of lower.
    // We remove the last point of each to avoid duplication.
    lower.pop();
    upper.pop();
    
    lower.extend(upper);
    lower
}