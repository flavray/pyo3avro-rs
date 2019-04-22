from pyo3avro_rs import AvroSchema


def test_dummy() -> None:
    schema = AvroSchema('{"type": "string"}')
    assert schema is not None
