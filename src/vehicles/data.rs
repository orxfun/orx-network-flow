use crate::vehicle_types::VehicleType;

pub struct VehicleData {
    vehicle_type: VehicleType,
}

impl VehicleData {
    pub fn new(vehicle_type: VehicleType) -> Self {
        Self { vehicle_type }
    }

    pub fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }
}
