#![warn(missing_docs)]

//!
//! This module provides the `Polygon` struct and associated methods.
//!
//! [Pyo3](https://github.com/PyO3/pyo3) is used to turn the `Polygon` struct into a Python class.
//!

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Represents a polygon defined by its vertices.
/// Vertices should be provided as a flat array of (x, y) pairs.
/// E.g. `[x1, y1, x2, y2, ...]`
#[pyclass]
struct Polygon {
    #[pyo3(get)]
    vertices: Vec<f64>,
}

impl Polygon {
    /// Validates the vertices of a polygon.
    /// Vertices should contain x, y pairs, so the length should be even.
    /// A polygon should have at least 3 vertices.
    fn validate_vertices(vertices: &Vec<f64>) -> PyResult<()> {
        let n = vertices.len();
        if n % 2 != 0 {
            return Err(PyValueError::new_err(
                "Vertices should be provided as a flat array, e.g. [x1, y1, x2, y2, ...]",
            ));
        }
        if n < 6 {
            return Err(PyValueError::new_err(
                "A polygon should have at least 3 vertices.",
            ));
        }
        Ok(())
    }
}

#[pymethods]
impl Polygon {
    /// Creates a new `Polygon` with the given vertices.
    ///
    /// Arguments
    /// ---------
    /// * `vertices` - A `list` of `float` values representing the vertices of the polygon.
    ///
    /// Returns
    /// -------
    /// * A `Polygon` object or a `ValueError` if the vertices are invalid.
    #[new]
    fn new(vertices: Vec<f64>) -> PyResult<Self> {
        Self::validate_vertices(&vertices)?;
        Ok(Polygon { vertices })
    }

    /// Sets the vertices of the polygon.
    ///
    /// Arguments
    /// ---------
    /// * `vertices` - A `list` of `float` values representing the vertices of the polygon.
    ///
    /// Returns
    /// -------
    /// `None` or a `ValueError` if the vertices are invalid.
    #[setter(vertices)]
    fn set_vertices(&mut self, vertices: Vec<f64>) -> PyResult<()> {
        Self::validate_vertices(&vertices)?;
        self.vertices = vertices;
        Ok(())
    }

    /// Calculates and returns the area of the polygon.
    ///
    /// Returns
    /// -------
    /// A `float` representing the area of the polygon.
    fn area(&self) -> PyResult<f64> {
        Ok(polygon_area_internal(&self.vertices))
    }

    /// Calculates and returns the centroid of the polygon.
    ///
    /// Arguments
    /// ---------
    /// * `area` - An optional `float` representing the area of the polygon. If `None`, the area will be calculated.
    ///
    /// Returns
    /// -------
    /// * A `tuple` of `float` values representing the (x, y) coordinates of the centroid.
    fn centroid(&self, area: Option<f64>) -> PyResult<(f64, f64)> {
        Ok(polygon_centroid_internal(&self.vertices, area))
    }
}

#[pymodule]
fn polyprops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Polygon>()?;
    Ok(())
}

