use z3::*;

pub fn main_simple() {

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // define constants:
    let x = ast::Bool::new_const(&ctx, "x");
    let y = ast::Bool::new_const(&ctx, "y");

    let not_y = y.not(); // !y
    let x_or_not_y = ast::Bool::or(&ctx, &[&x, &not_y]); // y -> x
    let not_x = x.not(); // !x
    let not_x_or_y = ast::Bool::or(&ctx, &[&not_x, &y]); // x -> y

    solver.assert(&x_or_not_y);
    solver.assert(&not_x_or_y);
    solver.assert(&x);

    println!();
    println!("finding a model for x -> y, y -> x, x true:");
    println!();
    println!("{}", solver);

    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("model:");
        println!("x -> {}", model.eval(&x, false).unwrap().as_bool().unwrap());
        println!("y -> {}", model.eval(&y, false).unwrap().as_bool().unwrap());
    }
    solver.reset();

    solver.assert(&x_or_not_y);
    solver.assert(&not_x_or_y);
    solver.assert(&not_x);

    println!();
    println!("finding a model for x -> y, y -> x, x false:");
    println!();
    println!("{}", solver);

    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!("model:");
        println!("x -> {}", model.eval(&x, false).unwrap().as_bool().unwrap());
        println!("y -> {}", model.eval(&y, false).unwrap().as_bool().unwrap());
    }

}