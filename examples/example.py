from pyo3avro_rs import AvroSchema


def run():  # type: () -> None
    schema = AvroSchema('{"type": "string"}')

    payload = schema.write("some-text")
    initial = schema.read(payload)

    print(initial)
    assert initial == "some-text"


if __name__ == "__main__":
    run()
