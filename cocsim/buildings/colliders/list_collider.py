from cocsim.utils import distance

from .collider import Collider


class ListCollider(Collider):
    colliders: list[Collider]

    def __init__(self, colliders: list[Collider]):
        super().__init__()

        self.colliders = colliders

    def get_attack_area(self, attack_range: float) -> Collider:
        return ListCollider(
            [
                collider.get_attack_area(attack_range)
                for collider in self.colliders
            ]
        )

    def get_nearest_point(self, x: float, y: float) -> tuple[float, float]:
        return min(
            (collider.get_nearest_point(x, y) for collider in self.colliders),
            key=lambda point: distance(point[0], point[1], x, y),
        )

    def __contains__(self, item: tuple[int, int]) -> bool:
        return any(item in collider for collider in self.colliders)
