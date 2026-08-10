use acebau_unit::{EnergyPerTime, Length, Price, PricePerTime, Time};
use sqlx::FromRow;
use uuid::Uuid;

use crate::Table;

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "machine_models")]
#[changeset(setter(prefix = "with"))]
pub struct MachineModel {
    pub name: String,
    pub amortized_lifetime: Time,
    pub energy_consumption: EnergyPerTime,
    pub price: Price,
    pub additional_pieces: Price,
    pub annual_maintenance: PricePerTime,
    pub print_width: Length,
    pub print_depth: Length,
    pub print_height: Length,
}

impl MachineModel {
    /// Calculate the total price of ownership for the machine, including maintenance over its
    /// amortized lifetime.
    pub fn total_price(&self) -> Price {
        self.price + self.additional_pieces + (self.annual_maintenance * self.amortized_lifetime)
    }

    /// Calculate the depreciation cost by dividing the total price by the amortized lifetime of the
    /// machine.
    ///
    /// In order to account for the operating factor (i.e. how much the machine is actually used),
    /// the depreciation cost must be multiplied by the operating factor.
    pub fn depreciation_cost(&self) -> PricePerTime {
        self.total_price() / self.amortized_lifetime
    }
}

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "machines")]
#[changeset(setter(prefix = "with"))]
pub struct Machine {
    pub machine_model_id: Uuid,
    pub printing_environment_id: Uuid,
    pub nickname: String,
}

#[cfg(test)]
mod tests {
    use acebau_unit::Energy;

    use super::*;

    fn create_machine_model() -> MachineModel {
        MachineModel {
            name: "Bambulab X1C".to_string(),
            amortized_lifetime: Time::from_years(5.0),
            energy_consumption: EnergyPerTime::new(Energy::from_watts(200.0), Time::from_hours(1.0)),
            price: Price::new(1650.0),
            additional_pieces: Price::new(200.0),
            annual_maintenance: PricePerTime::new(Price::new(165.0), Time::from_years(1.0)),
            print_width: Length::from_millimeters(256.0),
            print_depth: Length::from_millimeters(256.0),
            print_height: Length::from_millimeters(256.0),
        }
    }

    #[test]
    fn test_machine_model_total_price() {
        let model = create_machine_model();

        assert_eq!(model.total_price(), Price::new(2675.0));
    }

    #[test]
    fn test_machine_model_depreciation_cost() {
        let model = create_machine_model();

        assert_eq!(
            model.depreciation_cost(),
            PricePerTime::new(Price::new(2675.0), Time::from_years(5.0))
        );
    }
}
