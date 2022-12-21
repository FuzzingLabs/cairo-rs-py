import cairo_rs_py

with open(f"cairo_programs/assert_not_zero.json") as file:
    runner = cairo_rs_py.CairoRunner(file.read(), "main", "all", False)
    runner.cairo_run(True)
