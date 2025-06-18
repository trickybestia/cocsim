from dataclasses import dataclass
from numbers import Number


@dataclass
class Point:
    x: float
    y: float

    def length(self) -> float:
        return (self.x**2 + self.y**2) ** 0.5

    def normalize(self) -> "Point":
        length = self.length()

        return Point(self.x / length, self.y / length)

    def cross(self, other: "Point") -> float:
        return self.x * other.y - self.y * other.x

    def __add__(self, other) -> "Point":
        if isinstance(other, Point):
            return Point(self.x + other.x, self.y + other.y)
        if isinstance(other, Number):
            return Point(self.x + other, self.y + other)

        raise TypeError()

    def __sub__(self, other) -> "Point":
        if isinstance(other, Point):
            return Point(self.x - other.x, self.y - other.y)
        if isinstance(other, Number):
            return Point(self.x - other, self.y - other)

        raise TypeError()

    def __mul__(self, other) -> "Point":
        if isinstance(other, Number):
            return Point(self.x * other, self.y * other)

        raise TypeError()

    def __truediv__(self, other) -> "Point":
        if isinstance(other, Number):
            return Point(self.x / other, self.y / other)

        raise TypeError()
