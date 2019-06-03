from pyo3avro_rs import Schema


def run():  # type: () -> None
    schema = Schema('{"type": "string"}')

    payload = schema.write("some-text")
    initial = schema.read(payload)

    print(initial)
    assert initial == "some-text"


if __name__ == "__main__":
    run()
