use netrc_rs;
use std::{error::Error, fs, string};

use git2::{Remote, Repository};
use thiserror::Error;
use url::{ParseError, Url};

pub fn get_gitlab_remote(origin: &str) -> Result<(String, String), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    let remote = origin;
    let mut remote = repo.find_remote(remote)?;

    let url = remote.url().expect("No remote url was set");
    let parsed_url = parse_remote_to_gitlab_namespace(url)?;

    Ok(parsed_url)

    // parse url and get host url
}

fn parse_remote_to_gitlab_namespace(remote: &str) -> Result<(String, String), Box<dyn Error>> {
    let namespace = match Url::parse(remote) {
        Ok(url) => get_namespace_from_http(url),
        Err(ParseError::RelativeUrlWithoutBase) => get_namespace_from_ssh(remote.to_string()),
        Err(_) => todo!(),
    };

    namespace
}

// Parses a url struct into a tuple containing the domain (gitlab.com) and the projects namespace
// (NiklasTreml/gitlab-tui)
fn get_namespace_from_http(url: Url) -> Result<(String, String), Box<dyn Error>> {
    let domain = url.domain().ok_or(RemoteUrlParseError::InvalidDomain)?;
    let namespace = url
        .path()
        .split_once("/")
        .ok_or(RemoteUrlParseError::InvalidNamespace)?
        .1
        .split_once(".git")
        .ok_or(RemoteUrlParseError::InvalidNamespace)?
        .0;

    Ok((domain.to_string(), namespace.to_string()))
}

fn get_namespace_from_ssh(url: String) -> Result<(String, String), Box<dyn Error>> {
    todo!()
}

#[derive(Debug, Error)]
pub enum RemoteUrlParseError {
    #[error("The provided url was not a valid ssh or https url")]
    NotSshOrHttp,
    #[error("The provided url did not include a valid domain")]
    InvalidDomain,
    #[error("The provided url did not have a valid namespace")]
    InvalidNamespace,
}

#[derive(Debug, Error)]
pub enum NetrcError {
    #[error("Could not find .netrc")]
    NotFound,
    #[error("Invalid netrc")]
    Invalid,
    #[error("Could not find machine for domain in netrc")]
    MachineNotFound,
    #[error("No password set for domain in netrc")]
    NoPassword,
}

// parses .netrc  and tries to find a token for <domain>
pub fn get_token(domain: String) -> Result<String, Box<dyn Error>> {
    let home_dir = std::env::home_dir()
        .ok_or(NetrcError::NotFound)?
        .join(".netrc");
    let netrc: String = fs::read_to_string(home_dir)?.parse()?;

    let res = netrc_rs::Netrc::parse(netrc, false).or(Err(NetrcError::Invalid))?;

    let machine = res
        .machines
        .iter()
        .find(|machine| {
            machine.login == Some("__token__".to_string()) && machine.name == Some(domain.clone())
        })
        .ok_or(NetrcError::MachineNotFound)?;

    let i = machine.password.clone().ok_or(NetrcError::NoPassword);
    Ok(i?)
}
