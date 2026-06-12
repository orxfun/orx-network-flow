use good_lp::solvers::lp_solvers::Model;

pub unsafe fn lp_solvers_model_to_problem<S>(model: &Model<S>) -> &lp_solvers::problem::Problem {
    let x = model as *const Model<_> as *const lp_solvers::problem::Problem;
    unsafe { &*x }
}
