use std::time::{Duration, Instant};
use std::error::Error;
use good_lp::{constraint, default_solver, Solution, SolverModel, variables, minilp,
Constraint};

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

    let mut cond: Vec<Constraint> = vec![];
    cond.push(constraint!(x[0] + x[1] + x[2] <= acre));
    cond.push(constraint!(2.5 *x[0] + y[0] - w[0] >= 200.));
    cond.push(constraint!(3 * x[1] + y[1] - w[1] >= 240.));
    cond.push(constraint!(w[2] + w[3] - 20.* x[2] <=0 ));
    cond.push(constraint!(w[2] <= 6000.));

    let mut solution = vars.minimise(
        &objective
    )
        .using(minilp); // multiple solvers available

    for c in cond{
        solution.add_constraint(c);
    }
    let res = solution.solve()?;
        // .with(constraint!(x[0] + x[1] + x[2] <= acre))
        // .with(constraint!(2.5 *x[0] + y[0] - w[0] >= 200.))
        // .with(constraint!(3 * x[1] + y[1] - w[1] >= 240.))
        // .with(constraint!(w[2] + w[3] - 20.* x[2] <=0 ))
        // .with(constraint!(w[2] <= 6000.))
        // .solve()?;


        // for (idx,v) in x.iter().enumerate()  {
        //     println!("x[{idx}] = {}", solution.value(*v));
        // }
        // for (idx,v) in y.iter().enumerate()  {
        //     println!("y[{idx}] = {}", solution.value(*v));
        // }
        // for (idx,v) in w.iter().enumerate()  {
        //     println!("w[{idx}] = {}", solution.value(*v));
        // }
        println!("optimal value: {}", res.eval(&objective));
    Ok(())
}


fn main() {
    let now = Instant::now();

    for idx in 1..=1 {
        farmer(500*idx);
    }
     println!("elapsed:{:.4} secs", now.elapsed().as_secs_f32());
}
