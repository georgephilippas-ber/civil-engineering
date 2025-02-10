pub fn root<Function, Derivative>(
    function: Function,
    derivative: Derivative,
    x_0: f64,
    tolerance: f64,
    maximum_iterations: usize,
) -> Result<f64, &'static str>
where
    Function: Fn(f64) -> f64,
    Derivative: Fn(f64) -> f64,
{
    let mut x = x_0;

    for i in 0..maximum_iterations {
        let y = function(x);
        let dy = derivative(x);

        if dy.abs() < 1.0e-12 {
            return Err("ZERO_DERIVATIVE");
        }

        let x_next = x - y / dy;

        if (x_next - x).abs() < tolerance {
            return Ok(x_next);
        }

        x = x_next;
    }

    Err("MAXIMUM_ITERATIONS")
}
