from enum import Enum
from json import dumps
from typing import List, Optional


class FileType(Enum):
    CSV = "csv"


class Fburpp:
    pass


class FburppError(Exception):
    pass


def fburpp(filetype: FileType) -> Fburpp:
    if not isinstance(filetype, FileType):
        msg = "`filetype` must be a value of fburpp.FileType"
        raise FburppError(msg)

    if filetype == FileType.CSV:
        return CSV()


class CSV(Fburpp):
    def __init__(self) -> None:
        self._input_path = None
        self._output_path = None
        self._col_names = None
        self._col_types = None
        self._select = None

    @property
    def plan(self) -> dict:
        if not self._plan_ready():
            msg = "Missing attributes required for plan"
            raise FburppError(msg)

        return {
            "input_path": self._input_path,
            "output_path": self._output_path,
            "col_names": self._col_names,
            "col_types": self._col_types,
            "select": self._select,
        }

    def run(self) -> None:
        plan = dumps(self.plan)
        # call to rust fburpp with plan

    def read(
        self, 
        path: str, 
        col_names: List[str], 
        col_types: List[type],
    ):
        self._input_path = path
        self._col_names = col_names
        self._col_types = list(
            map(self._map_type, col_types)
        )
        return self

    def write(self, path: str) -> None:
        self._output_path = path
        return self

    def select(self, columns: List[str]) -> None:
        for col in columns:
            if col not in self._col_names:
                msg = f"Column '{col}' does not exist"
                raise FburppError(msg)

        self._select = columns
        return self

    def _plan_ready(self) -> Optional[List[str]]:
        required = (
            self._col_names,
            self._col_types,
            self._input_path,
            self._output_path,
        )

        for r in required:
            if not r:
                return False

        return True

    @staticmethod
    def _map_type(col_type: type) -> str:
        types = {
            str: "str",
            int: "int",
        }

        if col_type not in types:
            msg = f"Type '{col_type}' not supported"
            raise FburppError(msg)

        return types[col_type]
