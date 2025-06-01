from .collider import Collider
from ...utils import distance


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
        nearest_point = None
        nearest_point_distance = None

        for collider in self.colliders:
            collider_nearest_point = collider.get_nearest_point(x, y)
            collider_nearest_point_distance = distance(
                x, y, collider_nearest_point[0], collider_nearest_point[1]
            )

            if (
                nearest_point_distance is None
                or collider_nearest_point_distance < nearest_point_distance
            ):
                nearest_point = collider_nearest_point
                nearest_point_distance = collider_nearest_point_distance

        return nearest_point

    def __contains__(self, item: tuple[int, int]) -> bool:
        return any(item in collider for collider in self.colliders)
