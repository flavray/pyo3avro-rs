import pytest
from pyo3avro_rs import Schema


def test_dummy() -> None:
    assert Schema('{"type": "string"}')


def test_wrong_schema() -> None:
    with pytest.raises(ValueError):
        Schema('{"type": "error"}')
