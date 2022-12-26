use cairo_rs_py::cairo_runner::PyCairoRunner;
use std::fs;

fn main() {
    let path = "../cairo_programs/assert_not_zero.json".to_string();
    let program = fs::read_to_string(path).unwrap();
    let mut runner =
        PyCairoRunner::new(program, Some("main".to_string()), None, false).unwrap();
    runner
        .cairo_run_py(false, None, None, None, None, None)
        .expect("Couldn't run program");

}