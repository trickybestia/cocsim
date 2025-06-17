from math import sin, cos

from .point import Point
from .square import Square
from .segment import Segment


class Ray:
    start: Point
    direction: Point  # vector

    def __init__(self, start: Point, angle: float):
        self.start = start
        self.direction = Point(cos(angle), sin(angle))

    def intersection_with_segment(
        self, segment: Segment
    ) -> (
        Point | None
    ):  # https://stackoverflow.com/questions/14307158/how-do-you-check-for-intersection-between-a-line-segment-and-a-line-ray-emanatin
        p = self.start
        r = self.direction

        q = segment.a
        s = segment.b - segment.a

        if abs(r.cross(s)) < 0.00001:
            return

        t = (q - p).cross(s / r.cross(s))
        u = (q - p).cross(r / r.cross(s))

        if t >= 0.0 and 0.0 <= u <= 1.0:
            return q + s * u

    def intersection_with_square(self, square: Square) -> Point | None:
        for segment in square.segments:
            intersection = self.intersection_with_segment(segment)

            if intersection is not None:
                return intersection
