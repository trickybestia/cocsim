from dataclasses import dataclass
from typing import Type

from tkinter import *
import PIL.Image
import PIL.ImageTk

from ..buildings import Building, Wall, BUILDINGS_DICT
from ..map import Map
from cocsim.consts import *
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


CANVAS_SIZE = 832

INITIAL_BASE_SIZE = 44
INITIAL_BORDER_SIZE = 4


class MainWindow:
    image: PIL.Image.Image

    root: Tk
    tk_image: PIL.ImageTk.PhotoImage | None
    canvas: Canvas

    controls_frame: Frame
    draw_grid_label: Label
    draw_grid_variable: BooleanVar
    draw_grid_checkbutton: Checkbutton
    base_size_label: Label
    base_size_variable: IntVar
    base_size_spinbox: Spinbox
    border_size_label: Label
    border_size_variable: IntVar
    border_size_spinbox: Spinbox
    image_start_x_label: Label
    image_start_x_variable: IntVar
    image_start_x_spinbox: Spinbox
    image_start_y_label: Label
    image_start_y_variable: IntVar
    image_start_y_spinbox: Spinbox
    image_end_x_label: Label
    image_end_x_variable: IntVar
    image_end_x_spinbox: Spinbox
    image_end_y_label: Label
    image_end_y_variable: IntVar
    image_end_y_spinbox: Spinbox

    cursor_rectangle: int | None
    selection_rectangle: int | None
    selection_start_pos: tuple[int, int] | None
    invalid_tiles_selected: bool

    base_image_id: int | None
    grid_tiles_ids: list[int]

    buildings: list[BuildingInfo]
    buildings_grid: list[list[BuildingInfo | None]]

    @property
    def total_size(self) -> int:
        return (
            self.base_size_variable.get() + 2 * self.border_size_variable.get()
        )

    @property
    def _pixels_per_tile(self) -> float:
        return CANVAS_SIZE / self.total_size

    def __init__(self, image: PIL.Image.Image):
        self.image = image

        self.cursor_rectangle = None
        self.selection_rectangle = None
        self.selection_start_pos = None
        self.invalid_tiles_selected = False
        self.grid_tiles_ids = []

        self.root = Tk()
        self.root.resizable(False, False)
        self.root.title("cocsim.map_editor_gui")

        self.draw_grid_variable = BooleanVar()
        self.base_size_variable = IntVar(value=INITIAL_BASE_SIZE)
        self.border_size_variable = IntVar(value=INITIAL_BORDER_SIZE)
        self.image_start_x_variable = IntVar()
        self.image_start_y_variable = IntVar()
        self.image_size_y_variable = IntVar(
            value=min(image.width, image.height)
        )
        self.image_end_x_variable = IntVar(value=image.width - 1)
        self.image_end_y_variable = IntVar(value=image.height - 1)

        self.buildings = []
        self._update_buildings_grid()

        self.canvas = Canvas(width=CANVAS_SIZE, height=CANVAS_SIZE)
        self.tk_image = None
        self.base_image_id = None
        self._update_base_image()

        self.canvas.bind("<Button-1>", self.on_left_click)
        self.canvas.bind("<Button-3>", self.on_right_click)
        self.canvas.bind("<Motion>", self.on_motion)
        self.canvas.bind("<Escape>", self.on_escape)

        self.controls_frame = Frame()

        # self.root.columnconfigure(2, weight=1)

        self.canvas.grid(column=1, row=0, sticky=N + S + E + W)
        self.controls_frame.grid(column=0, row=0, sticky=N + E + W)

        self.draw_grid_label = Label(
            self.controls_frame, text="Draw grid:", anchor=E
        )
        self.draw_grid_checkbutton = Checkbutton(
            self.controls_frame, variable=self.draw_grid_variable
        )
        self.draw_grid_variable.trace_add("write", self._on_draw_grid_changed)
        self.draw_grid_label.grid(column=0, row=0, sticky=N + E + W)
        self.draw_grid_checkbutton.grid(column=1, row=0, sticky=N + E + W)

        self.base_size_label = Label(
            self.controls_frame, text="Base size:", anchor=E
        )
        self.base_size_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.base_size_variable,
            from_=1,
            to=44,
        )
        self.base_size_variable.trace_add("write", self._on_base_size_changed)
        self.base_size_label.grid(column=0, row=1, sticky=N + E + W)
        self.base_size_spinbox.grid(column=1, row=1, sticky=N + E + W)

        self.border_size_label = Label(
            self.controls_frame, text="Border size:", anchor=E
        )
        self.border_size_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.border_size_variable,
            from_=0,
            to=4,
        )
        self.border_size_variable.trace_add("write", self._on_base_size_changed)
        self.border_size_label.grid(column=0, row=2, sticky=N + E + W)
        self.border_size_spinbox.grid(column=1, row=2, sticky=N + E + W)

        self.image_start_x_label = Label(
            self.controls_frame, text="Start X:", anchor=E
        )
        self.image_start_x_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.image_start_x_variable,
            from_=0,
            to=image.width - 1,
        )
        self.image_start_x_variable.trace_add("write", self._update_base_image)
        self.image_start_x_label.grid(column=0, row=3, sticky=N + E + W)
        self.image_start_x_spinbox.grid(column=1, row=3, sticky=N + E + W)

        self.image_start_y_label = Label(
            self.controls_frame, text="Start Y:", anchor=E
        )
        self.image_start_y_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.image_start_y_variable,
            from_=0,
            to=image.height - 1,
        )
        self.image_start_y_variable.trace_add("write", self._update_base_image)
        self.image_start_y_label.grid(column=0, row=4, sticky=N + E + W)
        self.image_start_y_spinbox.grid(column=1, row=4, sticky=N + E + W)

        self.image_end_x_label = Label(
            self.controls_frame, text="End X:", anchor=E
        )
        self.image_end_x_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.image_end_x_variable,
            from_=0,
            to=image.width - 1,
        )
        self.image_end_x_variable.trace_add("write", self._update_base_image)
        self.image_end_x_label.grid(column=0, row=5, sticky=N + E + W)
        self.image_end_x_spinbox.grid(column=1, row=5, sticky=N + E + W)

        self.image_end_y_label = Label(
            self.controls_frame, text="End Y:", anchor=E
        )
        self.image_end_y_spinbox = Spinbox(
            self.controls_frame,
            textvariable=self.image_end_y_variable,
            from_=0,
            to=image.height - 1,
        )
        self.image_end_y_variable.trace_add("write", self._update_base_image)
        self.image_end_y_label.grid(column=0, row=6, sticky=N + E + W)
        self.image_end_y_spinbox.grid(column=1, row=6, sticky=N + E + W)

    def set_map(self, map: Map):
        self.base_size_variable.set(map["base_size"])
        self.border_size_variable.set(map["border_size"])
        self.buildings = []

        self._update_buildings_grid()

        for building in map["buildings"]:
            self._add_building(
                BUILDINGS_DICT[building["name"]],
                building["level"],
                building["x"],
                building["y"],
            )

        self._update_buildings_grid()

    def get_map(self) -> Map:
        result: Map = {}

        result["base_size"] = self.base_size_variable.get()
        result["border_size"] = self.border_size_variable.get()
        result["buildings"] = [
            {
                "name": building.building.__name__,
                "level": building.level,
                "x": building.tile_x,
                "y": building.tile_y,
            }
            for building in self.buildings
        ]

        return result

    def on_left_click(self, event):
        tile_x, tile_y = self._get_tile_position(event.x, event.y)

        if self.selection_start_pos is None:
            self.selection_start_pos = (tile_x, tile_y)
        elif not self.invalid_tiles_selected:
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
                self._remove_building(building)

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

    def _on_draw_grid_changed(self, *args):
        for tile_id in self.grid_tiles_ids:
            self.canvas.delete(tile_id)

        self.grid_tiles_ids = []

        if self.draw_grid_variable.get():
            for tile_x in range(self.total_size):
                for tile_y in range(self.total_size):
                    self.grid_tiles_ids.append(
                        self._draw_tiles_rectangle(
                            tile_x, tile_y, tile_x, tile_y, outline="white"
                        )
                    )

    def _on_base_size_changed(self, *args):
        self._update_buildings_grid()
        self._on_draw_grid_changed()

    def _update_base_image(self, *args):
        image = self.image.crop(
            (
                self.image_start_x_variable.get(),
                self.image_start_y_variable.get(),
                self.image_end_x_variable.get() + 1,
                self.image_end_y_variable.get() + 1,
            )
        )
        image = image.resize((CANVAS_SIZE, CANVAS_SIZE))

        self.canvas.delete(self.base_image_id)
        del self.tk_image
        self.tk_image = PIL.ImageTk.PhotoImage(image)
        self.base_image_id = self.canvas.create_image(
            0, 0, image=self.tk_image, anchor=NW
        )
        self.canvas.tag_lower(self.base_image_id)

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

    def _update_buildings_grid(self):
        self.buildings_grid = [
            [None] * self.total_size for _ in range(self.total_size)
        ]

        i = 0

        while i < len(self.buildings):
            building = self.buildings[i]

            if not self._check_inside_building_area(
                building.tile_x, building.tile_y
            ) or not self._check_inside_building_area(
                building.tile_x + building.building.width() - 1,
                building.tile_y + building.building.height() - 1,
            ):
                self.canvas.delete(building.rectangle)
                self.buildings.remove(building)

                continue

            for x in range(building.building.width()):
                for y in range(building.building.height()):
                    self.buildings_grid[building.tile_x + x][
                        building.tile_y + y
                    ] = building

            i += 1

    def _check_inside_building_area(self, tile_x: int, tile_y: int) -> bool:
        border_size = self.border_size_variable.get()

        return (
            border_size <= tile_x < self.total_size - border_size
            and border_size <= tile_y < self.total_size - border_size
        )

    def _remove_building(self, building: BuildingInfo):
        self.canvas.delete(building.rectangle)

        self.buildings.remove(building)

        for x in range(building.building.width()):
            for y in range(building.building.height()):
                self.buildings_grid[building.tile_x + x][
                    building.tile_y + y
                ] = None

    def _get_tile_position(self, x: int, y: int) -> tuple[int, int]:
        tile_x = int(x / self._pixels_per_tile)
        tile_y = int(y / self._pixels_per_tile)

        return tile_x, tile_y

    def _draw_selection(self, end_tile_x: int, end_tile_y: int):
        self._check_can_build_in_selection(end_tile_x, end_tile_y)

        color = "red" if self.invalid_tiles_selected else "black"
        start_tile_x, start_tile_y = self.selection_start_pos

        self.selection_rectangle = self._draw_tiles_rectangle(
            start_tile_x, start_tile_y, end_tile_x, end_tile_y, outline=color
        )

    def _check_can_build_in_selection(self, end_tile_x: int, end_tile_y: int):
        min_x, max_x = sorted((self.selection_start_pos[0], end_tile_x))
        min_y, max_y = sorted((self.selection_start_pos[1], end_tile_y))

        self.invalid_tiles_selected = False

        for x in range(min_x, max_x + 1):
            for y in range(min_y, max_y + 1):
                if (
                    not self._check_inside_building_area(x, y)
                    or self.buildings_grid[x][y] is not None
                ):
                    self.invalid_tiles_selected = True

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
        min_x = min(
            start_tile_x * self._pixels_per_tile,
            end_tile_x * self._pixels_per_tile,
        )
        max_x = max(
            (start_tile_x + 1) * self._pixels_per_tile - 1,
            (end_tile_x + 1) * self._pixels_per_tile - 1,
        )
        min_y = min(
            start_tile_y * self._pixels_per_tile,
            end_tile_y * self._pixels_per_tile,
        )
        max_y = max(
            (start_tile_y + 1) * self._pixels_per_tile - 1,
            (end_tile_y + 1) * self._pixels_per_tile - 1,
        )

        return self.canvas.create_rectangle(
            min_x, min_y, max_x, max_y, **kwargs
        )
