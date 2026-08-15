#![allow(clippy::pedantic)]
#![recursion_limit = "256"]

mod handlers;

use acebau_activity::{ActivityMutation, ActivityQuery};
use acebau_analytics::AnalyticsQuery;
use acebau_catalogue::{ProductMutation, ProductQuery, VariantMutation, VariantQuery};
use acebau_database::{Database, DatabaseError, MigrationOptions};
use acebau_files::{FileMetadataMutation, FileMetadataQuery};
use acebau_finance::{ExpenseMutation, ExpenseQuery};
use acebau_inventory::{FilamentSpoolMutation, FilamentSpoolQuery, SupplyMutation, SupplyQuery};
use acebau_invoice::{ImportedInvoiceMutation, ImportedInvoiceQuery};
use acebau_machine::{
    MachineMaintenanceMutation, MachineMaintenanceQuery, MachineMaintenanceSettingMutation,
    MachineMaintenanceSettingQuery, MachineMaintenanceStatusQuery, MachineModelMutation, MachineModelQuery,
    MachineMutation, MachineQuery,
};
use acebau_order::{CustomerOrderMutation, CustomerOrderQuery, OrderLineMutation, OrderLineQuery};
use acebau_part::{PieceMachineProfileMutation, PieceMachineProfileQuery, PrintedPieceMutation, PrintedPieceQuery};
use acebau_production::{ProductionMutation, ProductionTaskLineQuery, ProductionTaskQuery};
use acebau_recipe::{RecipeItemMutation, RecipeItemQuery};
use acebau_reseller::{ResellerMutation, ResellerQuery};
use acebau_settings::{ApplicationSettingMutation, ApplicationSettingQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema};
use axum::{Router, routing::get};

#[derive(MergedObject, Default)]
struct QueryRoot(
    ProductQuery,
    VariantQuery,
    PrintedPieceQuery,
    PieceMachineProfileQuery,
    RecipeItemQuery,
    MachineQuery,
    MachineModelQuery,
    MachineMaintenanceSettingQuery,
    MachineMaintenanceQuery,
    MachineMaintenanceStatusQuery,
    SupplyQuery,
    FilamentSpoolQuery,
    ProductionTaskQuery,
    ProductionTaskLineQuery,
    ResellerQuery,
    CustomerOrderQuery,
    OrderLineQuery,
    ImportedInvoiceQuery,
    ExpenseQuery,
    ActivityQuery,
    FileMetadataQuery,
    ApplicationSettingQuery,
    AnalyticsQuery,
);

#[derive(MergedObject, Default)]
struct MutationRoot(
    ProductMutation,
    VariantMutation,
    PrintedPieceMutation,
    PieceMachineProfileMutation,
    RecipeItemMutation,
    MachineMutation,
    MachineModelMutation,
    MachineMaintenanceSettingMutation,
    MachineMaintenanceMutation,
    SupplyMutation,
    FilamentSpoolMutation,
    ProductionMutation,
    ResellerMutation,
    CustomerOrderMutation,
    OrderLineMutation,
    ImportedInvoiceMutation,
    ExpenseMutation,
    ActivityMutation,
    FileMetadataMutation,
    ApplicationSettingMutation,
);

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

pub async fn router(state: AppState) -> Result<Router, DatabaseError> {
    state
        .database
        .migrate(
            &[
                &acebau_database::MIGRATOR,
                &acebau_machine::MIGRATOR,
                &acebau_activity::MIGRATOR,
                &acebau_catalogue::MIGRATOR,
                &acebau_part::MIGRATOR,
                &acebau_inventory::MIGRATOR,
                &acebau_recipe::MIGRATOR,
                &acebau_reseller::MIGRATOR,
                &acebau_order::MIGRATOR,
                &acebau_production::MIGRATOR,
                &acebau_invoice::MIGRATOR,
                &acebau_finance::MIGRATOR,
                &acebau_files::MIGRATOR,
                &acebau_settings::MIGRATOR,
            ],
            MigrationOptions::default(),
        )
        .await?;

    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(state.database)
        .finish();

    Ok(Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/", get(handlers::graphql::handler).post(handlers::graphql::handler))
        .with_state(schema))
}
