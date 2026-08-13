#![allow(clippy::pedantic)]

mod machine;
mod maintenance;

pub use machine::{
    Machine, MachineInput, MachineModel, MachineModelChangeset, MachineModelInput, MachineModelMutation,
    MachineModelQuery, MachineMutation, MachineQuery, MachineState,
};
pub use maintenance::{
    MachineMaintenance, MachineMaintenanceChangeset, MachineMaintenanceCriticity, MachineMaintenanceInput,
    MachineMaintenanceKind, MachineMaintenanceMutation, MachineMaintenanceOverview, MachineMaintenanceQuery,
    MachineMaintenanceSetting, MachineMaintenanceSettingChangeset, MachineMaintenanceSettingInput,
    MachineMaintenanceSettingMutation, MachineMaintenanceSettingQuery, MachineMaintenanceStatus,
    MachineMaintenanceStatusQuery,
};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
