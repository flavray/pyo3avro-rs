#![feature(specialization)]

use std::collections::HashMap;

use avro_rs::from_avro_datum;
use avro_rs::to_avro_datum;
use avro_rs::types::Value;
use avro_rs::Schema;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

struct Bytes {
    bytes: Vec<u8>,
}

impl IntoPyObject for Bytes {
    #[inline]
    fn into_object(self, py: Python) -> PyObject {
        PyBytes::new(py, self.bytes.as_ref()).into()
    }
}

#[pyclass]
struct AvroSchema {
    schema: Schema,
}

#[pymethods]
impl AvroSchema {
    #[new]
    fn __new__(obj: &PyRawObject, input: String) {
        obj.init(AvroSchema {
            schema: Schema::parse_str(&input).unwrap(), // TODO
        })
    }

    fn write(&self, py: Python, datum: PyObject) -> PyResult<Bytes> {
        let value = to_avro_value(py, &datum, &self.schema)?;

        let bytes = to_avro_datum(&self.schema, value).unwrap(); // TODO
        Ok(Bytes { bytes })
    }

    fn read(&self, py: Python, datum: &PyBytes) -> PyResult<PyObject> {
        let mut bytes = datum.as_bytes();
        let value = from_avro_datum(&self.schema, &mut bytes, None).unwrap(); // TODO
        Ok(to_pyobject(py, value))
    }
}

fn to_pyobject(py: Python, datum: Value) -> PyObject {
    match datum {
        Value::Null => py.None(),
        Value::Boolean(b) => b.into_object(py),
        Value::Int(n) => n.into_object(py),
        Value::Long(n) => n.into_object(py),
        Value::Float(x) => x.into_object(py),
        Value::Double(x) => x.into_object(py),
        Value::Bytes(bytes) => Bytes { bytes }.into_object(py),
        Value::String(string) => string.into_object(py),
        Value::Fixed(_, bytes) => Bytes { bytes }.into_object(py),
        Value::Enum(_, symbol) => symbol.into_object(py),
        Value::Union(None) => py.None(),
        Value::Union(Some(item)) => to_pyobject(py, *item),
        Value::Array(items) => {
            // TODO
            let list = PyList::empty(py);
            for item in items {
                list.append(to_pyobject(py, item)).unwrap(); // TODO
            }
            list.into_object(py)
        }
        Value::Map(items) => {
            // TODO
            let dict = PyDict::new(py);
            for (key, value) in items {
                dict.set_item(key, to_pyobject(py, value)).unwrap(); // TODO
            }
            dict.into_object(py)
        }
        Value::Record(fields) => {
            let dict = PyDict::new(py);
            for (name, value) in fields {
                dict.set_item(name, to_pyobject(py, value)).unwrap(); // TODO
            }
            dict.into_object(py)
        }
    }
}

fn to_avro_value(py: Python, datum: &PyObject, schema: &Schema) -> PyResult<Value> {
    match schema {
        &Schema::Null if datum.is_none() => Ok(Value::Null),
        &Schema::Null => panic!("argh"),
        &Schema::Boolean => {
            let b = datum.extract::<bool>(py)?;
            Ok(Value::Boolean(b))
        }
        &Schema::Int => {
            // TODO: PyInt/PyLong?
            let n = datum.extract::<i32>(py)?;
            Ok(Value::Int(n))
        }
        &Schema::Long => {
            // TODO: PyInt/PyLong?
            let n = datum.extract::<i64>(py)?;
            Ok(Value::Long(n))
        }
        &Schema::Float => {
            let x = datum.extract::<f32>(py)?;
            Ok(Value::Float(x))
        }
        &Schema::Double => {
            let x = datum.extract::<f64>(py)?;
            Ok(Value::Double(x))
        }
        &Schema::Bytes => {
            let bytes = datum.extract::<Vec<u8>>(py)?;
            Ok(Value::Bytes(bytes))
        }
        &Schema::String => {
            let string = datum.extract::<String>(py)?;
            Ok(Value::String(string))
        }
        &Schema::Array(ref inner) => {
            // TODO: PyTuple?
            let array = datum.extract::<Vec<PyObject>>(py)?;
            let items = array
                .iter()
                .map(|item| to_avro_value(py, &item, inner))
                .collect::<PyResult<Vec<Value>>>()?;
            Ok(Value::Array(items))
        }
        &Schema::Map(ref inner) => {
            let items = datum
                .cast_as::<PyDict>(py)?
                .iter()
                .map(|(keyo, valueo)| {
                    Ok((
                        keyo.extract::<String>()?,
                        to_avro_value(py, &valueo.to_object(py), inner)?,
                    ))
                })
                .collect::<PyResult<HashMap<String, Value>>>()?;

            Ok(Value::Map(items))
        }
        &Schema::Union(_) if datum.is_none() => Ok(Value::Union(None)),
        &Schema::Union(ref inner) => Ok(Value::Union(Some(Box::new(to_avro_value(
            py, datum, inner,
        )?)))),
        &Schema::Record {
            ref fields,
            ref lookup,
            ..
        } => {
            let record_dict = datum.cast_as::<PyDict>(py)?;
            let mut rfields = Vec::with_capacity(record_dict.len());

            for (keyo, valueo) in record_dict.iter() {
                let key = keyo.extract::<String>()?;

                let fschema = if let Some(&position) = lookup.get(&key) {
                    &fields[position].schema
                } else {
                    panic!("argh");
                };

                let value = to_avro_value(py, &valueo.to_object(py), fschema)?;

                rfields.push((key, value));
            }

            Ok(Value::Record(rfields))
        }
        &Schema::Enum { ref symbols, .. } => {
            let string = datum.extract::<String>(py);
            if let Ok(string) = string {
                if let Some(index) = symbols.iter().position(|ref item| item == &&string) {
                    Ok(Value::Enum(index as i32, string))
                } else {
                    panic!("argh");
                }
            } else {
                let index = datum.extract::<i32>(py)? as usize;
                if index < symbols.len() {
                    Ok(Value::Enum(index as i32, symbols[index].clone()))
                } else {
                    panic!("argh")
                }
            }
        }
        &Schema::Fixed { .. } => {
            let bytes = datum.extract::<Vec<u8>>(py)?;
            Ok(Value::Fixed(bytes.len(), bytes))
        }
    }
}

#[pymodule]
fn pyo3avro_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AvroSchema>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
