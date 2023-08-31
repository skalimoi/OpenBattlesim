pub mod clut {
    
    use std::fs;

    use pyo3::{PyResult, Python, types::PyModule, PyAny, Py};

    pub fn generate_clut() {
        let gen_clut = fs::read_to_string("main.py");
        let from_python = Python::with_gil(|py| {
            let app: Py<PyAny> = PyModule::from_code(py, &gen_clut.unwrap(), "", "")?
                .getattr("gen_clut")?
                .into();
            app.call0(py)
        });

        

    }

    pub fn apply_clut() {
        let gen_clut = fs::read_to_string("main.py");
        let from_python = Python::with_gil(|py| {
            let app: Py<PyAny> = PyModule::from_code(py, &gen_clut.unwrap(), "", "")?
                .getattr("apply_clut_png")?
                .into();
            app.call0(py)
        });

    }
}
