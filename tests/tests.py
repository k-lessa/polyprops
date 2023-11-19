import unittest
from polyprops import Polygon


class TestPolygons(unittest.TestCase):
    @staticmethod
    def isclose(a, b):
        return abs(a - b) < 1e-6

    def test_polygon_area(self):
        vertices = [0, 0, 10, 0, 10, 10]
        polygon = Polygon(vertices)
        self.assertTrue(self.isclose(polygon.area(), 50))

    def test_polygon_centroid(self):
        vertices = [0, 0, 10, 0, 10, 10]
        polygon = Polygon(vertices)
        centroid = polygon.centroid()
        self.assertTrue(self.isclose(centroid[0], 10 * 2 / 3))
        self.assertTrue(self.isclose(centroid[1], 10 / 3))


if __name__ == "__main__":
    unittest.main()
