use std::time::{Duration, Instant};
use std::error::Error;
use good_lp::{constraint, default_solver, Solution, SolverModel, variables,  highs};

fn farmer(acre: u32)-> Result<(), Box<dyn Error>>{
     let bounds = 0;
      variables! {
        vars:
           x[3] >= bounds;
           0 <= y[2];
           0 <= w[4];
    }
    let objective = 150 * x[0] + 230 * x[1] + 260 * x[2] +
            238 * y[0] -170 * w[0] + 210 * y[1]  - 150 * w[1] -
            36 * w[2] - 10 * w[3];
    let solution = vars.minimise(
        &objective
    )
<<<<<<< HEAD
        .using(highs) // multiple solvers available
        .with(constraint!(x[0] + x[1] + x[2] <= 500.))
=======
        .using(minilp) // multiple solvers available
        .with(constraint!(x[0] + x[1] + x[2] <= acre))
>>>>>>> 530d6bf02e90ecb0d8b79909cd8c88e5cfa9cac9
        .with(constraint!(2.5 *x[0] + y[0] - w[0] >= 200.))
        .with(constraint!(3 * x[1] + y[1] - w[1] >= 240.))
        .with(constraint!(w[2] + w[3] - 20.* x[2] <=0 ))
        .with(constraint!(w[2] <= 6000.))
        .solve()?;


        // for (idx,v) in x.iter().enumerate()  {
        //     println!("x[{idx}] = {}", solution.value(*v));
        // }
        // for (idx,v) in y.iter().enumerate()  {
        //     println!("y[{idx}] = {}", solution.value(*v));
        // }
        // for (idx,v) in w.iter().enumerate()  {
        //     println!("w[{idx}] = {}", solution.value(*v));
        // }
        println!("optimal value: {}", solution.eval(&objective));
    Ok(())
}


fn main() {
    let now = Instant::now();
    for idx in 1..=3000 {
        farmer(500*idx);
    }
     println!("elapsed:{:.4} secs", now.elapsed().as_secs_f32());
}
