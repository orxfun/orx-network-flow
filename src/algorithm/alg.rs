pub trait Alg {
    type Params;

    type Input;

    type Output;

    fn run<'a>(&'a mut self, input: Self::Input) -> Self::Output;
}
