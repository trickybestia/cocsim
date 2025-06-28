from .point import Point
from .segment import Segment


class Square:
    segments: list[Segment]

    def __init__(self, center: Point, side: float):
        left_bottom = center - side / 2
        left_top = left_bottom + Point(0.0, side)
        right_bottom = left_bottom + Point(side, 0.0)
        right_top = right_bottom + Point(0.0, side)

        self.segments = [
            Segment(left_top, right_top),
            Segment(right_top, right_bottom),
            Segment(right_bottom, left_bottom),
            Segment(left_bottom, left_top),
        ]
