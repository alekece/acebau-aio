use acebau_api::{AppState, router};
use acebau_database::Database;
use axum::serve;
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::net::TcpListener;

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
    let endpoint = format!("http://{address}/");
    let http = reqwest::Client::new();
    let prefix = format!("graphql-test-{}", std::process::id());
    let metric_input_type = graphql(
        &http,
        &endpoint,
        r#"query {
            __type(name: "PriceMetricInput") {
                inputFields { name type { kind name ofType { name } } }
            }
        }"#,
        json!({}),
    )
    .await;
    assert!(
        metric_input_type["__type"]["inputFields"]
            .as_array()
            .expect("metric input fields should be introspectable")
            .iter()
            .any(|field| field["name"] == "value" && field["type"]["ofType"]["name"] == "Decimal")
    );

    let first = create_machine_model(&http, &endpoint, &prefix, "one").await;
    let second = create_machine_model(&http, &endpoint, &prefix, "two").await;
    let third = create_machine_model(&http, &endpoint, &prefix, "three").await;
    assert_eq!(first["name"], "one");
    assert_eq!(third["name"], "three");

    let fetched = graphql(
        &http,
        &endpoint,
        "query($id: String!) { machineModel(id: $id) { id name } }",
        json!({ "id": second["id"] }),
    )
    .await;
    assert_eq!(fetched["machineModel"]["id"], second["id"]);
    assert_eq!(fetched["machineModel"]["name"], second["name"]);

    let updated = graphql(
        &http,
        &endpoint,
        r#"mutation($id: String!, $input: MachineModelInput!) {
            updateMachineModel(id: $id, input: $input) { id name }
        }"#,
        json!({ "id": second["id"], "input": machine_model_input(&prefix, "updated") }),
    )
    .await;
    assert_eq!(updated["updateMachineModel"]["name"], "updated");

    let patched = graphql(
        &http,
        &endpoint,
        r#"mutation($id: String!, $input: MachineModelChangeset!) {
            patchMachineModel(id: $id, input: $input) { id name }
        }"#,
        json!({ "id": second["id"], "input": { "name": "patched" } }),
    )
    .await;
    assert_eq!(patched["patchMachineModel"]["name"], "patched");

    let deleted = graphql(
        &http,
        &endpoint,
        "mutation($id: String!) { deleteMachineModel(id: $id) }",
        json!({ "id": third["id"] }),
    )
    .await;
    assert_eq!(deleted["deleteMachineModel"], true);

    let listed = graphql(&http, &endpoint, "query { machineModels { id name } }", json!({})).await;
    let models = listed["machineModels"]
        .as_array()
        .expect("machine models should be a list");
    assert!(models.iter().any(|model| model["id"] == first["id"]));
    assert!(models.iter().any(|model| model["name"] == "patched"));
    assert!(!models.iter().any(|model| model["id"] == third["id"]));

    server.abort();
}

fn machine_model_input(brand: &str, name: &str) -> Value {
    json!({
        "brand": brand,
        "name": name,
        "purchaseCost": { "value": "100", "unit": "€" },
        "maintenanceCost": {
            "value": "5",
            "numeratorUnit": "€",
            "denominatorUnit": "h"
        },
        "lifetime": { "value": "10", "unit": "h" },
        "averagePower": { "value": "100", "unit": "W" }
    })
}

async fn create_machine_model(http: &reqwest::Client, endpoint: &str, brand: &str, name: &str) -> Value {
    graphql(
        http,
        endpoint,
        r#"mutation($input: MachineModelInput!) {
            createMachineModel(input: $input) { id name }
        }"#,
        json!({ "input": machine_model_input(brand, name) }),
    )
    .await["createMachineModel"]
        .clone()
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
            "purchaseCost": { "value": "100", "unit": "€" },
            "maintenanceCost": {
                "value": "5",
                "numeratorUnit": "€",
                "denominatorUnit": "h"
            },
            "lifetime": { "value": "10", "unit": "h" },
            "averagePower": { "value": "100", "unit": "W" }
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
            "printingTime": { "value": "0", "unit": "h" },
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
