from time import time
from typing import Callable


class SpinTimer:
    on_tick: Callable[[float | None], None]

    _running: bool
    _interval: float
    _last_tick_time: float | None

    def __init__(self, interval: float):
        self.on_tick = None

        self._running = False
        self._interval = interval
        self._last_tick_time = None

    def stop(self):
        self._running = False
        self._last_tick_time = None

    def run(self):
        self._running = True

        while True:
            tick_start_time = time()

            self.on_tick(
                None
                if self._last_tick_time is None
                else tick_start_time - self._last_tick_time
            )

            self._last_tick_time = tick_start_time

            if not self._running:
                break

            next_call_time = tick_start_time + self._interval

            while time() < next_call_time:
                pass  # spin wait
