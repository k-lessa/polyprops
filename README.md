# polyprops

polyprops is a Python package written in Rust used for the analysis of polygon properties with a focus on performance.

## Features

- Fast and efficient calculation of polygon properties using algorithms written in Rust

## Installation

Install polyprops with pip:

```bash
  pip install polyprops
```

## Usage/Examples

```python
import polyprops as pp

# Create a polygon
polygon = pp.Polygon([
    0, 0,
    10, 0,
    10, 10,
])

# Calculate the area
area = polygon.area()

# Calculate the centroid
centroid = polygon.centroid()
```