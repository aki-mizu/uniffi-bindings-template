use reqwest::blocking::Client;
use anyhow::*;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use core::result::Result::Ok;
#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql"
)]
struct RepoView;

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

pub fn get_repo_info() -> Result<String, anyhow::Error> {

    let github_api_token = "YOUR GITHUB API TOKEN";

    let (owner, name) = parse_repo_name(&"facebook/graphql").unwrap_or(("tomhoule", "graphql-client"));

    let variables = repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<RepoView, _>(&client, "https://api.github.com/graphql", variables).unwrap();

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars: Option<i64> = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count);
    return Ok (format!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }

    #[test]
    fn test_get_repo_info() {
        let result = get_repo_info();
        match result {
            Ok(info) => {
                print!("{}", info);
                assert!(!info.is_empty());
            }
            Err(e) => {
                let error = format!("{e}");
                print!("{}", error);
            }
        }
    }
}
