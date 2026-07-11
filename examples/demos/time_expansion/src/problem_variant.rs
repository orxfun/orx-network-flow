use orx_network_flow::Variant;

#[derive(Clone, Copy, Default)]
pub struct ProblemVariant;

impl Variant for ProblemVariant {
    type S = String;

    type K = String;

    type W = String;

    type V = usize;

    type T = usize;

    type F = u64;

    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}
