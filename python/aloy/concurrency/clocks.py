###############################################################################
# Copyright (C) 2023 Oliver Michael Kamperis
# Email: olliekampo@gmail.com
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free Software
# Foundation, either version 3 of the License, or any later version.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
# FOR A PARTICULAR PURPOSE. See the GNU General Public License for details.
#
# You should have received a copy of the GNU General Public License along with
# this program. If not, see <https://www.gnu.org/licenses/>.

"""Module containing functions and classes for thread clocks."""

import threading
import time
from typing import Any, Callable, Final, Protocol, final, runtime_checkable
import warnings

__copyright__ = "Copyright (C) 2023 Oliver Michael Kamperis"
__license__ = "GPL-3.0"
__version__ = "0.1.0"

__all__ = (
    "SimpleClockThread",
    "Tickable"
)


def __dir__() -> tuple[str, ...]:
    """Get the names of module attributes."""
    return __all__


@runtime_checkable
class Tickable(Protocol):
    """A protocol for tickable objects."""

    def tick(self, *args: Any, **kwargs: Any) -> None:
        """Tick the object."""


@final
class _SimpleClockItem:
    """Class defining a simple clock item."""

    __slots__ = {
        "__func": "The function to call.",
        "__args": "The arguments to pass to the function.",
        "__kwargs": "The keyword arguments to pass to the function."
    }

    def __init__(
        self,
        func: Callable[..., None],
        *args: Any,
        **kwargs: Any
    ) -> None:
        """Create a new simple clock item."""
        self.__func = func
        self.__args = args
        self.__kwargs = kwargs

    def __call__(self) -> None:
        """Call the function."""
        self.__func(*self.__args, **self.__kwargs)

    def __eq__(self, __value: object) -> bool:
        """Return whether the value is equal to the item."""
        if isinstance(__value, _SimpleClockItem):
            return self.__func == __value.__func  # pylint: disable=W0212
        if callable(__value):
            return self.__func == __value
        return NotImplemented


@final
class SimpleClockThread:
    """
    Class defining a thread used to run a clock for
    regularly calling functions at a given tick rate.
    """

    __DELAYED_TICKS_WARNING: Final[int] = 100
    __DELAYED_TICKS_RESET: Final[int] = 10

    __slots__ = {
        "__items": "The items to tick.",
        "__atomic_update_lock": "A lock making start and stop calls atomic.",
        "__sleep_time": "The time to sleep between ticks.",
        "__thread": "The thread that runs the clock.",
        "__running": "Event handling whether the clock is running.",
        "__stopped": "Event handling whether the clock should stop."
    }

    def __init__(
        self,
        tick_rate: int = 10
    ) -> None:
        """
        Create a new clock thread with the given tickable items.

        `tick_rate: int = 10` - The tick rate of the clock (ticks/second).
        This is approximate, the actual tick rate may vary, the only
        guarantee is that the tick rate will not exceed the given value.
        """
        # Schedule items.
        self.__atomic_update_lock = threading.Lock()
        self.__items: list[_SimpleClockItem] = []
        self.tick_rate = tick_rate

        # Variables for the clock thread.
        self.__thread = threading.Thread(target=self.__run)
        self.__thread.daemon = True
        self.__running = threading.Event()
        self.__stopped = threading.Event()
        self.__thread.start()

    def __str__(self) -> str:
        """Return a string representation of the clock thread."""
        return (f"ClockThread: with {len(self.__items)} items "
                f"at tick rate {self.tick_rate} ticks/second.")

    def schedule(
        self,
        func: Tickable | Callable[..., None],
        *args: Any,
        **kwargs: Any
    ) -> None:
        """
        Schedule an item to be ticked by the clock.

        Parameters
        ----------
        """
        with self.__atomic_update_lock:
            if isinstance(func, Tickable):
                func = func.tick
            elif not callable(func):
                raise TypeError(f"Item {func!r} of type {type(func)} is "
                                "not tickable or callable.")
            self.__items.append(_SimpleClockItem(func, *args, **kwargs))

    def unschedule(self, func: Tickable | Callable[..., None]) -> None:
        """Unschedule an item from being ticked by the clock."""
        with self.__atomic_update_lock:
            if isinstance(func, Tickable):
                func = func.tick
            elif not callable(func):
                raise TypeError(f"Item {func!r} of type {type(func)} is "
                                "not tickable or callable.")
            self.__items.remove(func)  # type: ignore[arg-type]

    @property
    def tick_rate(self) -> int:
        """Return the tick rate of the clock."""
        return int(1.0 / self.__sleep_time)

    @tick_rate.setter
    def tick_rate(self, value: int) -> None:
        """Set the tick rate of the clock."""
        if value <= 0:
            raise ValueError("Tick rate must be greater than 0. "
                             f"Got; {value}.")
        self.__sleep_time = 1.0 / value

    def __run(self) -> None:
        """Run the clock."""
        while True:
            self.__running.wait()

            sleep_time: float = self.__sleep_time
            start_sleep_time: float = time.perf_counter()
            delayed_ticks: int = 0
            ticks_since_last_delayed_tick: int = 0

            while not self.__stopped.wait(sleep_time):
                actual_sleep_time = time.perf_counter() - start_sleep_time
                if actual_sleep_time > (sleep_time * 1.05):
                    delayed_ticks += 1
                    if (delayed_ticks % self.__DELAYED_TICKS_WARNING) == 0:
                        warnings.warn(
                            f"[{self!s}] Unable to reach tick rate "
                            f"for {delayed_ticks} ticks."
                        )
                elif (delayed_ticks > 0
                        and (ticks_since_last_delayed_tick
                             < self.__DELAYED_TICKS_RESET)):
                    ticks_since_last_delayed_tick += 1
                elif (ticks_since_last_delayed_tick
                      == self.__DELAYED_TICKS_RESET):
                    delayed_ticks = 0
                    ticks_since_last_delayed_tick = 0

                start_update_time = time.perf_counter()
                with self.__atomic_update_lock:
                    for item in self.__items:
                        item()
                update_time = time.perf_counter() - start_update_time

                if update_time > actual_sleep_time:
                    sleep_time = 0.0
                    warnings.warn(
                        f"[{self!s}] Tick rate too high for scheduled items. "
                        "Update time longer than sleep time. Actual sleep "
                        f"time = {actual_sleep_time:.3f} seconds, items "
                        f"update time = {update_time:.3f} seconds. Setting "
                        "sleep time to 0.0 seconds."
                    )
                else:
                    # Adjust sleep time to account for update time.
                    sleep_time = ((sleep_time + self.__sleep_time)
                                  - (actual_sleep_time + update_time))

                start_sleep_time = time.perf_counter()

    def start(self) -> None:
        """Start the clock."""
        with self.__atomic_update_lock:
            if not self.__running.is_set():
                self.__stopped.clear()
                self.__running.set()

    def stop(self) -> None:
        """Stop the clock."""
        with self.__atomic_update_lock:
            if self.__running.is_set():
                self.__stopped.set()
                self.__running.wait()
