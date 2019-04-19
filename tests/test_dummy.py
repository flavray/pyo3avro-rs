from pyo3avro_rs import AvroSchema


def test_dummy():
    schema = AvroSchema('{"type": "string"}')
    assert schema is not None
