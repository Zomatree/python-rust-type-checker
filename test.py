import typing

class A:
    def __init__(self, arg: int) -> None:
        self.attr = arg


def make_a(arg: int) -> A:
    return A(arg)
