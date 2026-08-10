use acebau_database::Record;
use acebau_machine::{MachineModel, MachineModelChangeset, MachineModelInput};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::{Error as JsonError, Value, json};
use snafu::Snafu;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum ClientError {
    #[snafu(display("API transport failed: {source}"))]
    Transport { source: reqwest::Error },
    #[snafu(display("API returned HTTP status {status}"))]
    Http { status: StatusCode },
    #[snafu(display("GraphQL request failed: {message}"))]
    Graphql { message: String },
    #[snafu(display("API response could not be decoded: {source}"))]
    Decode { source: JsonError },
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    url: Url,
}

impl Client {
    pub fn new(url: Url) -> Self {
        Self {
            http: reqwest::Client::new(),
            url,
        }
    }

    pub async fn create_machine_model(&self, input: MachineModelInput) -> Result<Record<MachineModel>, ClientError> {
        let data = self
            .request(
                r#"
                    mutation($input: MachineModelInput!) {
                        createMachineModel(input: $input) {
                            id status brand name purchase_cost: purchaseCost maintenance_cost: maintenanceCost lifetime average_power: averagePower
                            created_at: createdAt updated_at: updatedAt
                        }
                    }
                "#,
                json!({"input": machine_model_input(input)}),
            )
            .await?;

        self.decode(data, "createMachineModel")
    }

    pub async fn get_machine_model(&self, id: &Uuid) -> Result<Record<MachineModel>, ClientError> {
        let data = self
            .request(
                r#"
                    query($id: String!) {
                        machineModel(id: $id) {
                            id status brand name purchase_cost: purchaseCost maintenance_cost: maintenanceCost lifetime average_power: averagePower
                            created_at: createdAt updated_at: updatedAt
                        }
                    }
                "#,
                json!({"id": id.to_string()}),
            )
            .await?;

        self.decode(data, "machineModel")
    }

    pub async fn list_machine_models(&self) -> Result<Vec<Record<MachineModel>>, ClientError> {
        let data = self
            .request(
                "{ machineModels { id status brand name purchase_cost: purchaseCost maintenance_cost: maintenanceCost lifetime average_power: averagePower created_at: createdAt updated_at: updatedAt } }",
                json!({}),
            )
            .await?;
        self.decode(data, "machineModels")
    }

    pub async fn update_machine_model(
        &self,
        id: &Uuid,
        input: MachineModelInput,
    ) -> Result<Record<MachineModel>, ClientError> {
        let data = self
            .request(
                r#"
                    mutation($id: String!, $input: MachineModelInput!) {
                        updateMachineModel(id: $id, input: $input) {
                            id status brand name purchase_cost: purchaseCost maintenance_cost: maintenanceCost lifetime average_power: averagePower
                            created_at: createdAt updated_at: updatedAt
                        }
                    }
                "#,
                json!({"id": id.to_string(), "input": machine_model_input(input)}),
            )
            .await?;

        self.decode(data, "updateMachineModel")
    }

    pub async fn patch_machine_model(
        &self,
        id: &Uuid,
        changeset: MachineModelChangeset,
    ) -> Result<Record<MachineModel>, ClientError> {
        let data = self
            .request(
                r#"
                    mutation($id: String!, $input: MachineModelChangeset!) {
                        patchMachineModel(id: $id, input: $input) {
                            id status brand name purchase_cost: purchaseCost maintenance_cost: maintenanceCost lifetime average_power: averagePower
                            created_at: createdAt updated_at: updatedAt
                        }
                    }
                "#,
                json!({"id": id.to_string(), "input": machine_model_changeset(changeset)}),
            )
            .await?;

        self.decode(data, "patchMachineModel")
    }

    pub async fn delete_machine_model(&self, id: &Uuid) -> Result<bool, ClientError> {
        let data = self
            .request(
                r#"
                    mutation($id: String!) {
                        deleteMachineModel(id: $id)
                    }
                "#,
                json!({"id": id.to_string()}),
            )
            .await?;

        self.decode(data, "deleteMachineModel")
    }

    async fn request(&self, query: &str, variables: Value) -> Result<Value, ClientError> {
        let response = self
            .http
            .post(self.url.clone())
            .json(&json!({"query": query, "variables": variables}))
            .send()
            .await
            .map_err(|source| ClientError::Transport { source })?;
        let status = response.status();
        let body: Value = response
            .json()
            .await
            .map_err(|source| ClientError::Transport { source })?;

        if status != StatusCode::OK {
            return Err(ClientError::Http { status });
        }
        if let Some(errors) = body.get("errors") {
            return Err(ClientError::Graphql {
                message: errors.to_string(),
            });
        }

        Ok(body["data"].clone())
    }

    fn decode<T: DeserializeOwned>(&self, data: Value, field: &str) -> Result<T, ClientError> {
        serde_json::from_value(data[field].clone()).map_err(|source| ClientError::Decode { source })
    }
}

fn machine_model_input(input: MachineModelInput) -> Value {
    json!({
        "brand": input.brand,
        "name": input.name,
        "purchaseCost": input.purchase_cost,
        "maintenanceCost": input.maintenance_cost,
        "lifetime": input.lifetime,
        "averagePower": input.average_power,
    })
}

fn machine_model_changeset(changeset: MachineModelChangeset) -> Value {
    json!({
        "status": changeset.status,
        "brand": changeset.brand,
        "name": changeset.name,
        "purchaseCost": changeset.purchase_cost,
        "maintenanceCost": changeset.maintenance_cost,
        "lifetime": changeset.lifetime,
        "averagePower": changeset.average_power,
    })
}
