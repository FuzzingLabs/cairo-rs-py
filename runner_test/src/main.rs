
use std::env;
use std::fs;
mod parse_json;
mod cairo_rs_py;
use cairo_rs_py::cairo_runner::PyCairoRunner;
/* use cairo_rs_py;
use cairo_rs_py::cairo_runner as cairo_rs_py_runner;
use cairo_rs_py_runner::PyCairoRunner; */
use crate::parse_json::parse_json;
mod utils;

fn py_runner(json: &String, func_name: String, args_num: u64, data: isize) {
    println!("Running py_runner");
    let mut cairo_runner = PyCairoRunner::new(json.to_string(), Some(func_name.clone()), Some("all".to_string()), false).unwrap();
    //cairo_runner.initialize_function_runner();
    cairo_runner.initialize_segments();
/*     Python::with_gil(|py| {
        cairo_runner
            .run_from_entrypoint(
                py,
                py.eval("0", None, None).unwrap(),
                Vec::<&PyAny>::new().to_object(py),
                None,
                None,
                Some(false),
                None,
                None,
            )
            .unwrap();
    }); */
    let entrypoint: &str = &func_name.clone();
    cairo_runner.cairo_run_py(true, None, None, None, None, Some(entrypoint)).expect("could not run program");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage : cargo run -- <PATH>");
        return;
    }
    let filename: &String = &args[1];
    let functions = parse_json(filename);
    let contents = fs::read_to_string(filename).expect("could not read contract artefact");
    for function in functions {
        py_runner(&contents, function.name, function.num_args, 5);
    }
}
