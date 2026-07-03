use crate::serialization::ProblemInput;
use orx_network_flow::{ProblemBuilder, Variant};
#[derive(Clone, Copy, Default)]
pub struct MyVariant;

impl Variant for MyVariant {
    type S = String;
    type K = usize;
    type W = String;
    type V = usize;
    type T = usize;
    type F = u64;
    type C = i64;

    fn chargeable_flow(flow: Self::F) -> Self::C {
        flow as i64
    }
}

/// Build problem from form input
pub fn build_problem_from_input(
    input: ProblemInput,
) -> Result<orx_network_flow::Problem<MyVariant>, String> {
    // Validate inputs
    if input.spaces.is_empty() {
        return Err("At least one geographic space is required".into());
    }

    // Verify all referenced spaces exist
    let space_names: std::collections::HashSet<_> = input.spaces.iter().map(|s| &s.name).collect();

    for commodity in &input.commodities {
        if !space_names.contains(&commodity.origin) {
            return Err(format!(
                "Commodity {} references unknown origin space: {}",
                commodity.id, commodity.origin
            ));
        }
        if !space_names.contains(&commodity.destination) {
            return Err(format!(
                "Commodity {} references unknown destination space: {}",
                commodity.id, commodity.destination
            ));
        }
    }

    for transport in &input.transports {
        if !space_names.contains(&transport.origin) {
            return Err(format!(
                "Transport {} references unknown origin space: {}",
                transport.id, transport.origin
            ));
        }
        if !space_names.contains(&transport.destination) {
            return Err(format!(
                "Transport {} references unknown destination space: {}",
                transport.id, transport.destination
            ));
        }
    }

    let builder: ProblemBuilder<MyVariant, _> = ProblemBuilder::new();

    // Add geographic spaces - this changes the builder state to DefiningProblem
    let spaces_iter = input
        .spaces
        .iter()
        .map(|s| (s.name.clone(), s.latitude, s.longitude));
    let mut builder = builder.with_geographic_spaces(spaces_iter);

    // Add commodities
    for commodity in input.commodities {
        builder.push_commodity(
            commodity.id,
            commodity.origin,
            commodity.ready_time,
            commodity.destination,
            commodity.due_time,
            commodity.quantity,
        );
    }

    // Add transports
    for transport in input.transports {
        builder.push_transport(
            transport.id,
            transport.id, // Use same ID for vehicle
            transport.vehicle_type,
            transport.origin,
            transport.departure_time,
            transport.destination,
            transport.arrival_time,
            transport.capacity,
        );
    }

    // Add lost revenue costs
    {
        let mut lost_rev = builder.lost_revenue_cost();
        for item in input.lost_revenue_costs {
            lost_rev.commodity_specific(&item.commodity_id, item.cost_per_unit);
        }
    }

    // Finish building the problem
    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization::{
        FormCommodity, FormGeographicSpace, FormLostRevenueItem, FormTransport,
    };

    #[test]
    fn test_build_empty_problem() {
        let input = ProblemInput {
            spaces: vec![],
            commodities: vec![],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_problem_with_spaces_only() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace {
                    name: "A".into(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
                FormGeographicSpace {
                    name: "B".into(),
                    latitude: 1.0,
                    longitude: 1.0,
                },
            ],
            commodities: vec![],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.len_spaces(), 2);
        assert_eq!(problem.len_commodities(), 0);
        assert_eq!(problem.len_transports(), 0);
    }

    #[test]
    fn test_build_problem_with_commodities() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace {
                    name: "A".into(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
                FormGeographicSpace {
                    name: "B".into(),
                    latitude: 1.0,
                    longitude: 1.0,
                },
            ],
            commodities: vec![FormCommodity {
                id: 1,
                origin: "A".into(),
                ready_time: 0,
                destination: "B".into(),
                due_time: 100,
                quantity: 50,
            }],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.len_spaces(), 2);
        assert_eq!(problem.len_commodities(), 1);
    }

    #[test]
    fn test_build_problem_with_transports() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace {
                    name: "A".into(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
                FormGeographicSpace {
                    name: "B".into(),
                    latitude: 1.0,
                    longitude: 1.0,
                },
            ],
            commodities: vec![],
            transports: vec![FormTransport {
                id: 1,
                vehicle_type: "truck".into(),
                origin: "A".into(),
                departure_time: 10,
                destination: "B".into(),
                arrival_time: 90,
                capacity: 100,
            }],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.len_spaces(), 2);
        assert_eq!(problem.len_transports(), 1);
    }

    #[test]
    fn test_build_complete_problem() {
        let input = ProblemInput {
            spaces: vec![
                FormGeographicSpace {
                    name: "A".into(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
                FormGeographicSpace {
                    name: "B".into(),
                    latitude: 1.0,
                    longitude: 1.0,
                },
                FormGeographicSpace {
                    name: "C".into(),
                    latitude: 2.0,
                    longitude: 2.0,
                },
            ],
            commodities: vec![
                FormCommodity {
                    id: 1,
                    origin: "A".into(),
                    ready_time: 0,
                    destination: "B".into(),
                    due_time: 100,
                    quantity: 50,
                },
                FormCommodity {
                    id: 2,
                    origin: "B".into(),
                    ready_time: 50,
                    destination: "C".into(),
                    due_time: 150,
                    quantity: 30,
                },
            ],
            transports: vec![
                FormTransport {
                    id: 1,
                    vehicle_type: "truck".into(),
                    origin: "A".into(),
                    departure_time: 10,
                    destination: "B".into(),
                    arrival_time: 90,
                    capacity: 100,
                },
                FormTransport {
                    id: 2,
                    vehicle_type: "van".into(),
                    origin: "B".into(),
                    departure_time: 100,
                    destination: "C".into(),
                    arrival_time: 140,
                    capacity: 60,
                },
            ],
            lost_revenue_costs: vec![
                FormLostRevenueItem {
                    commodity_id: 1,
                    cost_per_unit: 5,
                },
                FormLostRevenueItem {
                    commodity_id: 2,
                    cost_per_unit: 3,
                },
            ],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_ok());
        let problem = result.unwrap();
        assert_eq!(problem.len_spaces(), 3);
        assert_eq!(problem.len_commodities(), 2);
        assert_eq!(problem.len_transports(), 2);
    }

    #[test]
    fn test_invalid_commodity_origin() {
        let input = ProblemInput {
            spaces: vec![FormGeographicSpace {
                name: "A".into(),
                latitude: 0.0,
                longitude: 0.0,
            }],
            commodities: vec![FormCommodity {
                id: 1,
                origin: "X".into(), // Non-existent space
                ready_time: 0,
                destination: "A".into(),
                due_time: 100,
                quantity: 50,
            }],
            transports: vec![],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("origin"));
    }

    #[test]
    fn test_invalid_transport_destination() {
        let input = ProblemInput {
            spaces: vec![FormGeographicSpace {
                name: "A".into(),
                latitude: 0.0,
                longitude: 0.0,
            }],
            commodities: vec![],
            transports: vec![FormTransport {
                id: 1,
                vehicle_type: "truck".into(),
                origin: "A".into(),
                departure_time: 10,
                destination: "Y".into(), // Non-existent space
                arrival_time: 90,
                capacity: 100,
            }],
            lost_revenue_costs: vec![],
        };

        let result = build_problem_from_input(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("destination"));
    }
}
