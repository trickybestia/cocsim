from tkinter import *
from sys import argv


class Gui:
    root: Tk

    def __init__(self):
        self.root = Tk()

    def run(self):
        self.root.mainloop()
