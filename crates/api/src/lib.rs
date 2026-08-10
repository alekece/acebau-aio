#![allow(clippy::pedantic)]
#![recursion_limit = "256"]

mod client;
mod handlers;

use acebau_activity::{ActivityMutation, ActivityQuery, MIGRATOR as ACTIVITY_MIGRATOR};
use acebau_analytics::AnalyticsQuery;
use acebau_catalogue::{MIGRATOR as CATALOGUE_MIGRATOR, ProductMutation, ProductQuery, VariantMutation, VariantQuery};
use acebau_database::{Database, DatabaseError, FetchOptions, MIGRATOR, MigrationOptions, Record, Repository, Status};
use acebau_files::{FileMetadataMutation, FileMetadataQuery, MIGRATOR as FILES_MIGRATOR};
use acebau_finance::{ExpenseMutation, ExpenseQuery, MIGRATOR as FINANCE_MIGRATOR};
use acebau_inventory::{
    FilamentSpoolMutation, FilamentSpoolQuery, MIGRATOR as INVENTORY_MIGRATOR, SupplyMutation, SupplyQuery,
};
use acebau_invoice::{ImportedInvoiceMutation, ImportedInvoiceQuery, MIGRATOR as INVOICE_MIGRATOR};
use acebau_machine::{
    MIGRATOR as MACHINE_MIGRATOR, Machine, MachineModel, MachineModelMutation, MachineModelQuery, MachineMutation,
    MachineState,
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
use acebau_unit::Time;
use async_graphql::{Context, EmptySubscription, MergedObject, Object, Schema, SimpleObject};
use axum::{Router, routing::get};
use uuid::Uuid;

pub use client::{Client, ClientError};

#[derive(MergedObject, Default)]
struct QueryRoot(
    ProductQuery,
    VariantQuery,
    PrintedPieceQuery,
    PieceMachineProfileQuery,
    RecipeItemQuery,
    MachineViewQuery,
    MachineModelQuery,
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

#[derive(Debug, SimpleObject)]
struct MachineView {
    id: Uuid,
    status: Status,
    model_id: Uuid,
    surname: String,
    printing_time: Time,
    state: MachineState,
    model: Record<MachineModel>,
}

#[derive(Default)]
struct MachineViewQuery;

#[Object]
impl MachineViewQuery {
    async fn machines(
        &self,
        ctx: &Context<'_>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> async_graphql::Result<Vec<MachineView>> {
        let mut database = ctx.data::<Database>()?.clone();
        let page = u32::try_from(page.unwrap_or(1)).map_err(|_| "page number must be greater than zero")?;
        let page_size = u32::try_from(page_size.unwrap_or(10)).map_err(|_| "page size must be positive")?;
        let machines = database
            .repository::<Machine>()
            .fetch_all(FetchOptions::default().with_page(page, page_size)?)
            .await?;
        let mut views = Vec::with_capacity(machines.len());

        for machine in machines {
            let model = database
                .repository::<MachineModel>()
                .fetch_by_id(machine.model_id)
                .await?;
            views.push(MachineView {
                id: machine.id,
                status: machine.status,
                model_id: machine.model_id,
                surname: machine.surname.clone(),
                printing_time: machine.printing_time,
                state: machine.state,
                model,
            });
        }

        Ok(views)
    }
}

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
