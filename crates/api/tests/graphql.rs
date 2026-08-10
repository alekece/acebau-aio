use acebau_api::{AppState, Client, router};
use acebau_database::Database;
use acebau_machine::{MachineModelChangeset, MachineModelInput};
use acebau_unit::{Power, Price, Time};
use axum::serve;
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::net::TcpListener;
use url::Url;

#[sqlx::test]
#[ignore = "requires the Docker Compose PostgreSQL service"]
async fn machine_models_can_be_created_read_updated_and_deleted(pool: PgPool) {
    let app = router(AppState::new(Database::new(pool)))
        .await
        .expect("database migrations should succeed");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("test server should bind");
    let address = listener.local_addr().expect("test server should have an address");
    let server = tokio::spawn(async move {
        serve(listener, app).await.expect("test server should run");
    });
    let client = Client::new(Url::parse(&format!("http://{address}/")).expect("test server URL should parse"));
    let prefix = format!("graphql-test-{}", std::process::id());
    let input = |name: &str| MachineModelInput {
        brand: prefix.clone(),
        name: name.to_owned(),
        purchase_cost: Price::new(100.0),
        maintenance_cost: Price::new(5.0) / Time::from_hours(1.0),
        lifetime: Time::from_hours(10.0),
        average_power: Power::from_watts(100.0),
    };

    let first = client
        .create_machine_model(input("one"))
        .await
        .expect("model should be created");
    let second = client
        .create_machine_model(input("two"))
        .await
        .expect("model should be created");
    let third = client
        .create_machine_model(input("three"))
        .await
        .expect("model should be created");
    assert_eq!(first.name, "one");
    assert_eq!(third.name, "three");

    let fetched = client
        .get_machine_model(&second.id)
        .await
        .expect("model should be readable");
    assert_eq!(fetched.id, second.id);
    assert_eq!(fetched.name, second.name);

    let updated = client
        .update_machine_model(&second.id, input("updated"))
        .await
        .expect("model should be updated");
    assert_eq!(updated.name, "updated");

    let patched = client
        .patch_machine_model(
            &second.id,
            MachineModelChangeset {
                name: Some("patched".to_owned()),
                ..Default::default()
            },
        )
        .await
        .expect("model should be patched");
    assert_eq!(patched.name, "patched");

    assert!(
        client
            .delete_machine_model(&third.id)
            .await
            .expect("model should be deleted")
    );

    let models = client.list_machine_models().await.expect("models should be listed");
    assert!(models.iter().any(|model| model.id == first.id));
    assert!(models.iter().any(|model| model.name == "patched"));
    assert!(!models.iter().any(|model| model.id == third.id));

    server.abort();
}

