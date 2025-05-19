class Collider:
    def get_attack_area(self, attack_range: float) -> "Collider":
        """Returns attack area from which units with given attack range
        can attack this building.
        """
        ...

    def get_nearest_point(self, x: float, y: float) -> tuple[float, float]:
        """Returns point of collider nearest to (x, y)."""

    def __contains__(self, item: tuple[int, int]) -> bool:
        """Checks if item (x, y) is inside of collider."""
        ...
