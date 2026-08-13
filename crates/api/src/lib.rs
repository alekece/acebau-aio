#![allow(clippy::pedantic)]
#![recursion_limit = "256"]

mod handlers;

use acebau_activity::{ActivityMutation, ActivityQuery, MIGRATOR as ACTIVITY_MIGRATOR};
use acebau_analytics::AnalyticsQuery;
use acebau_catalogue::{MIGRATOR as CATALOGUE_MIGRATOR, ProductMutation, ProductQuery, VariantMutation, VariantQuery};
use acebau_database::{Database, DatabaseError, MIGRATOR, MigrationOptions};
use acebau_files::{FileMetadataMutation, FileMetadataQuery, MIGRATOR as FILES_MIGRATOR};
use acebau_finance::{ExpenseMutation, ExpenseQuery, MIGRATOR as FINANCE_MIGRATOR};
use acebau_inventory::{
    FilamentSpoolMutation, FilamentSpoolQuery, MIGRATOR as INVENTORY_MIGRATOR, SupplyMutation, SupplyQuery,
};
use acebau_invoice::{ImportedInvoiceMutation, ImportedInvoiceQuery, MIGRATOR as INVOICE_MIGRATOR};
use acebau_machine::{
    MIGRATOR as MACHINE_MIGRATOR, MachineMaintenanceMutation, MachineMaintenanceQuery,
    MachineMaintenanceSettingMutation, MachineMaintenanceSettingQuery, MachineMaintenanceStatusQuery,
    MachineModelMutation, MachineModelQuery, MachineMutation, MachineQuery,
};
use acebau_order::{
    CustomerOrderMutation, CustomerOrderQuery, MIGRATOR as ORDER_MIGRATOR, OrderLineMutation, OrderLineQuery,
};
use acebau_part::{
    MIGRATOR as PART_MIGRATOR, PieceMachineProfileMutation, PieceMachineProfileQuery, PrintedPieceMutation,
    PrintedPieceQuery,
};
use acebau_production::{
    MIGRATOR as PRODUCTION_MIGRATOR, ProductionMutation, ProductionTaskLineQuery, ProductionTaskQuery,
};
use acebau_recipe::{MIGRATOR as RECIPE_MIGRATOR, RecipeItemMutation, RecipeItemQuery};
use acebau_reseller::{MIGRATOR as RESELLER_MIGRATOR, ResellerMutation, ResellerQuery};
use acebau_settings::{ApplicationSettingMutation, ApplicationSettingQuery, MIGRATOR as SETTINGS_MIGRATOR};
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
                &MIGRATOR,
                &MACHINE_MIGRATOR,
                &ACTIVITY_MIGRATOR,
                &CATALOGUE_MIGRATOR,
                &PART_MIGRATOR,
                &INVENTORY_MIGRATOR,
                &RECIPE_MIGRATOR,
                &RESELLER_MIGRATOR,
                &ORDER_MIGRATOR,
                &PRODUCTION_MIGRATOR,
                &INVOICE_MIGRATOR,
                &FINANCE_MIGRATOR,
                &FILES_MIGRATOR,
                &SETTINGS_MIGRATOR,
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
