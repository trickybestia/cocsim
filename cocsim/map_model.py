from typing import Annotated, Protocol

from pydantic import Field, create_model

from . import buildings


class BuildingModel(Protocol):
    name: str


class MapModel(Protocol):
    base_size: int
    border_size: int

    buildings: list[BuildingModel]


def create_map_model() -> MapModel:
    buildings_type = buildings.BUILDINGS[0].model()

    for building_type in buildings.BUILDINGS[1:]:  # create union
        buildings_type |= building_type.model()

    return create_model(
        "MapModel",
        base_size=(int, Field(alias="baseSize", ge=1, le=44)),
        border_size=(int, Field(alias="borderSize", ge=0, le=4)),
        buildings=(
            list[Annotated[buildings_type, Field(discriminator="name")]]
        ),
    )
