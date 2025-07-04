from typing import TypedDict


class Map(TypedDict):
    """Map information DTO. Naming as in TypeScript code."""

    baseSize: int
    borderSize: int

    buildings: list["MapBuilding"]


class MapBuilding(TypedDict):
    name: str
    level: int
    x: int
    y: int
