from tkinter import *
from typing import Callable, Type

from cocsim.buildings import Building
from cocsim.consts import *

from .level_selection_window import LevelSelectionWindow
from .utils import fuzzy_sort, get_buildings_with_size


class BuildingSelectionWindow:
    on_building_selected: Callable[[Type[Building], int], None]
    buildings: list[Type[Building]]

    root: Tk
    window: Toplevel
    entry_variable: StringVar
    entry: Entry
    ok_button: Button
    list_variable: StringVar
    listbox: Listbox

    def __init__(
        self,
        root: Tk,
        selection_size: tuple[int, int],
        on_building_selected: Callable[[Type[Building], int], None],
    ):
        self.on_building_selected = on_building_selected
        self.buildings = get_buildings_with_size(selection_size)

        self.root = root
        self.window = Toplevel(self.root)
        self.window.bind("<Return>", self.select_building)
        self.window.bind("<Escape>", lambda e: self.window.destroy())
        self.window.bind("<Up>", self.on_up)
        self.window.bind("<Down>", self.on_down)

        self.entry_variable = StringVar(self.window)
        self.entry = Entry(self.window, textvariable=self.entry_variable)
        self.entry_variable.trace_add("write", self.on_entry_text_change)

        self.ok_button = Button(
            self.window, text="OK", command=self.select_building
        )

        self.list_variable = StringVar(
            self.window, [b.__name__ for b in self.buildings]
        )
        self.listbox = Listbox(
            self.window, selectmode=SINGLE, listvariable=self.list_variable
        )
        self.listbox.selection_set(0)
        self.listbox.bind("<Double-1>", self.select_building)

        self.window.rowconfigure(1, weight=1)
        self.window.columnconfigure(0, weight=1)

        self.entry.grid(column=0, row=0, sticky=N + S + E + W)
        self.ok_button.grid(column=1, row=0, sticky=N + S + E + W)
        self.listbox.grid(column=0, row=1, columnspan=2, sticky=N + S + E + W)

        self.window.wait_visibility()
        self.window.grab_set()

        self.entry.focus()

    def on_entry_text_change(self, *args):
        text = self.entry_variable.get()

        fuzzy_sort(text, self.buildings, lambda b: b.__name__)

        self.list_variable.set([b.__name__ for b in self.buildings])
        self.listbox.selection_clear(0, END)
        self.listbox.selection_set(0)

    def on_up(self, event):
        if len(self.listbox.curselection()) == 0:
            return

        selected_index = self.listbox.curselection()[0]

        if selected_index != 0:
            self.listbox.selection_clear(0, END)
            self.listbox.selection_set(selected_index - 1)

    def on_down(self, event):
        if len(self.listbox.curselection()) == 0:
            return

        selected_index = self.listbox.curselection()[0]

        if selected_index != len(self.buildings) - 1:
            self.listbox.selection_clear(0, END)
            self.listbox.selection_set(selected_index + 1)

    def select_building(self, *args):
        if len(self.listbox.curselection()) == 0:
            return

        building = self.buildings[self.listbox.curselection()[0]]

        self.window.destroy()

        LevelSelectionWindow(
            self.root,
            building,
            self.on_building_selected,
        )
