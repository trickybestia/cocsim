from tkinter import *


INVALID_SPINBOX_BACKGROUND = "red"


class NamedSpinbox:
    from_: int
    to: int

    label: Label
    spinbox: Spinbox

    _variable: IntVar
    _variable_internal: Variable

    _default_spinbox_background: str

    def __init__(
        self,
        master: Misc | None,
        label_text: str,
        from_: int,
        to: int,
        variable: IntVar,
    ):
        self.from_ = from_
        self.to = to
        self._variable = variable
        self._variable_internal = Variable(master, value=variable.get())

        self.label = Label(master, text=label_text, anchor=E)
        self.spinbox = Spinbox(
            master, textvariable=self._variable_internal, from_=from_, to=to
        )

        self._default_spinbox_background = self.spinbox.cget("background")

        self._variable_internal.trace_add(
            "write", self._on_variable_internal_write
        )
        self._variable.trace_add(
            "write",
            lambda *args: self._variable_internal.set(self._variable.get()),
        )

    def _on_variable_internal_write(self, *args):
        if (
            value := self._validate_spinbox(
                self._variable_internal.get(), self.from_, self.to
            )
        ) is not None:
            self._variable.set(value)
            self.spinbox.configure(background=self._default_spinbox_background)
        else:
            self.spinbox.configure(background=INVALID_SPINBOX_BACKGROUND)

    def _validate_spinbox(
        self, value: str, min_value: int, max_value: int
    ) -> int | None:
        try:
            value = int(value)

            if min_value <= value <= max_value:
                return value

            return None
        except ValueError:
            return None
