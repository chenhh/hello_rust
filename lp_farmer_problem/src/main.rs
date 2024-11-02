use std::time::{Duration, Instant};
use std::error::Error;
use good_lp::{constraint, Variable, Expression, Solution, SolverModel, variables,  default_solver};

fn farmer(acre: u32)-> Result<(), Box<dyn Error>>{
    // 使用marco建立變數
     let bounds = 0;
      variables! {
        vars:
           x[3] >= bounds;  // // x will be a vector (x[0], x[1],x[2]) of variables
           0 <= y[2];
           0 <= w[4];
    }
    let objective = 150 * x[0] + 230 * x[1] + 260 * x[2] +
            238 * y[0] -170 * w[0] + 210 * y[1]  - 150 * w[1] -
            36 * w[2] - 10 * w[3];
    let solution = vars.minimise(
        &objective
    )
        .using(default_solver) // multiple solvers available
        .with(constraint!(x[0] + x[1] + x[2] <= acre))
        .with(constraint!(2.5 *x[0] + y[0] - w[0] >= 200))
        .with(constraint!(3 * x[1] + y[1] - w[1] >= 240))
        .with(constraint!(w[2] + w[3]  <=  20* x[2] ))
        .with(constraint!(w[2] <= 6000 ))
        .solve()?;

        x.iter().enumerate().for_each(|(idx, v)|  println!("x[{idx}] = {}", solution.value(*v)));
        y.iter().enumerate().for_each(|(idx, v)|  println!("y[{idx}] = {}", solution.value(*v)));
        w.iter().enumerate().for_each(|(idx, v)|  println!("w[{idx}] = {}", solution.value(*v)));

        println!("optimal value: {}", solution.eval(&objective));
    Ok(())
}


fn main() {
    let now = Instant::now();
    farmer(500).unwrap();
    // for idx in 1..=10000 {
    //     farmer(500*idx);
    // }
     println!("elapsed:{:.4} secs", now.elapsed().as_secs_f32());
}
