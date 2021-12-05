from dataclasses import dataclass, is_dataclass
from enum import Enum
from typing import List


class FileType(Enum):
    csv = "csv"
    json = "json"


@dataclass
class Read:
    path: str
    filetype: str
    schema: dict


@dataclass
class Transform:
    pass


@dataclass
class Write:
    path: str
    filetype: str


@dataclass
class Plan:
    read: Read = None
    transform: List[Transform] = None
    write: Write = None


class Fburpp:
    def __init__(self) -> None:
        self._plan = Plan()

    @property
    def plan(self) -> dict:
        output = {}
        for k, v in self._plan.__dict__.items():
            pass

    def read(
        self, filetype: FileType, path: str, schema: dict
    ) -> None:
        if not isinstance(filetype, FileType):
            msg = f"`filetype` must be of FileType option"
            raise TypeError(msg)
        
        read = Read(
            path=path, filetype=filetype, schema=schema
        )
        self._plan.read = read
        return self

    def filter(self, **kwargs) -> None:
        return self

    def write(
        self, filetype: FileType, path: str
    ) -> None:
        if not isinstance(filetype, FileType):
            msg = f"`filetype` must be of FileType option"
            raise TypeError(msg)

        write = Write(path=path, filetype=filetype)
        self._plan.write = write
        return self
