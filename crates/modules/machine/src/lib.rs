#![allow(clippy::pedantic)]

mod machine;

pub use machine::{
    Machine, MachineInput, MachineModel, MachineModelChangeset, MachineModelInput, MachineModelMutation,
    MachineModelQuery, MachineMutation, MachineQuery, MachineState,
};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
