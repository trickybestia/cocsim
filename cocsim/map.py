from typing import TypedDict


class Map(TypedDict):
    """Map information DTO."""

    base_size: int
    border_size: int

    buildings: list["MapBuilding"]


class MapBuilding(TypedDict):
    name: str
    level: int
    x: int
    y: int
