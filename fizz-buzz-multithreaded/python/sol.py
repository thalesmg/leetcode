from threading import Barrier
from typing import Callable


class FizzBuzz:
    def __init__(self, n: int):
        self.n = n
        self.is_3 = False
        self.is_5 = False
        self.barrier = Barrier(4)


    # printFizz() outputs "fizz"
    def fizz(self, printFizz: 'Callable[[], None]') -> None:
        for i in range(1, self.n + 1):
            self.is_3 = i % 3 == 0
            self.barrier.wait()
            if self.is_3 and not self.is_5:
                printFizz()
            self.barrier.wait()


    # printBuzz() outputs "buzz"
    def buzz(self, printBuzz: 'Callable[[], None]') -> None:
        for i in range(1, self.n + 1):
            self.is_5 = i % 5 == 0
            self.barrier.wait()
            if self.is_5 and not self.is_3:
                printBuzz()
            self.barrier.wait()


    # printFizzBuzz() outputs "fizzbuzz"
    def fizzbuzz(self, printFizzBuzz: 'Callable[[], None]') -> None:
        for i in range(1, self.n + 1):
            self.barrier.wait()
            if self.is_3 and self.is_5:
                printFizzBuzz()
            self.barrier.wait()


    # printNumber(x) outputs "x", where x is an integer.
    def number(self, printNumber: 'Callable[[int], None]') -> None:
        for i in range(1, self.n + 1):
            self.barrier.wait()
            if not self.is_3 and not self.is_5:
                printNumber(i)
            self.barrier.wait()
