use std::error::Error;
use good_lp::{constraint, default_solver, Solution, SolverModel, variables, minilp};

// fn farmer(){
//     variables! {
//         vars:
//            0 <= x[3];
//            0 <= y[2];
//            0 <= w[4];
//     }
//     let solution = vars.maximise(
//         150 * x[0] + 230 * x[1] + 260 * x[2] -
//             238 * y[0] -170 * w[0] + 210 * y[1]  - 150 * w[1] -
//             36 * w[2] - 10 * w[3]
//     )
//         .using(minilp) // multiple solvers available
//         .with(constraint!(x[0] + x[1] + x[2] <= 500))
//         .with(constraint!(2.5 *x[0] + y[0] - w[0] >= 200))
//         .with(constraint!(3 * x[1] + y[1] - w[1] >= 240))
//         .with(constraint!(w[2] - w[3] <= 20* x[2]))
//         .with(constraint!(w[3] <= 6000))
//         .solve()?;
//
//       println!("x={}, {}, {}", solution.value(x[0]),
//                solution.value(x[1]), solution.value(x[2]));
//       println!("y={}, {}", solution.value(y[0]),
//                solution.value(y[1]));
//       println!("w={}, {}, {}, {}", solution.value(w[0]),
//                solution.value(w[1]), solution.value(w[2]),
//                 solution.value(w[3]));
// }


fn main() -> Result<(), Box<dyn Error>> {
    // variables! {
    //     vars:
    //            a <= 1;
    //       2 <= b <= 4;
    // } // variables can also be added dynamically
    // let solution = vars.maximise(10 * (a - b / 5) - b)
    //     .using(minilp) // multiple solvers available
    //     .with(constraint!(a + 2 <= b))
    //     .with(constraint!(1 + a >= 4 - b))
    //     .solve()?;
    // println!("a={}   b={}", solution.value(a), solution.value(b));
    // println!("a + b = {}", solution.eval(a + b));
      variables! {
        vars:
           0 <= x[3];
           0 <= y[2];
           0 <= w[4];
    }
    let objective = 150 * x[0] + 230 * x[1] + 260 * x[2] +
            238 * y[0] -170 * w[0] + 210 * y[1]  - 150 * w[1] -
            36 * w[2] - 10 * w[3];
    let solution = vars.minimise(
        &objective
    )
        .using(minilp) // multiple solvers available
        .with(constraint!(x[0] + x[1] + x[2] <= 500.))
        .with(constraint!(2.5 *x[0] + y[0] - w[0] >= 200.))
        .with(constraint!(3 * x[1] + y[1] - w[1] >= 240.))
        .with(constraint!(w[2] + w[3] - 20.* x[2] <=0 ))
        .with(constraint!(w[3] <= 6000.))
        .solve()?;


        for (idx,v) in x.iter().enumerate()  {
            println!("x[{idx}] = {}", solution.value(*v));
        }
      println!("y={}, {}", solution.value(y[0]),
               solution.value(y[1]));
      println!("w={}, {}, {}, {}", solution.value(w[0]),
               solution.value(w[1]), solution.value(w[2]),
                solution.value(w[3]));
    println!("objective:{}", solution.eval(&objective));

    Ok(())
}
