use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    types::{Duration, Energy, EnergyPerTime, Length, Percentage, PreTaxPrice, PricePerTime, Rate, Time},
    Table,
};

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "machine_models")]
#[changeset(setter(prefix = "with"))]
pub struct MachineModel {
    pub name: String,
    pub amortized_lifetime: Duration,
    pub average_energy_consumption: EnergyPerTime,
    pub price: PreTaxPrice,
    pub additional_pieces: PreTaxPrice,
    pub annual_maintenance: PricePerTime,
    pub print_width: Length,
    pub print_depth: Length,
    pub print_height: Length,
}

// impl MachineModel {
//     /// Calculate the total cost of ownership for the machine model over its amortized lifetime.
//     // pub fn total_cost(&self) -> PreTaxPrice {
//     //     self.price + self.additional_pieces + (*self.annual_maintenance * self.amortized_lifetime.as_years())
//     // }

//     /// Calculate the hourly energy cost based on the provided energy cost per kWh.
//     // pub fn hourly_energy_cost(&self, energy_cost: PreTaxPrice) -> PreTaxPrice {
//     //     energy_cost * *self.average_energy_consumption.to_kilowatt_hour()
//     // }

//     /// Calculate the hourly depreciation cost based on the total cost and amortized lifetime.
//     // pub fn hourly_depreciation_cost(&self) -> PreTaxPrice {
//     //     self.total_cost() / self.amortized_lifetime.as_hours()
//     // }

//     // /// Calculate the total hourly operating cost, including energy and depreciation, adjusted by the operating factor.
//     // pub fn hourly_operating_cost(&self, energy_cost: PreTaxPrice, operating_factor: Percentage) -> PreTaxPrice {
//     //     (self.hourly_energy_cost(energy_cost) + self.hourly_depreciation_cost()) * operating_factor.to_normalize()
//     // }
// }

#[derive(Debug, Clone, FromRow, Table)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[table(name = "machines")]
#[changeset(setter(prefix = "with"))]
pub struct Machine {
    pub machine_model_id: Uuid,
    pub printing_environment_id: Uuid,
    pub nickname: String,
}
