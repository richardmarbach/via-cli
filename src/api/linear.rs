use color_eyre::{eyre::ContextCompat, Result};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use reqwest::{blocking::Client, header};

use self::get_assigned_issues::GetAssignedIssuesUserAssignedIssuesNodes;

const API_URL: &str = "https://api.linear.app/graphql";

pub struct LinearClient<'a> {
    http_client: Client,
    url: &'a str,
}

impl<'a> LinearClient<'a> {
    pub fn new(api_key: &str) -> Self {
        Self {
            http_client: Client::builder()
                .default_headers(
                    std::iter::once((
                        header::AUTHORIZATION,
                        header::HeaderValue::from_str(&format!("{}", api_key)).unwrap(),
                    ))
                    .collect(),
                )
                .build()
                .unwrap(),
            url: API_URL,
        }
    }

    pub fn current_user_id(&self) -> Result<String> {
        let response_body =
            post_graphql::<GetUserID, _>(&self.http_client, self.url, get_user_id::Variables {})
                .unwrap();
        let response_data: get_user_id::ResponseData =
            response_body.data.context("missing data")?;

        Ok(response_data.viewer.id)
    }

    pub fn assigned_issues(
        &self,
        user_id: &str,
    ) -> Result<Vec<GetAssignedIssuesUserAssignedIssuesNodes>> {
        let variables = get_assigned_issues::Variables {
            user_id: user_id.to_owned(),
        };
        let response_body =
            post_graphql::<GetAssignedIssues, _>(&self.http_client, self.url, variables)?;
        let response_data: get_assigned_issues::ResponseData =
            response_body.data.context("missing data")?;

        Ok(response_data.user.assigned_issues.nodes)
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_user.graphql",
    response_derives = "Debug"
)]
pub struct GetUserID;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/get_assigned_issues.graphql",
    response_derives = "Serialize, Debug"
)]
pub struct GetAssignedIssues;
