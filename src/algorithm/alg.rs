pub trait Alg {
    type Params;

    type Input<'a>
    where
        Self: 'a;

    type Output;

    fn run<'a>(&'a mut self, input: Self::Input<'a>) -> Self::Output;
}
