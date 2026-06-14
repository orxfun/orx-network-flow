pub trait Alg {
    type Params;

    type Input;

    type Output;

    fn run(&mut self, input: &Self::Input) -> Self::Output;
}