///
/// Computes the centroid of a polygon given its vertices.
///
/// Vertices should be provided as a flat array of (x, y) pairs.
fn polygon_centroid_internal(vertices: &Vec<f64>, area: Option<f64>) -> (f64, f64) {
    ///
    /// Shifts all x-coordinates by dx
    /// Assuming they are in the form [x1, y1, x2, y2, ...]
    fn shift_x(vertices: &mut Vec<f64>, dx: f64) {
        let n = vertices.len();
        for i in 0..(n / 2) {
            vertices[2 * i] += dx;
        }
    }

    ///
    /// Shifts all y-coordinates by dy
    /// Assuming they are in the form [x1, y1, x2, y2, ...]
    fn shift_y(vertices: &mut Vec<f64>, dy: f64) {
        let n = vertices.len();
        for i in 0..(n / 2) {
            vertices[2 * i + 1] += dy;
        }
    }

    ///
    /// Computes the centroid of a polygon given its vertices using the Bourke & Nürnberg method.
    /// Does not allow negative vertices.
    fn centroid_algorithm(vertices: &Vec<f64>, area: Option<f64>) -> Result<(f64, f64), PyErr> {
        // If the area is not provided, compute it.
        let area = match area {
            Some(area) => area,
            None => polygon_area_internal(&vertices),
        };

        // Calculate X and Y coordinates of the centroid using the Bourke & Nürnberg method
        let mut summation_x = 0.0;
        let mut summation_y = 0.0;

        let n = vertices.len();
        for i in 0..(n / 2) {
            let x1 = vertices[2 * i];
            let y1 = vertices[2 * i + 1];
            let x2 = vertices[(2 * i + 2) % n];
            let y2 = vertices[(2 * i + 3) % n];

            // Check if any of the vertices are negative. The Bourke & Nürnberg method does not work
            // for polygons with negative vertices.
            if x1 < 0.0 || x2 < 0.0 || y1 < 0.0 || y2 < 0.0 {
                Err(PyValueError::new_err("Polygon contains negative vertices."))?;
            }

            summation_x += (x1 + x2) * (x1 * y2 - x2 * y1);
            summation_y += (y1 + y2) * (x1 * y2 - x2 * y1);
        }

        let centroid_x = (summation_x / (6.0 * area)).abs();
        let centroid_y = (summation_y / (6.0 * area)).abs();

        Ok((centroid_x, centroid_y))
    }

    // Find the minimum x and y-coordinates of the polygon.
    // If they are negative, shift the polygon by the absolute value of the minimum
    // so that all vertices are positive.

    let mut min_x = vertices[0];
    let mut min_y = vertices[1];

    for i in 0..(vertices.len() / 2) {
        let x = vertices[2 * i];
        let y = vertices[2 * i + 1];
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
    }

    if min_x < 0.0 || min_y < 0.0 {
        let mut shifted_vertices = vertices.to_owned();
        if min_x < 0.0 {
            shift_x(&mut shifted_vertices, min_x.abs());
        }
        if min_y < 0.0 {
            shift_y(&mut shifted_vertices, min_y.abs());
        }

        // Compute the centroid of the shifted polygon. Then shift the centroid back.
        let (x, y) = centroid_algorithm(&shifted_vertices, area).unwrap();
        return (x - min_x.abs(), y - min_y.abs());
    } else {
        return centroid_algorithm(&vertices, area).unwrap();
    }
}

///
/// Computes the area of a polygon given its vertices using the shoelace formula.
///
/// Vertices should be provided as a flat array of (x, y) pairs.
fn polygon_area_internal(vertices: &Vec<f64>) -> f64 {
    // Compute the area using the shoelace formula.
    let mut area = 0.0;
    let n = vertices.len();
    for i in 0..(n / 2) {
        let x1 = vertices[2 * i];
        let y1 = vertices[2 * i + 1];
        let x2 = vertices[(2 * i + 2) % n];
        let y2 = vertices[(2 * i + 3) % n];
        area += x1 * y2 - x2 * y1;
    }
    area.abs() / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Checks if two floats are close enough to be considered equal.
    /// Avoids floating point errors.
    fn isclose(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn test_polygon_area() {
        let vertices = vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
        assert!(isclose(polygon_area_internal(&vertices), 0.5));
    }

    #[test]
    fn test_polygon_centroid() {
        let vertices = vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
        let (x, y) = polygon_centroid_internal(&vertices, None);
        assert!(isclose(x, 2.0 / 3.0));
        assert!(isclose(y, 1.0 / 3.0));
    }

    #[test]
    fn test_polygon_centroid_negative_vertices() {
        let vertices = vec![0.0, 0.0, 1.0, 0.0, 1.0, -1.0];
        let (x, y) = polygon_centroid_internal(&vertices, None);
        assert!(isclose(x, 2.0 / 3.0));
        assert!(isclose(y, -1.0 / 3.0));
    }

    #[test]
    fn test_polygon_area_negative_vertices() {
        let vertices = vec![0.0, 0.0, 1.0, 0.0, 1.0, -1.0];
        assert!(isclose(polygon_area_internal(&vertices), 0.5));
    }
}