#[sqlx::test]
#[ignore = "requires the Docker Compose PostgreSQL service"]
async fn business_modules_are_composed_and_paginated(pool: PgPool) {
    let app = router(AppState::new(Database::new(pool)))
        .await
        .expect("database migrations should succeed");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("test server should bind");
    let address = listener.local_addr().expect("test server should have an address");
    let server = tokio::spawn(async move {
        serve(listener, app).await.expect("test server should run");
    });
    let endpoint = format!("http://{address}/");
    let http = reqwest::Client::new();
    let unique_name = format!("catalogue-test-{}", std::process::id());

    let created = graphql(
        &http,
        &endpoint,
        r#"mutation($input: ProductInput!) {
            createProduct(input: $input) { id name collection category }
        }"#,
        json!({"input": {
            "name": unique_name,
            "collection": "Tests",
            "category": "Fixtures",
            "shortDescription": "Created by integration test",
            "customizable": false
        }}),
    )
    .await;
    assert!(created["createProduct"]["id"].is_string());

    let piece = graphql(
        &http,
        &endpoint,
        r#"mutation($input: PrintedPieceInput!) {
            createPrintedPiece(input: $input) { id reference }
        }"#,
        json!({"input": {
            "name": "Integration vase",
            "reference": format!("PIECE-{}", std::process::id())
        }}),
    )
    .await;
    let supply = graphql(
        &http,
        &endpoint,
        r#"mutation($input: SupplyInput!) {
            createSupply(input: $input) { id reference }
        }"#,
        json!({"input": {
            "name": "Integration blue filament",
            "reference": format!("FILAMENT-{}", std::process::id()),
            "kind": "filament",
            "baseUnit": "g",
            "availableQuantity": 1000.0,
            "lowStockThreshold": 100.0,
            "targetQuantity": 500.0
        }}),
    )
    .await;
    let production = graphql(
        &http,
        &endpoint,
        r#"mutation($input: CreateProductionInput!) {
            createProduction(input: $input) { id reference }
        }"#,
        json!({"input": {
            "linkedOrderReference": null,
            "deadline": "2026-08-20",
            "planningPreference": "quality",
            "lines": [{
                "pieceId": piece["createPrintedPiece"]["id"],
                "filamentSupplyId": supply["createSupply"]["id"],
                "quantity": 4
            }]
        }}),
    )
    .await;
    assert!(
        production["createProduction"]["reference"]
            .as_str()
            .is_some_and(|reference| reference.starts_with("PRD-"))
    );

    let machine_model = graphql(
        &http,
        &endpoint,
        r#"mutation($input: MachineModelInput!) {
            createMachineModel(input: $input) { id }
        }"#,
        json!({"input": {
            "brand": "Integration",
            "name": format!("Production model {}", std::process::id()),
            "purchaseCost": "100",
            "maintenanceCost": "5/1h",
            "lifetime": "10h",
            "averagePower": "100W"
        }}),
    )
    .await;
    let machine = graphql(
        &http,
        &endpoint,
        r#"mutation($input: MachineInput!) {
            createMachine(input: $input) { id }
        }"#,
        json!({"input": {
            "modelId": machine_model["createMachineModel"]["id"],
            "surname": format!("Production printer {}", std::process::id()),
            "printingTime": "0h",
            "state": "available"
        }}),
    )
    .await;

    let listed = graphql(
        &http,
        &endpoint,
        r#"query {
            products(page: 1, pageSize: 25) { id name }
            supplies(page: 1, pageSize: 25) { id reference }
            productionTaskLines(page: 1, pageSize: 25) {
                id productionTaskId pieceId filamentSupplyId quantity machineId state startedAt
            }
            machines(page: 1, pageSize: 25) { id surname }
            applicationSettings { defaultTimeUnit defaultMassUnit defaultPageSize }
            operationalOverview { activeProducts activeVariants openOrders pendingProductions }
        }"#,
        json!({}),
    )
    .await;
    assert!(
        listed["products"]
            .as_array()
            .expect("products should be a list")
            .iter()
            .any(|product| product["name"] == unique_name)
    );
    assert_eq!(listed["applicationSettings"][0]["defaultPageSize"], 10);
    assert!(
        listed["supplies"]
            .as_array()
            .is_some_and(|supplies| !supplies.is_empty())
    );
    assert!(
        listed["productionTaskLines"]
            .as_array()
            .expect("production lines should be a list")
            .iter()
            .any(|line| line["productionTaskId"] == production["createProduction"]["id"] && line["quantity"] == 4)
    );
    assert!(listed["machines"].is_array());
    assert!(listed["operationalOverview"]["activeProducts"].as_i64().unwrap() >= 1);

    let line = listed["productionTaskLines"]
        .as_array()
        .expect("production lines should be a list")
        .iter()
        .find(|line| line["productionTaskId"] == production["createProduction"]["id"])
        .expect("created production line should be listed");
    let line_id = line["id"].clone();
    let updated_job = graphql(
        &http,
        &endpoint,
        r#"mutation($id: UUID!, $input: UpdateProductionJobInput!) {
            updateProductionJob(id: $id, input: $input) { id machineId state }
        }"#,
        json!({
            "id": line_id,
            "input": {
                "pieceId": line["pieceId"],
                "filamentSupplyId": line["filamentSupplyId"],
                "machineId": machine["createMachine"]["id"],
                "quantity": 4
            }
        }),
    )
    .await;
    assert_eq!(updated_job["updateProductionJob"]["state"], "to_print");

    let started_job = graphql(
        &http,
        &endpoint,
        r#"mutation($id: UUID!) {
            startProductionJob(id: $id) { id state startedAt }
        }"#,
        json!({"id": line_id}),
    )
    .await;
    assert_eq!(started_job["startProductionJob"]["state"], "printing");
    assert!(started_job["startProductionJob"]["startedAt"].is_string());

    let completed_job = graphql(
        &http,
        &endpoint,
        r#"mutation($id: UUID!) {
            completeProductionJob(id: $id) { id state }
        }"#,
        json!({"id": line_id}),
    )
    .await;
    assert_eq!(completed_job["completeProductionJob"]["state"], "done");

    let completed_task = graphql(
        &http,
        &endpoint,
        r#"query { productionTasks(pageSize: 25) { id state } }"#,
        json!({}),
    )
    .await;
    assert!(
        completed_task["productionTasks"]
            .as_array()
            .expect("production tasks should be a list")
            .iter()
            .any(|task| task["id"] == production["createProduction"]["id"] && task["state"] == "completed")
    );

    let invalid_page = http
        .post(&endpoint)
        .json(&json!({"query": "query { products(pageSize: 12) { id } }"}))
        .send()
        .await
        .expect("GraphQL request should succeed")
        .json::<Value>()
        .await
        .expect("GraphQL response should decode");
    assert!(invalid_page["errors"].is_array());

    server.abort();
}

async fn graphql(http: &reqwest::Client, endpoint: &str, query: &str, variables: Value) -> Value {
    let body = http
        .post(endpoint)
        .json(&json!({"query": query, "variables": variables}))
        .send()
        .await
        .expect("GraphQL request should succeed")
        .json::<Value>()
        .await
        .expect("GraphQL response should decode");

    if let Some(errors) = body.get("errors") {
        panic!("GraphQL request failed: {errors}");
    }

    body["data"].clone()
}
