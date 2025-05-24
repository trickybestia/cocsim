from dataclasses import dataclass
from typing import Type

from tkinter import *
import PIL.Image
import PIL.ImageTk

from ..buildings import Building, Wall
from cocsim.consts import *
from .compose_base_images import REVERSE_PROJECTION_IMAGE_SIZE
from .building_selection_window import BuildingSelectionWindow


@dataclass
class BuildingInfo:
    building: Type[Building]
    level: int
    tile_x: int
    tile_y: int
    rectangle: int

    def to_tuple(self) -> tuple[str, int, int, int]:
        return self.building.__name__, self.level, self.tile_x, self.tile_y


class MainWindow:
    root: Tk
    image: PIL.ImageTk.PhotoImage
    canvas: Canvas

    cursor_rectangle: int | None
    selection_rectangle: int | None
    selection_start_pos: tuple[int, int] | None
    occupied_tiles_selected: bool

    buildings: list[BuildingInfo]
    buildings_grid: list[list[BuildingInfo | None]]

    def __init__(self, image: PIL.Image.Image):
        self.cursor_rectangle = None
        self.selection_rectangle = None
        self.selection_start_pos = None
        self.occupied_tiles_selected = False

        self.buildings = []
        self.buildings_grid = [[None] * MAP_SIZE for _ in range(MAP_SIZE)]

        self.root = Tk()
        self.root.resizable(False, False)
        self.root.title("cocsim.map_editor_gui")
        self.root.geometry(
            f"{REVERSE_PROJECTION_IMAGE_SIZE}x{REVERSE_PROJECTION_IMAGE_SIZE}"
        )
        self.root.bind("<Button-1>", self.on_left_click)
        self.root.bind("<Button-3>", self.on_right_click)
        self.root.bind("<Motion>", self.on_motion)
        self.root.bind("<Escape>", self.on_escape)

        self.image = PIL.ImageTk.PhotoImage(image)
        self.canvas = Canvas(
            width=REVERSE_PROJECTION_IMAGE_SIZE,
            height=REVERSE_PROJECTION_IMAGE_SIZE,
        )
        self.canvas.create_image(0, 0, image=self.image, anchor=NW)
        self.canvas.pack()

    def on_left_click(self, event):
        tile_x, tile_y = self._get_tile_position(event.x, event.y)

        if self.selection_start_pos is None:
            self.selection_start_pos = (tile_x, tile_y)
        elif not self.occupied_tiles_selected:
            building_x = min(tile_x, self.selection_start_pos[0])
            building_y = min(tile_y, self.selection_start_pos[1])
            selection_size = (
                abs(tile_x - self.selection_start_pos[0]) + 1,
                abs(tile_y - self.selection_start_pos[1]) + 1,
            )

            def on_building_selected(building: Type[Building], level: int):
                if building is Wall:
                    for x in range(selection_size[0]):
                        for y in range(selection_size[1]):
                            self._add_building(
                                building, level, building_x + x, building_y + y
                            )
                else:
                    self._add_building(building, level, building_x, building_y)

            BuildingSelectionWindow(
                self.root, selection_size, on_building_selected
            )

            self.selection_start_pos = None

    def on_right_click(self, event):
        self.selection_start_pos = None

        if self.selection_rectangle is not None:
            self.canvas.delete(self.selection_rectangle)
            self.selection_rectangle = None
        else:
            tile_x, tile_y = self._get_tile_position(event.x, event.y)

            if (building := self.buildings_grid[tile_x][tile_y]) is not None:
                self.canvas.delete(building.rectangle)

                self.buildings.remove(building)

                for x in range(building.building.width()):
                    for y in range(building.building.height()):
                        self.buildings_grid[building.tile_x + x][
                            building.tile_y + y
                        ] = None

    def on_motion(self, event):
        tile_x, tile_y = self._get_tile_position(event.x, event.y)

        if self.cursor_rectangle is not None:
            self.canvas.delete(self.cursor_rectangle)
        if self.selection_rectangle is not None:
            self.canvas.delete(self.selection_rectangle)
            self.selection_rectangle = None

        if self.selection_start_pos is not None:
            self._draw_selection(tile_x, tile_y)

        self._draw_cursor_rectangle(tile_x, tile_y)

    def on_escape(self, event):
        self.selection_start_pos = None

        if self.selection_rectangle is not None:
            self.canvas.delete(self.selection_rectangle)
            self.selection_rectangle = None

    def run(self):
        self.root.mainloop()

    def _add_building(
        self, building: Type[Building], level: int, tile_x: int, tile_y: int
    ):
        building_info = BuildingInfo(
            building,
            level,
            tile_x,
            tile_y,
            self._draw_tiles_rectangle(
                tile_x,
                tile_y,
                tile_x + building.width() - 1,
                tile_y + building.height() - 1,
                outline="yellow",
                width=2,
            ),
        )

        self.buildings.append(building_info)

        for x in range(building.width()):
            for y in range(building.height()):
                self.buildings_grid[tile_x + x][tile_y + y] = building_info

    def _get_tile_position(self, x: int, y: int) -> tuple[int, int]:
        pixels_per_tile = REVERSE_PROJECTION_IMAGE_SIZE // MAP_SIZE

        tile_x = x // pixels_per_tile
        tile_y = y // pixels_per_tile

        return tile_x, tile_y

    def _draw_selection(self, end_tile_x: int, end_tile_y: int):
        self._check_selection_occupied(end_tile_x, end_tile_y)

        color = "red" if self.occupied_tiles_selected else "black"
        start_tile_x, start_tile_y = self.selection_start_pos

        self.selection_rectangle = self._draw_tiles_rectangle(
            start_tile_x, start_tile_y, end_tile_x, end_tile_y, outline=color
        )

    def _check_selection_occupied(self, end_tile_x: int, end_tile_y: int):
        min_x, max_x = sorted((self.selection_start_pos[0], end_tile_x))
        min_y, max_y = sorted((self.selection_start_pos[1], end_tile_y))

        self.occupied_tiles_selected = False

        for x in range(min_x, max_x + 1):
            for y in range(min_y, max_y + 1):
                if self.buildings_grid[x][y] is not None:
                    self.occupied_tiles_selected = True

                    return

    def _draw_cursor_rectangle(self, tile_x: int, tile_y: int):
        self.cursor_rectangle = self._draw_tiles_rectangle(
            tile_x, tile_y, tile_x, tile_y
        )

    def _draw_tiles_rectangle(
        self,
        start_tile_x: int,
        start_tile_y: int,
        end_tile_x: int,
        end_tile_y: int,
        **kwargs,
    ) -> int:
        pixels_per_tile = REVERSE_PROJECTION_IMAGE_SIZE // MAP_SIZE

        min_x = min(
            start_tile_x * pixels_per_tile, end_tile_x * pixels_per_tile
        )
        max_x = max(
            (start_tile_x + 1) * pixels_per_tile - 1,
            (end_tile_x + 1) * pixels_per_tile - 1,
        )
        min_y = min(
            start_tile_y * pixels_per_tile, end_tile_y * pixels_per_tile
        )
        max_y = max(
            (start_tile_y + 1) * pixels_per_tile - 1,
            (end_tile_y + 1) * pixels_per_tile - 1,
        )

        return self.canvas.create_rectangle(
            min_x, min_y, max_x, max_y, **kwargs
        )
