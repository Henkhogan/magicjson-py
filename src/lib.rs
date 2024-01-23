use chrono::NaiveDateTime;
use magicjsonrust::load_file as _load_file;
use magicjsonrust::objects::{JsonKey, JsonItem, JsonCustomType};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBool, PyDateTime, PyDict, PyFloat, PyString, PyTuple};
use chrono::Datelike;
use chrono::Timelike;
use std::collections::HashMap;

struct PyJsonCustomType {
    pub name: String,
    pub value: String,
}

impl From<JsonCustomType> for PyJsonCustomType {
    fn from(item: JsonCustomType) -> Self {
        PyJsonCustomType {
            name: item.name,
            value: item.value,
        }
    }
    
}


impl IntoPy<Py<PyAny>> for PyJsonCustomType {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        let dict = PyDict::new(py);
        dict.set_item("name", self.name).unwrap();
        dict.set_item("value", self.value).unwrap();
        return dict.into();
    }
}

// Ideally later on these are native python types which are converted within From<JsonItem>
enum PyJsonItem {
    Bool(bool),
    Dict(HashMap<JsonKey, Self>),
    Int(i32),
    List(Vec<Self>),
    Float(f64),
    Null(), 
    Str(String),
    Custom(JsonCustomType),
    Datetime(NaiveDateTime),
    Timestamp(f64),
}


impl From<JsonItem> for PyJsonItem {
    fn from(item: JsonItem) -> Self {
        match item {
            JsonItem::Bool(value) => {
                return PyJsonItem::Bool(value);
            },
            JsonItem::Dict(value) => {
                return PyJsonItem::Dict(value.into_iter().map(|(k, v)|(k, Self::from(v))).collect());
            },
            JsonItem::Int(value) => {
                return PyJsonItem::Int(value);
            },
            JsonItem::List(value) => {
                return PyJsonItem::List(value.into_iter().map(|i|Self::from(i)).collect());
            },
            JsonItem::Float(value) => {
                return PyJsonItem::Float(value);
            },
            JsonItem::Null() => {
                return PyJsonItem::Null();
            },
            JsonItem::Str(value) => {
                return PyJsonItem::Str(value);
            },
            JsonItem::Datetime(value) => {
                return PyJsonItem::Datetime(value);
            },
            JsonItem::Timestamp(value) => {
                return PyJsonItem::Timestamp(value);
            },
            JsonItem::Custom(value) => {
                return PyJsonItem::Custom(value);
            },
        }
    }
}

impl IntoPy<Py<PyAny>> for PyJsonItem {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            PyJsonItem::Bool(_value) => {
                return PyBool::new(py, _value).into();
            },
            PyJsonItem::Dict(_value) => {
                return _value.into_py(py);
            },
            PyJsonItem::Int(_value) => {
                return _value.into_py(py);
            },
            PyJsonItem::List(_value) => {
                return PyTuple::new(py, _value.into_iter().map(|i|i.into_py(py))).into();
            },
            PyJsonItem::Float(_value) => {
                return PyFloat::new(py, _value).into();
            },
            PyJsonItem::Null() => {
                return ().into_py(py);
            },
            PyJsonItem::Str(_value) => {
                return PyString::new(py, &_value).into();
            },
            PyJsonItem::Datetime(_value) => {

                return PyDateTime::new(
                    py,
                    _value.date().year() as i32,
                    _value.date().month() as u8,
                    _value.date().day() as u8,
                    _value.time().hour() as u8,
                    _value.time().minute() as u8,
                    _value.time().second() as u8,
                    _value.timestamp_subsec_micros(),
                    None,
                ).unwrap().into();
            },
            PyJsonItem::Timestamp(_value) => {
                return PyDateTime::from_timestamp(py, _value, None).unwrap().into();
            },
            PyJsonItem::Custom(_value) => {
                return (_value.name, _value.value).into_py(py);
            },
        }

    }
}


/// Reads a JSON file and returns a JsonItem
#[pyfunction]
fn load_file(file_path: String) -> PyJsonItem {
   return _load_file(file_path);
}

#[pyfunction]
fn loads(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn dump(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn dumps(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn magicjson(_py: Python, m: &PyModule) -> PyResult<()> {
    
    //Builder::new()
    //.format(|buf, record| {
    //    writeln!(buf,
    //        "{} [{}] - {}",
    //        Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //        record.level(),
    //        record.args()
    //    )
    //})
    //.filter(None, LevelFilter::Debug)
    //.init();

    pyo3_log::init();



    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}