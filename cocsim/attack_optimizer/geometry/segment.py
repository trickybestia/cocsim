from dataclasses import dataclass

from .point import Point


@dataclass
class Segment:
    a: Point
    b: Point
