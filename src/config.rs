/*use std::convert::Infallible;

use anyhow::Context;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl, basic::BasicClient, RedirectUrl};

#[derive(Debug, clap::Parser)]
pub struct Config {
    /// The GitHub OAuth2 client ID
    #[clap(
        long = "github-client-id",
        value_parser = parse_client_id,
        env = "GITHUB_CLIENT_ID"
    )]
    github_client_id: ClientId,

    /// The GitHub OAuth2 client secret
    #[clap(
        long = "github-client-secret",
        value_parser = parse_client_secret,
        env = "GITHUB_CLIENT_SECRET"
    )]
    github_client_secret: ClientSecret,

    /// The GitHub OAuth2 authorization URL
    #[clap(
        long = "github-auth-url",
        value_parser = parse_auth_url,
        env = "GITHUB_AUTH_URL",
        default_value = "https://github.com/login/oauth/authorize"
    )]
    github_auth_url: AuthUrl,

    /// The GitHub OAuth2 token URL
    #[clap(
        long = "github-token-url",
        value_parser = parse_token_url,
        env = "GITHUB_TOKEN_URL",
        default_value = "https://github.com/login/oauth/access_token"
    )]
    github_token_url: TokenUrl,

    #[clap(
        long = "redirect-url",
        value_parser = parse_redirect_url,
        env = "REDIRECT_URL",
    )]
    redirect_url: RedirectUrl,
}

impl Config {
    /// Create a new GitHub OAuth2 client
    pub fn github_client(&self) -> BasicClient {
        BasicClient::new(
            self.github_client_id.clone(),
            Some(self.github_client_secret.clone()),
            self.github_auth_url.clone(),
            Some(self.github_token_url.clone()),
        )
        .set_redirect_uri(self.redirect_url.clone())
    }
}

fn parse_client_id(s: &str) -> Result<ClientId, Infallible> {
    Ok(ClientId::new(s.to_string()))
}

fn parse_client_secret(s: &str) -> Result<ClientSecret, Infallible> {
    Ok(ClientSecret::new(s.to_string()))
}

fn parse_auth_url(s: &str) -> anyhow::Result<AuthUrl> {
    AuthUrl::new(s.to_string()).context("failed to parse auth URL")
}

fn parse_token_url(s: &str) -> anyhow::Result<TokenUrl> {
    TokenUrl::new(s.to_string()).context("failed to parse token URL")
}

fn parse_redirect_url(s: &str) -> anyhow::Result<RedirectUrl> {
    RedirectUrl::new(s.to_string()).context("failed to parse redirect URL")
}*/
