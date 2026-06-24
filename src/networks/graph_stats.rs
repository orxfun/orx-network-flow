#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GraphStats {
    pub num_vertices: usize,
    pub num_edges: usize,
    pub min_in_degree: usize,
    pub max_in_degree: usize,
    pub avg_in_degree: f64,
    pub min_out_degree: usize,
    pub max_out_degree: usize,
    pub avg_out_degree: f64,
    pub min_total_degree: usize,
    pub max_total_degree: usize,
    pub avg_total_degree: f64,
}

impl GraphStats {
    pub(crate) fn from_degrees(
        in_degrees: &[usize],
        out_degrees: &[usize],
        num_edges: usize,
    ) -> Self {
        let num_vertices = in_degrees.len();
        if num_vertices == 0 {
            return Self::default();
        }

        let total_in: usize = in_degrees.iter().sum();
        let total_out: usize = out_degrees.iter().sum();

        let min_in_degree = *in_degrees.iter().min().expect("non-empty");
        let max_in_degree = *in_degrees.iter().max().expect("non-empty");
        let min_out_degree = *out_degrees.iter().min().expect("non-empty");
        let max_out_degree = *out_degrees.iter().max().expect("non-empty");

        let mut min_total_degree = usize::MAX;
        let mut max_total_degree = 0usize;
        let mut total_degree_sum = 0usize;
        for (&in_degree, &out_degree) in in_degrees.iter().zip(out_degrees.iter()) {
            let degree = in_degree + out_degree;
            min_total_degree = core::cmp::min(min_total_degree, degree);
            max_total_degree = core::cmp::max(max_total_degree, degree);
            total_degree_sum += degree;
        }

        let avg_in_degree = total_in as f64 / num_vertices as f64;
        let avg_out_degree = total_out as f64 / num_vertices as f64;
        let avg_total_degree = total_degree_sum as f64 / num_vertices as f64;

        Self {
            num_vertices,
            num_edges,
            min_in_degree,
            max_in_degree,
            avg_in_degree,
            min_out_degree,
            max_out_degree,
            avg_out_degree,
            min_total_degree,
            max_total_degree,
            avg_total_degree,
        }
    }
}
