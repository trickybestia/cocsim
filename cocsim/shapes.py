Shape = tuple
"""Shape type. Represents graphics primitive used in Building.draw().
For all values see frontend/types.ts.
"""


def rect(x: float, y: float, width: float, height: float, color: str) -> Shape:
    return ("rect", x, y, width, height, color)


def circle(x: float, y: float, radius: float, color: str) -> Shape:
    return ("circle", x, y, radius, color)


def line(
    x1: float, y1: float, x2: float, y2: float, width: float, color: str
) -> Shape:
    return ("line", x1, y1, x2, y2, width, color)
