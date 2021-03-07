#![allow(unused_imports, unused_qualifications, unused_extern_crates)]

use serde::ser::Serializer;
use serde::de::{DeserializeOwned, Deserializer};
use serde_json::value::Value;

use std::cmp::Eq;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::{self, Formatter, Display};
use std::hash::Hash;

use chrono::DateTime;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsBillingUsage {
    /// The sum of the free and paid GitHub Actions minutes used.
    #[serde(rename = "total_minutes_used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_minutes_used: Option<i64>,

    /// The total paid GitHub Actions minutes used.
    #[serde(rename = "total_paid_minutes_used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_paid_minutes_used: Option<i64>,

    /// The amount of free GitHub Actions minutes available.
    #[serde(rename = "included_minutes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub included_minutes: Option<i64>,

    #[serde(rename = "minutes_used_breakdown")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minutes_used_breakdown: Option<ActionsbillingusageMinutesUsedBreakdown>,

}
/// Whether GitHub Actions is enabled on the repository.
pub type ActionsEnabled = bool;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsEnterprisePermissions {
    #[serde(rename = "enabled_organizations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_organizations: Option<EnabledOrganizations>,

    /// The API URL to use to get or set the selected organizations that are allowed to run GitHub Actions, when `enabled_organizations` is set to `selected`.
    #[serde(rename = "selected_organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organizations_url: Option<String>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

    #[serde(rename = "selected_actions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsOrganizationPermissions {
    #[serde(rename = "enabled_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_repositories: Option<EnabledRepositories>,

    /// The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`.
    #[serde(rename = "selected_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

    #[serde(rename = "selected_actions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,

}
/// The public key used for setting Actions Secrets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsPublicKey {
    /// The identifier for the key.
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    /// The Base64 encoded public key.
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsRepositoryPermissions {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<ActionsEnabled>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

    #[serde(rename = "selected_actions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,

}
/// Set secrets for GitHub Actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsSecret {
    /// The name of the secret.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsbillingusageMinutesUsedBreakdown {
    /// Total minutes used on Ubuntu runner machines.
    #[serde(rename = "UBUNTU")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ubuntu: Option<i64>,

    /// Total minutes used on macOS runner machines.
    #[serde(rename = "MACOS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub macos: Option<i64>,

    /// Total minutes used on Windows runner machines.
    #[serde(rename = "WINDOWS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub windows: Option<i64>,

}
/// Actor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "display_login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_login: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

}
/// The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
pub type AlertCreatedAt = DateTime<Utc>;
/// The GitHub URL of the alert resource.
pub type AlertHtmlUrl = String;
/// The REST API URL for fetching the list of instances for an alert.
pub type AlertInstancesUrl = String;
/// The security alert number.
pub type AlertNumber = i32;
/// The REST API URL of the alert resource.
pub type AlertUrl = String;
/// The permissions policy that controls the actions that are allowed to run. Can be one of: `all`, `local_only`, or `selected`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum AllowedActions { 
    #[serde(rename = "all")]
    ALL,
    #[serde(rename = "local_only")]
    LOCAL_ONLY,
    #[serde(rename = "selected")]
    SELECTED,
}

impl Display for AllowedActions {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            AllowedActions::ALL => write!(f, "{}", "all"),
            AllowedActions::LOCAL_ONLY => write!(f, "{}", "local_only"),
            AllowedActions::SELECTED => write!(f, "{}", "selected"),
        }
    }
}

impl std::str::FromStr for AllowedActions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(AllowedActions::ALL),
            "local_only" => Ok(AllowedActions::LOCAL_ONLY),
            "selected" => Ok(AllowedActions::SELECTED),
            _ => Err(()),
        }
    }
}
/// Api Overview
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiOverview {
    #[serde(rename = "verifiable_password_authentication")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verifiable_password_authentication: Option<bool>,

    #[serde(rename = "ssh_key_fingerprints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key_fingerprints: Option<ApioverviewSshKeyFingerprints>,

    #[serde(rename = "hooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks: Option<Vec<String>>,

    #[serde(rename = "web")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub web: Option<Vec<String>>,

    #[serde(rename = "api")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub api: Option<Vec<String>>,

    #[serde(rename = "git")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git: Option<Vec<String>>,

    #[serde(rename = "pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<Vec<String>>,

    #[serde(rename = "importer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub importer: Option<Vec<String>>,

    #[serde(rename = "actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApioverviewSshKeyFingerprints {
    #[serde(rename = "SHA256_RSA")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha256_rsa: Option<String>,

    #[serde(rename = "SHA256_DSA")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha256_dsa: Option<String>,

}
/// The permissions granted to the user-to-server access token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppPermissions {
    /// The level of permission to grant the access token for GitHub Actions workflows, workflow runs, and artifacts. Can be one of: `read` or `write`.
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<String>,

    /// The level of permission to grant the access token for repository creation, deletion, settings, teams, and collaborators creation. Can be one of: `read` or `write`.
    #[serde(rename = "administration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub administration: Option<String>,

    /// The level of permission to grant the access token for checks on code. Can be one of: `read` or `write`.
    #[serde(rename = "checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,

    /// The level of permission to grant the access token for notification of content references and creation content attachments. Can be one of: `read` or `write`.
    #[serde(rename = "content_references")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_references: Option<String>,

    /// The level of permission to grant the access token for repository contents, commits, branches, downloads, releases, and merges. Can be one of: `read` or `write`.
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

    /// The level of permission to grant the access token for deployments and deployment statuses. Can be one of: `read` or `write`.
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<String>,

    /// The level of permission to grant the access token for managing repository environments. Can be one of: `read` or `write`.
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environments: Option<String>,

    /// The level of permission to grant the access token for issues and related comments, assignees, labels, and milestones. Can be one of: `read` or `write`.
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,

    /// The level of permission to grant the access token to search repositories, list collaborators, and access repository metadata. Can be one of: `read` or `write`.
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,

    /// The level of permission to grant the access token for packages published to GitHub Packages. Can be one of: `read` or `write`.
    #[serde(rename = "packages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub packages: Option<String>,

    /// The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds. Can be one of: `read` or `write`.
    #[serde(rename = "pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<String>,

    /// The level of permission to grant the access token for pull requests and related comments, assignees, labels, milestones, and merges. Can be one of: `read` or `write`.
    #[serde(rename = "pull_requests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<String>,

    /// The level of permission to grant the access token to manage the post-receive hooks for a repository. Can be one of: `read` or `write`.
    #[serde(rename = "repository_hooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_hooks: Option<String>,

    /// The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
    #[serde(rename = "repository_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_projects: Option<String>,

    /// The level of permission to grant the access token to view and manage secret scanning alerts. Can be one of: `read` or `write`.
    #[serde(rename = "secret_scanning_alerts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_scanning_alerts: Option<String>,

    /// The level of permission to grant the access token to manage repository secrets. Can be one of: `read` or `write`.
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secrets: Option<String>,

    /// The level of permission to grant the access token to view and manage security events like code scanning alerts. Can be one of: `read` or `write`.
    #[serde(rename = "security_events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_events: Option<String>,

    /// The level of permission to grant the access token to manage just a single file. Can be one of: `read` or `write`.
    #[serde(rename = "single_file")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,

    /// The level of permission to grant the access token for commit statuses. Can be one of: `read` or `write`.
    #[serde(rename = "statuses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<String>,

    /// The level of permission to grant the access token to retrieve Dependabot alerts. Can be one of: `read`.
    #[serde(rename = "vulnerability_alerts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vulnerability_alerts: Option<String>,

    /// The level of permission to grant the access token to update GitHub Actions workflow files. Can be one of: `write`.
    #[serde(rename = "workflows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflows: Option<String>,

    /// The level of permission to grant the access token for organization teams and members. Can be one of: `read` or `write`.
    #[serde(rename = "members")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<String>,

    /// The level of permission to grant the access token to manage access to an organization. Can be one of: `read` or `write`.
    #[serde(rename = "organization_administration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_administration: Option<String>,

    /// The level of permission to grant the access token to manage the post-receive hooks for an organization. Can be one of: `read` or `write`.
    #[serde(rename = "organization_hooks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_hooks: Option<String>,

    /// The level of permission to grant the access token for viewing an organization's plan. Can be one of: `read`.
    #[serde(rename = "organization_plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_plan: Option<String>,

    /// The level of permission to grant the access token to manage organization projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
    #[serde(rename = "organization_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_projects: Option<String>,

    /// The level of permission to grant the access token to manage organization secrets. Can be one of: `read` or `write`.
    #[serde(rename = "organization_secrets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_secrets: Option<String>,

    /// The level of permission to grant the access token to view and manage GitHub Actions self-hosted runners available to an organization. Can be one of: `read` or `write`.
    #[serde(rename = "organization_self_hosted_runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_self_hosted_runners: Option<String>,

    /// The level of permission to grant the access token to view and manage users blocked by the organization. Can be one of: `read` or `write`.
    #[serde(rename = "organization_user_blocking")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_user_blocking: Option<String>,

    /// The level of permission to grant the access token to manage team discussions and related comments. Can be one of: `read` or `write`.
    #[serde(rename = "team_discussions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_discussions: Option<String>,

}
/// The authorization associated with an OAuth Access.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGrant {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<ApplicationgrantApp>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationgrantApp {
    #[serde(rename = "client_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// An artifact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the artifact.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The size in bytes of the artifact.
    #[serde(rename = "size_in_bytes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_in_bytes: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_download_url: Option<String>,

    /// Whether or not the artifact has expired.
    #[serde(rename = "expired")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expired: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditLogEvent {
    /// The time the audit log event occurred, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
    #[serde(rename = "@timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<i64>,

    /// The name of the action that was performed, for example `user.login` or `repo.create`.
    #[serde(rename = "action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "active_was")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_was: Option<bool>,

    /// The actor who performed the action.
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<String>,

    /// The username of the account being blocked.
    #[serde(rename = "blocked_user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blocked_user: Option<String>,

    #[serde(rename = "business")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub business: Option<String>,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<Vec<Value>>,

    #[serde(rename = "config_was")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config_was: Option<Vec<Value>>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,

    /// The time the audit log event was recorded, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,

    #[serde(rename = "deploy_key_fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deploy_key_fingerprint: Option<String>,

    #[serde(rename = "emoji")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emoji: Option<String>,

    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<Value>>,

    #[serde(rename = "events_were")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_were: Option<Vec<Value>>,

    #[serde(rename = "explanation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explanation: Option<String>,

    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(rename = "hook_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hook_id: Option<i64>,

    #[serde(rename = "limited_availability")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limited_availability: Option<bool>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "old_user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub old_user: Option<String>,

    #[serde(rename = "openssh_public_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub openssh_public_key: Option<String>,

    #[serde(rename = "org")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub org: Option<String>,

    #[serde(rename = "previous_visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_visibility: Option<String>,

    #[serde(rename = "read_only")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,

    /// The name of the repository.
    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<String>,

    /// The name of the repository.
    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<String>,

    #[serde(rename = "repository_public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_public: Option<bool>,

    #[serde(rename = "target_login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_login: Option<String>,

    #[serde(rename = "team")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team: Option<String>,

    /// The type of protocol (for example, HTTP or SSH) used to transfer Git data.
    #[serde(rename = "transport_protocol")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transport_protocol: Option<i64>,

    /// A human readable name for the protocol (for example, HTTP or SSH) used to transfer Git data.
    #[serde(rename = "transport_protocol_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transport_protocol_name: Option<String>,

    /// The user that was affected by the action performed (if available).
    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,

    /// The repository visibility, for example `public` or `private`.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

}
/// Authentication Token
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationToken {
    /// The token used for authentication
    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

    /// The time this token expires
    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<HashMap<(), ()>>,

    /// The repositories this token has access to
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

    #[serde(rename = "single_file")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,

    /// Describe whether all repositories have been selected or there's a selection involved
    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

}
/// How the author is associated with the repository.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum AuthorAssociation { 
    #[serde(rename = "COLLABORATOR")]
    COLLABORATOR,
    #[serde(rename = "CONTRIBUTOR")]
    CONTRIBUTOR,
    #[serde(rename = "FIRST_TIMER")]
    FIRST_TIMER,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FIRST_TIME_CONTRIBUTOR,
    #[serde(rename = "MANNEQUIN")]
    MANNEQUIN,
    #[serde(rename = "MEMBER")]
    MEMBER,
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "OWNER")]
    OWNER,
}

impl Display for AuthorAssociation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            AuthorAssociation::COLLABORATOR => write!(f, "{}", "COLLABORATOR"),
            AuthorAssociation::CONTRIBUTOR => write!(f, "{}", "CONTRIBUTOR"),
            AuthorAssociation::FIRST_TIMER => write!(f, "{}", "FIRST_TIMER"),
            AuthorAssociation::FIRST_TIME_CONTRIBUTOR => write!(f, "{}", "FIRST_TIME_CONTRIBUTOR"),
            AuthorAssociation::MANNEQUIN => write!(f, "{}", "MANNEQUIN"),
            AuthorAssociation::MEMBER => write!(f, "{}", "MEMBER"),
            AuthorAssociation::NONE => write!(f, "{}", "NONE"),
            AuthorAssociation::OWNER => write!(f, "{}", "OWNER"),
        }
    }
}

impl std::str::FromStr for AuthorAssociation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COLLABORATOR" => Ok(AuthorAssociation::COLLABORATOR),
            "CONTRIBUTOR" => Ok(AuthorAssociation::CONTRIBUTOR),
            "FIRST_TIMER" => Ok(AuthorAssociation::FIRST_TIMER),
            "FIRST_TIME_CONTRIBUTOR" => Ok(AuthorAssociation::FIRST_TIME_CONTRIBUTOR),
            "MANNEQUIN" => Ok(AuthorAssociation::MANNEQUIN),
            "MEMBER" => Ok(AuthorAssociation::MEMBER),
            "NONE" => Ok(AuthorAssociation::NONE),
            "OWNER" => Ok(AuthorAssociation::OWNER),
            _ => Err(()),
        }
    }
}
/// The authorization for an OAuth app, GitHub App, or a Personal Access Token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "token_last_eight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_last_eight: Option<String>,

    #[serde(rename = "hashed_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hashed_token: Option<String>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<ApplicationgrantApp>,

    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "installation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub installation: Option<ScopedInstallation>,

}
/// The status of auto merging a pull request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMerge {
    #[serde(rename = "enabled_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_by: Option<SimpleUser>,

    /// The merge method to use.
    #[serde(rename = "merge_method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_method: Option<String>,

    /// Title for the merge commit message.
    #[serde(rename = "commit_title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_title: Option<String>,

    /// Commit message for the merge commit.
    #[serde(rename = "commit_message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_message: Option<String>,

}
/// Base Gist
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseGist {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "git_pull_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_pull_url: Option<String>,

    #[serde(rename = "git_push_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_push_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, BasegistFiles>>,

    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<Vec<Value>>,

    #[serde(rename = "history")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub history: Option<Vec<Value>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasegistFiles {
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "raw_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

}
/// Basic Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
/// Blob
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "highlighted_content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub highlighted_content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchAppsUpdateWebhookConfigForApp {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateInstallationAccessToken {
    /// List of repository names that the token should have access to
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,

    /// List of repository IDs that the token should have access to
    #[serde(rename = "repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_ids: Option<Vec<i32>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<AppPermissions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOauthAuthorizationsUpdateAuthorization {
    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// A list of scopes to add to this authorization.
    #[serde(rename = "add_scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_scopes: Option<Vec<String>>,

    /// A list of scopes to remove from this authorization.
    #[serde(rename = "remove_scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_scopes: Option<Vec<String>>,

    /// A note to remind you what the OAuth token is for.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    /// A URL to remind you what app the OAuth token is for.
    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateBlob {
    /// The new blob's content.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    /// The encoding used for `content`. Currently, `\"utf-8\"` and `\"base64\"` are supported.
    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateCommit {
    /// The commit message
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// The SHA of the tree object this commit points to
    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<String>,

    /// The SHAs of the commits that were the parents of this commit. If omitted or empty, the commit will be written as a root commit. For a single parent, an array of one SHA should be provided; for a merge commit, an array of more than one should be provided.
    #[serde(rename = "parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<String>>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepogitcommitsAuthor>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepogitcommitsCommitter>,

    /// The [PGP signature](https://en.wikipedia.org/wiki/Pretty_Good_Privacy) of the commit. GitHub adds the signature to the `gpgsig` header of the created commit. For a commit signature to be verifiable by Git or GitHub, it must be an ASCII-armored detached PGP signature over the string commit as it would be written to the object database. To pass a `signature` parameter, you need to first manually create a valid PGP signature, which can be complicated. You may find it easier to [use the command line](https://git-scm.com/book/id/v2/Git-Tools-Signing-Your-Work) to create signed commits.
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateRef {
    /// The name of the fully qualified reference (ie: `refs/heads/master`). If it doesn't start with 'refs' and have at least two slashes, it will be rejected.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    /// The SHA1 value for this reference.
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGitUpdateRef {
    /// The SHA1 value to set this reference to
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// Indicates whether to force the update or to make sure the update is a fast-forward update. Leaving this out or setting it to `false` will make sure you're not overwriting work.
    #[serde(rename = "force")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub force: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateTag {
    /// The tag's name. This is typically a version (e.g., \"v0.0.1\").
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag: Option<String>,

    /// The tag message.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// The SHA of the git object this is tagging.
    #[serde(rename = "object")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object: Option<String>,

    /// The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "tagger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tagger: Option<ReposownerrepogittagsTagger>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateTree {
    /// Objects (of `path`, `mode`, `type`, and `sha`) specifying a tree structure.
    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<Vec<ReposownerrepogittreesTree>>,

    /// The SHA1 of an existing Git tree object which will be used as the base for the new tree. If provided, a new Git tree object will be created from entries in the Git tree object pointed to by `base_tree` and entries defined in the `tree` parameter. Entries defined in the `tree` parameter will overwrite items from `base_tree` with the same `path`. If you're creating new changes on a branch, then normally you'd set `base_tree` to the SHA1 of the Git tree object of the current latest commit on the branch you're working on. If not provided, GitHub will create a new Git tree object from only the entries defined in the `tree` parameter. If you create a new commit pointing to such a tree, then all files which were a part of the parent commit's tree and were not defined in the `tree` parameter will be listed as deleted by the new commit. 
    #[serde(rename = "base_tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base_tree: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateWebhook {
    /// Use `web` to create a webhook. Default: `web`. This parameter only accepts the value `web`.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<ReposownerrepohooksConfig>,

    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateWebhook {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<ReposownerrepohookshookIdConfig>,

    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. This replaces the entire array of events.
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    /// Determines a list of events to be added to the list of events that the Hook triggers for.
    #[serde(rename = "add_events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_events: Option<Vec<String>>,

    /// Determines a list of events to be removed from the list of events that the Hook triggers for.
    #[serde(rename = "remove_events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_events: Option<Vec<String>>,

    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateWebhookConfigForRepo {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMigrationsStartImport {
    /// The URL of the originating repository.
    #[serde(rename = "vcs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_url: Option<String>,

    /// The originating VCS type. Can be one of `subversion`, `git`, `mercurial`, or `tfvc`. Please be aware that without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
    #[serde(rename = "vcs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,

    /// If authentication is required, the username to provide to `vcs_url`.
    #[serde(rename = "vcs_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_username: Option<String>,

    /// If authentication is required, the password to provide to `vcs_url`.
    #[serde(rename = "vcs_password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_password: Option<String>,

    /// For a tfvc import, the name of the project that is being imported.
    #[serde(rename = "tfvc_project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateContentAttachment {
    /// The title of the attachment
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The body of the attachment
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsUpdateImport {
    /// The username to provide to the originating repository.
    #[serde(rename = "vcs_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_username: Option<String>,

    /// The password to provide to the originating repository.
    #[serde(rename = "vcs_password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_password: Option<String>,

    #[serde(rename = "vcs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,

    #[serde(rename = "tfvc_project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsMapCommitAuthor {
    /// The new Git author email.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// The new Git author name.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "remote_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_id: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsSetLfsPreference {
    /// Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
    #[serde(rename = "use_lfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_lfs: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateInvitation {
    /// The permissions that the associated user will have on the repository. Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreate {
    /// The title of the issue.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<Value>,

    /// The contents of the issue.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is deprecated.**_
    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<String>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Value>,

    /// Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Value>>,

    /// Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateComment {
    /// The contents of the comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForIssueComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue comment.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdate {
    /// The title of the issue.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<Value>,

    /// The contents of the issue.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Login for the user that this issue should be assigned to. **This field is deprecated.**
    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<String>,

    /// State of the issue. Either `open` or `closed`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Value>,

    /// Labels to associate with this issue. Pass one or more Labels to _replace_ the set of Labels on this Issue. Send an empty array (`[]`) to clear all Labels from the Issue. _NOTE: Only users with push access can set labels for issues. Labels are silently dropped otherwise._
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Value>>,

    /// Logins for Users to assign to this issue. Pass one or more user logins to _replace_ the set of assignees on this Issue. Send an empty array (`[]`) to clear all assignees from the Issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesAddAssignees {
    /// Usernames of people to assign this issue to. _NOTE: Only users with push access can add assignees to an issue. Assignees are silently ignored otherwise._
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteIssuesRemoveAssignees {
    /// Usernames of assignees to remove from an issue. _NOTE: Only users with push access can remove assignees from an issue. Assignees are silently ignored otherwise._
    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetGithubActionsPermissionsEnterprise {
    #[serde(rename = "enabled_organizations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_organizations: Option<EnabledOrganizations>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateComment {
    /// The contents of the comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutIssuesSetLabels {
    /// The names of the labels to add to the issue. You can pass an empty array to remove all labels. **Note:** Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesAddLabels {
    /// The name of the label to add to the issue. Must contain at least one label. **Note:** Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key.
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutIssuesLock {
    /// The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:   \\* `off-topic`   \\* `too heated`   \\* `resolved`   \\* `spam`
    #[serde(rename = "lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForIssue {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeployKey {
    /// A name for the key.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The contents of the key.
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    /// If `true`, the key will only be able to read repository contents. Otherwise, the key will be able to read and write.      Deploy keys with write access can perform the same actions as an organization member with admin access, or a collaborator on a personal repository. For more information, see \"[Repository permission levels for an organization](https://help.github.com/articles/repository-permission-levels-for-an-organization/)\" and \"[Permission levels for a user account repository](https://help.github.com/articles/permission-levels-for-a-user-account-repository/).\"
    #[serde(rename = "read_only")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateLabel {
    /// The name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png \":strawberry:\"). For a full list of available emoji and codes, see [emoji-cheat-sheet.com](http://emoji-cheat-sheet.com/).
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    /// A short description of the label.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateLabel {
    /// The new name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png \":strawberry:\"). For a full list of available emoji and codes, see [emoji-cheat-sheet.com](http://emoji-cheat-sheet.com/).
    #[serde(rename = "new_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_name: Option<String>,

    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    /// A short description of the label.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposMerge {
    /// The name of the base branch that the head will be merged into.
    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<String>,

    /// The head to merge. This can be a branch name or a commit SHA1.
    #[serde(rename = "head")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<String>,

    /// Commit message to use for the merge commit. If omitted, a default message will be used.
    #[serde(rename = "commit_message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_message: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateMilestone {
    /// The title of the milestone.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The state of the milestone. Either `open` or `closed`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// A description of the milestone.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "due_on")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_on: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetSelectedOrganizationsEnabledGithubActionsEnterprise {
    /// List of organization IDs to enable for GitHub Actions.
    #[serde(rename = "selected_organization_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organization_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateMilestone {
    /// The title of the milestone.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The state of the milestone. Either `open` or `closed`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// A description of the milestone.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "due_on")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_on: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivityMarkRepoNotificationsAsRead {
    /// Describes the last point that notifications were checked. Anything updated since this time will not be marked as read. If you omit this parameter, all notifications are marked as read. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Default: The current timestamp.
    #[serde(rename = "last_read_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_read_at: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposUpdateInformationAboutPagesSite {
    /// Specify a custom domain for the repository. Sending a `null` value will remove the custom domain. For more about custom domains, see \"[Using a custom domain with GitHub Pages](https://help.github.com/articles/using-a-custom-domain-with-github-pages/).\"
    #[serde(rename = "cname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cname: Option<String>,

    /// Configures access controls for the GitHub Pages site. If public is set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site. This includes anyone in your Enterprise if the repository is set to `internal` visibility. This feature is only available to repositories in an organization on an Enterprise plan.
    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,

    #[serde(rename = "source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<Value>,

}
/// The source branch and directory used to publish your Pages site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreatePagesSite {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<ReposownerrepopagesSource>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForRepo {
    /// The name of the project.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the project.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreate {
    /// The title of the new pull request.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace `head` with a user like this: `username:branch`.
    #[serde(rename = "head")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<String>,

    /// The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository.
    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<String>,

    /// The contents of the pull request.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Indicates whether [maintainers can modify](https://help.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(rename = "maintainer_can_modify")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainer_can_modify: Option<bool>,

    /// Indicates whether the pull request is a draft. See \"[Draft Pull Requests](https://help.github.com/en/articles/about-pull-requests#draft-pull-requests)\" in the GitHub Help documentation to learn more.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    #[serde(rename = "issue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchPullsUpdateReviewComment {
    /// The text of the reply to the review comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForPullRequestReviewComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the pull request review comment.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchPullsUpdate {
    /// The title of the pull request.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The contents of the pull request.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The name of the branch you want your changes pulled into. This should be an existing branch on the current repository. You cannot update the base branch on a pull request to point to another repository.
    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<String>,

    /// Indicates whether [maintainers can modify](https://help.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(rename = "maintainer_can_modify")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainer_can_modify: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReviewComment {
    /// The text of the review comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// The SHA of the commit needing a comment. Not using the latest commit SHA may render your comment outdated if a subsequent commit modifies the line you specify as the `position`.
    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    /// The relative path to the file that necessitates a comment.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// **Required without `comfort-fade` preview**. The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. For help finding the position value, read the note above.
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    /// **Required with `comfort-fade` preview**. In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, side represents whether the last line of the comment range is a deletion or addition. For more information, see \"[Diff view options](https://help.github.com/en/articles/about-comparing-branches-in-pull-requests#diff-view-options)\" in the GitHub Help documentation.
    #[serde(rename = "side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,

    /// **Required with `comfort-fade` preview**. The line of the blob in the pull request diff that the comment applies to. For a multi-line comment, the last line of the range that your comment applies to.
    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

    /// **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_line` is the first line in the pull request diff that your multi-line comment applies to. To learn more about multi-line comments, see \"[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)\" in the GitHub Help documentation.
    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    /// **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see \"[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)\" in the GitHub Help documentation. See `side` in this table for additional context.
    #[serde(rename = "start_side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,

    #[serde(rename = "in_reply_to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminCreateSelfHostedRunnerGroupForEnterprise {
    /// Name of the runner group.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Visibility of a runner group. You can select all organizations or select individual organization. Can be one of: `all` or `selected`
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    /// List of organization IDs that can access the runner group.
    #[serde(rename = "selected_organization_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organization_ids: Option<Vec<i32>>,

    /// List of runner IDs to add to the runner group.
    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReplyForReviewComment {
    /// The text of the review comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsMerge {
    /// Title for the automatic commit message.
    #[serde(rename = "commit_title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_title: Option<String>,

    /// Extra detail to append to automatic commit message.
    #[serde(rename = "commit_message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_message: Option<String>,

    /// SHA that pull request head must match to allow merge.
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// Merge method to use. Possible values are `merge`, `squash` or `rebase`. Default is `merge`.
    #[serde(rename = "merge_method")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_method: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsRequestReviewers {
    /// An array of user `login`s that will be requested.
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<String>>,

    /// An array of team `slug`s that will be requested.
    #[serde(rename = "team_reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_reviewers: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletePullsRemoveRequestedReviewers {
    /// An array of user `login`s that will be removed.
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<String>>,

    /// An array of team `slug`s that will be removed.
    #[serde(rename = "team_reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_reviewers: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReview {
    /// The SHA of the commit that needs a review. Not using the latest commit SHA may render your review comment outdated if a subsequent commit modifies the line you specify as the `position`. Defaults to the most recent commit in the pull request when you do not specify a value.
    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    /// **Required** when using `REQUEST_CHANGES` or `COMMENT` for the `event` parameter. The body text of the pull request review.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    /// Use the following table to specify the location, destination, and contents of the draft review comment.
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<Vec<ReposownerrepopullspullNumberreviewsComments>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateReview {
    /// The body text of the pull request review.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsDismissReview {
    /// The message for the pull request review dismissal
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsSubmitReview {
    /// The body text of the pull request review
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateBranch {
    /// The expected SHA of the pull request's HEAD ref. This is the most recent commit on the pull request's branch. If the expected SHA does not match the pull request's HEAD, you will receive a `422 Unprocessable Entity` status. You can use the \"[List commits](https://docs.github.com/rest/reference/repos#list-commits)\" endpoint to find the most recent commit SHA. Default: SHA of the pull request's current HEAD ref.
    #[serde(rename = "expected_head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expected_head_sha: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateRelease {
    /// The name of the tag.
    #[serde(rename = "tag_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag_name: Option<String>,

    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch (usually `master`).
    #[serde(rename = "target_commitish")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,

    /// The name of the release.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Text describing the contents of the tag.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// `true` to create a draft (unpublished) release, `false` to create a published one.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    /// `true` to identify the release as a prerelease. `false` to identify the release as a full release.
    #[serde(rename = "prerelease")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateSelfHostedRunnerGroupForEnterprise {
    /// Name of the runner group.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Visibility of a runner group. You can select all organizations or select individual organizations. Can be one of: `all` or `selected`
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateReleaseAsset {
    /// The file name of the asset.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// An alternate short description of the asset. Used in place of the filename.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateRelease {
    /// The name of the tag.
    #[serde(rename = "tag_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag_name: Option<String>,

    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch (usually `master`).
    #[serde(rename = "target_commitish")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,

    /// The name of the release.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Text describing the contents of the tag.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// `true` makes the release a draft, and `false` publishes the release.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    /// `true` to identify the release as a prerelease, `false` to identify the release as a full release.
    #[serde(rename = "prerelease")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchSecretScanningUpdateAlert {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<SecretScanningAlertState>,

    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateCommitStatus {
    /// The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The target URL to associate with this status. This URL will be linked from the GitHub UI to allow users to easily see the source of the status.   For example, if your continuous integration system is posting build status, you would want to provide the deep link for the build output for this specific SHA:   `http://ci.example.com/user/repo/build/sha`
    #[serde(rename = "target_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,

    /// A short description of the status.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// A string label to differentiate this status from the status of other systems. This field is case-insensitive.
    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivitySetRepoSubscription {
    /// Determines if notifications should be received from this repository.
    #[serde(rename = "subscribed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribed: Option<bool>,

    /// Determines if all notifications should be blocked from this repository.
    #[serde(rename = "ignored")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposReplaceAllTopics {
    /// An array of topics to add to the repository. Pass one or more topics to _replace_ the set of existing topics. Send an empty array (`[]`) to clear all topics from the repository. **Note:** Topic `names` cannot contain uppercase letters.
    #[serde(rename = "names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposTransfer {
    /// The username or organization name the repository will be transferred to.
    #[serde(rename = "new_owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_owner: Option<String>,

    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateUsingTemplate {
    /// The organization or person who will own the new repository. To create a new repository in an organization, the authenticated user must be a member of the specified organization.
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<String>,

    /// The name of the new repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// A short description of the new repository.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Set to `true` to include the directory structure and files from all branches in the template repository, and not just the default branch. Default: `false`.
    #[serde(rename = "include_all_branches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_all_branches: Option<bool>,

    /// Either `true` to create a new private repository or `false` to create a new public one.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateEnvironmentSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an environment public key](https://docs.github.com/rest/reference/actions#get-an-environment-public-key) endpoint.
    #[serde(rename = "encrypted_value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,

    /// ID of the key you used to encrypt the secret.
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminProvisionAndInviteEnterpriseGroup {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// The name of the SCIM group. This must match the GitHub organization that the group maps to.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<Scimv2enterprisesenterpriseGroupsMembers>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetOrgAccessToSelfHostedRunnerGroupInEnterprise {
    /// List of organization IDs that can access the runner group.
    #[serde(rename = "selected_organization_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organization_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetInformationForProvisionedEnterpriseGroup {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// The name of the SCIM group. This must match the GitHub organization that the group maps to.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<Scimv2enterprisesenterpriseGroupsMembers>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateAttributeForEnterpriseGroup {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operations: Option<Vec<Scimv2enterprisesenterpriseGroupsscimGroupIdOperations>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminProvisionAndInviteEnterpriseUser {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// The username for the user.
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<Scimv2enterprisesenterpriseUsersName>,

    /// List of user emails.
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Scimv2enterprisesenterpriseUsersEmails>>,

    /// List of SCIM group IDs the user is a member of.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetInformationForProvisionedEnterpriseUser {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// The username for the user.
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<Scimv2enterprisesenterpriseUsersName>,

    /// List of user emails.
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Scimv2enterprisesenterpriseUsersEmails>>,

    /// List of SCIM group IDs the user is a member of.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateAttributeForEnterpriseUser {
    /// The SCIM schema URIs.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operations: Option<Vec<Value>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimProvisionAndInviteUser {
    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<Scimv2organizationsorgUsersName>,

    /// user emails
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Scimv2organizationsorgUsersEmails>>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<String>>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimSetInformationForProvisionedUser {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<String>>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<Scimv2organizationsorgUsersName>,

    /// user emails
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Scimv2organizationsorgUsersscimUserIdEmails>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimUpdateAttributeForUser {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// Set of operations to be performed
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operations: Option<Vec<Scimv2organizationsorgUsersscimUserIdOperations>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateLegacy {
    /// The name of the team.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the team.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    /// The ID of a team to set as the parent team.
    #[serde(rename = "parent_team_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionLegacy {
    /// The discussion post's title.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The discussion post's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Private posts are only visible to team members, organization owners, and team maintainers. Public posts are visible to all members of the organization. Set to `true` to create a private post.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetSelfHostedRunnersInGroupForEnterprise {
    /// List of runner IDs to add to the runner group.
    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionLegacy {
    /// The discussion post's title.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The discussion post's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionCommentLegacy {
    /// The discussion comment's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionCommentLegacy {
    /// The discussion comment's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionCommentLegacy {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionLegacy {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserLegacy {
    /// The role that this user should have in the team. Can be one of:   \\* `member` - a normal member of the team.   \\* `maintainer` - a team maintainer. Able to add/remove other team members, promote other team members to team maintainer, and edit the team's name and description.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsLegacy {
    /// The permission to grant to the team for this project. Can be one of:   \\* `read` - team members can read, but not write to or administer this project.   \\* `write` - team members can read and write, but not administer this project.   \\* `admin` - team members can read, write and administer this project.   Default: the team's `permission` attribute will be used to determine what permission to grant the team on this project. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs).\"
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateRepoPermissionsLegacy {
    /// The permission to grant the team on this repository. Can be one of:   \\* `pull` - team members can pull, but not push to or administer this repository.   \\* `push` - team members can pull and push, but not administer this repository.   \\* `admin` - team members can pull, push and administer this repository.      If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsCreateOrUpdateIdpGroupConnectionsLegacy {
    /// The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<TeamsteamIdteamsyncgroupmappingsGroups>>,

    #[serde(rename = "synced_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub synced_at: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchUsersUpdateAuthenticated {
    /// The new name of the user.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The publicly visible email address of the user.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// The new blog URL of the user.
    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

    /// The new Twitter username of the user.
    #[serde(rename = "twitter_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,

    /// The new company of the user.
    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    /// The new location of the user.
    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    /// The new hiring availability of the user.
    #[serde(rename = "hireable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,

    /// The new short biography of the user.
    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGistsCreate {
    /// Description of the gist
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Names and content for the files that make up the gist
    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, GistsFiles>>,

    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<Value>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchUsersSetPrimaryEmailVisibilityForAuthenticated {
    /// An email address associated with the GitHub user account to manage.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// Denotes whether an email is publically visible.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostUsersAddEmailForAuthenticated {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteUsersDeleteEmailForAuthenticated {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostUsersCreateGpgKeyForAuthenticated {
    /// A GPG key in ASCII-armored format.
    #[serde(rename = "armored_public_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub armored_public_key: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostUsersCreatePublicSshKeyForAuthenticated {
    /// A descriptive name for the new key.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The public SSH key to add to your GitHub account.
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateMembershipForAuthenticatedUser {
    /// The state that the membership should be in. Only `\"active\"` will be accepted.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMigrationsStartForAuthenticatedUser {
    /// Lock the repositories being migrated at the start of the migration
    #[serde(rename = "lock_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_repositories: Option<bool>,

    /// Do not include attachments in the migration
    #[serde(rename = "exclude_attachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_attachments: Option<bool>,

    /// Exclude attributes from the API response to improve performance
    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<String>>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForAuthenticatedUser {
    /// Name of the project
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Body of the project
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateForAuthenticatedUser {
    /// The name of the repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// A short description of the repository.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// A URL with more information about the repository.
    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    /// Whether the repository is private.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    /// Whether issues are enabled.
    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    /// Whether projects are enabled.
    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    /// Whether the wiki is enabled.
    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(rename = "team_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_id: Option<i64>,

    /// Whether the repository is initialized with a minimal README.
    #[serde(rename = "auto_init")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_init: Option<bool>,

    /// The desired language or platform to apply to the .gitignore.
    #[serde(rename = "gitignore_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gitignore_template: Option<String>,

    /// The license keyword of the open source license for this repository.
    #[serde(rename = "license_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license_template: Option<String>,

    /// Whether to allow squash merges for pull requests.
    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    /// Whether to allow merge commits for pull requests.
    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    /// Whether to allow rebase merges for pull requests.
    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    /// Whether to delete head branches when pull requests are merged
    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGistsUpdate {
    /// Description of the gist
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Names of files to be updated
    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, GistsgistIdFiles>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteAppsDeleteAuthorization {
    /// The OAuth access token used to authenticate to the GitHub API.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGistsCreateComment {
    /// The comment text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGistsUpdateComment {
    /// The comment text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMarkdownRender {
    /// The Markdown text to render in HTML.
    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

    /// The rendering mode.
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,

    /// The repository context to use when creating references in `gfm` mode.
    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivityMarkNotificationsAsRead {
    /// Describes the last point that notifications were checked.
    #[serde(rename = "last_read_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether the notification has been read.
    #[serde(rename = "read")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivitySetThreadSubscription {
    /// Whether to block all notifications from a thread.
    #[serde(rename = "ignored")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdate {
    /// Billing email address. This address is not publicized.
    #[serde(rename = "billing_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_email: Option<String>,

    /// The company name.
    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    /// The publicly visible email address.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// The Twitter username of the company.
    #[serde(rename = "twitter_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,

    /// The location.
    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    /// The shorthand name of the company.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the company.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Toggles whether an organization can use organization projects.
    #[serde(rename = "has_organization_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_organization_projects: Option<bool>,

    /// Toggles whether repositories that belong to the organization can use repository projects.
    #[serde(rename = "has_repository_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_repository_projects: Option<bool>,

    /// Default permission level members have for organization repositories:   \\* `read` - can pull, but not push to or administer this repository.   \\* `write` - can pull and push, but not administer this repository.   \\* `admin` - can pull, push, and administer this repository.   \\* `none` - no permissions granted by default.
    #[serde(rename = "default_repository_permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_repository_permission: Option<String>,

    /// Toggles the ability of non-admin organization members to create repositories. Can be one of:   \\* `true` - all organization members can create repositories.   \\* `false` - only organization owners can create repositories.   Default: `true`   **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details. **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details.
    #[serde(rename = "members_can_create_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_repositories: Option<bool>,

    /// Toggles whether organization members can create internal repositories, which are visible to all enterprise members. You can only allow members to create internal repositories if your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. Can be one of:   \\* `true` - all organization members can create internal repositories.   \\* `false` - only organization owners can create internal repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(rename = "members_can_create_internal_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,

    /// Toggles whether organization members can create private repositories, which are visible to organization members with permission. Can be one of:   \\* `true` - all organization members can create private repositories.   \\* `false` - only organization owners can create private repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(rename = "members_can_create_private_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,

    /// Toggles whether organization members can create public repositories, which are visible to anyone. Can be one of:   \\* `true` - all organization members can create public repositories.   \\* `false` - only organization owners can create public repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(rename = "members_can_create_public_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,

    /// Specifies which types of repositories non-admin organization members can create. Can be one of:   \\* `all` - all organization members can create public and private repositories.   \\* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.   \\* `none` - only admin members can create repositories.   **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
    #[serde(rename = "members_allowed_repository_creation_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,

    /// Toggles whether organization members can create GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create GitHub Pages sites.   \\* `false` - no organization members can create GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(rename = "members_can_create_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_pages: Option<bool>,

    /// Toggles whether organization members can create public GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create public GitHub Pages sites.   \\* `false` - no organization members can create public GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(rename = "members_can_create_public_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,

    /// Toggles whether organization members can create private GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create private GitHub Pages sites.   \\* `false` - no organization members can create private GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(rename = "members_can_create_private_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,

    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetGithubActionsPermissionsOrganization {
    #[serde(rename = "enabled_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled_repositories: Option<EnabledRepositories>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelectedRepositoriesEnabledGithubActionsOrganization {
    /// List of repository IDs to enable for GitHub Actions.
    #[serde(rename = "selected_repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsCreateSelfHostedRunnerGroupForOrg {
    /// Name of the runner group.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Visibility of a runner group. You can select all repositories, select individual repositories, or limit access to private repositories. Can be one of: `all`, `selected`, or `private`.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    /// List of repository IDs that can access the runner group.
    #[serde(rename = "selected_repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,

    /// List of runner IDs to add to the runner group.
    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchActionsUpdateSelfHostedRunnerGroupForOrg {
    /// Name of the runner group.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Visibility of a runner group. You can select all repositories, select individual repositories, or all private repositories. Can be one of: `all`, `selected`, or `private`.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCheckToken {
    /// The access_token of the OAuth application.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetRepoAccessToSelfHostedRunnerGroupInOrg {
    /// List of repository IDs that can access the runner group.
    #[serde(rename = "selected_repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelfHostedRunnersInGroupForOrg {
    /// List of runner IDs to add to the runner group.
    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateOrgSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an organization public key](https://docs.github.com/rest/reference/actions#get-an-organization-public-key) endpoint.
    #[serde(rename = "encrypted_value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,

    /// ID of the key you used to encrypt the secret.
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    /// Configures the access that repositories have to the organization secret. Can be one of:   \\- `all` - All repositories in an organization can access the secret.   \\- `private` - Private repositories in an organization can access the secret.   \\- `selected` - Only specific repositories can access the secret.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can manage the list of selected repositories using the [List selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#list-selected-repositories-for-an-organization-secret), [Set selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret), and [Remove selected repository from an organization secret](https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(rename = "selected_repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelectedReposForOrgSecret {
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can add and remove individual repositories using the [Set selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret) and [Remove selected repository from an organization secret](https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(rename = "selected_repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOrgsCreateWebhook {
    /// Must be passed as \"web\".
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<OrgsorghooksConfig>,

    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateWebhook {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<OrgsorghookshookIdConfig>,

    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateWebhookConfigForOrg {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOrgsCreateInvitation {
    /// **Required unless you provide `email`**. GitHub user ID for the person you are inviting.
    #[serde(rename = "invitee_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitee_id: Option<i64>,

    /// **Required unless you provide `invitee_id`**. Email address of the person you are inviting, which can be an existing GitHub user.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// Specify role for new member. Can be one of:   \\* `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.   \\* `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.   \\* `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

    /// Specify IDs for the teams you want to invite new members to.
    #[serde(rename = "team_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_ids: Option<Vec<i32>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOrgsSetMembershipForUser {
    /// The role to give the user in the organization. Can be one of:   \\* `admin` - The user will become an owner of the organization.   \\* `member` - The user will become a non-owner member of the organization.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMigrationsStartForOrg {
    /// A list of arrays indicating which repositories should be migrated.
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,

    /// Indicates whether repositories should be locked (to prevent manipulation) while migrating data.
    #[serde(rename = "lock_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_repositories: Option<bool>,

    /// Indicates whether attachments should be excluded from the migration (to reduce migration archive file size).
    #[serde(rename = "exclude_attachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_attachments: Option<bool>,

    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteAppsDeleteToken {
    /// The OAuth access token used to authenticate to the GitHub API.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForOrg {
    /// The name of the project.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the project.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateInOrg {
    /// The name of the repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// A short description of the repository.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// A URL with more information about the repository.
    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    /// Whether the repository is private.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    /// Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see \"[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)\" in the GitHub Help documentation.   The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(rename = "team_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_id: Option<i64>,

    /// Pass `true` to create an initial commit with empty README.
    #[serde(rename = "auto_init")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_init: Option<bool>,

    /// Desired language or platform [.gitignore template](https://github.com/github/gitignore) to apply. Use the name of the template without the extension. For example, \"Haskell\".
    #[serde(rename = "gitignore_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gitignore_template: Option<String>,

    /// Choose an [open source license template](https://choosealicense.com/) that best suits your needs, and then use the [license keyword](https://help.github.com/articles/licensing-a-repository/#searching-github-by-license-type) as the `license_template` string. For example, \"mit\" or \"mpl-2.0\".
    #[serde(rename = "license_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license_template: Option<String>,

    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreate {
    /// The name of the team.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the team.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// List GitHub IDs for organization members who will become team maintainers.
    #[serde(rename = "maintainers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainers: Option<Vec<String>>,

    /// The full name (e.g., \"organization-name/repository-name\") of repositories to add the team to.
    #[serde(rename = "repo_names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo_names: Option<Vec<String>>,

    /// The level of privacy this team should have. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   Default: `secret`   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.   Default for child team: `closed`
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    /// The ID of a team to set as the parent team.
    #[serde(rename = "parent_team_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateInOrg {
    /// The name of the team.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The description of the team.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    /// The ID of a team to set as the parent team.
    #[serde(rename = "parent_team_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionInOrg {
    /// The discussion post's title.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The discussion post's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Private posts are only visible to team members, organization owners, and team maintainers. Public posts are visible to all members of the organization. Set to `true` to create a private post.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionInOrg {
    /// The discussion post's title.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The discussion post's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionCommentInOrg {
    /// The discussion comment's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionCommentInOrg {
    /// The discussion comment's body text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionCommentInOrg {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionInOrg {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchAppsResetToken {
    /// The access_token of the OAuth application.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserInOrg {
    /// The role that this user should have in the team. Can be one of:   \\* `member` - a normal member of the team.   \\* `maintainer` - a team maintainer. Able to add/remove other team members, promote other team members to team maintainer, and edit the team's name and description.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsInOrg {
    /// The permission to grant to the team for this project. Can be one of:   \\* `read` - team members can read, but not write to or administer this project.   \\* `write` - team members can read and write, but not administer this project.   \\* `admin` - team members can read, write and administer this project.   Default: the team's `permission` attribute will be used to determine what permission to grant the team on this project. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs).\"
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateRepoPermissionsInOrg {
    /// The permission to grant the team on this repository. Can be one of:   \\* `pull` - team members can pull, but not push to or administer this repository.   \\* `push` - team members can pull and push, but not administer this repository.   \\* `admin` - team members can pull, push and administer this repository.   \\* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.   \\* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.      If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsCreateOrUpdateIdpGroupConnectionsInOrg {
    /// The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<OrgsorgteamsteamSlugteamsyncgroupmappingsGroups>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateCard {
    /// The project card's note
    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    /// Whether or not the card is archived
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveCard {
    /// The position of the card in a column
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<String>,

    /// The unique identifier of the column the card should be moved to
    #[serde(rename = "column_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub column_id: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateColumn {
    /// Name of the project column
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCard {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveColumn {
    /// The position of the column in a project
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdate {
    /// Name of the project
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Body of the project
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// State of the project; either 'open' or 'closed'
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The baseline permission that all organization members have on this project
    #[serde(rename = "organization_permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,

    /// Whether or not this project can be seen by everyone.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsScopeToken {
    /// **Required.** The OAuth access token used to authenticate to the GitHub API.
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,

    /// The name of the user or organization to scope the user-to-server access token to. **Required** unless `target_id` is specified.
    #[serde(rename = "target")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target: Option<String>,

    /// The ID of the user or organization to scope the user-to-server access token to. **Required** unless `target` is specified.
    #[serde(rename = "target_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<i64>,

    /// The list of repository names to scope the user-to-server access token to. `repositories` may not be specified if `repository_ids` is specified.
    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,

    /// The list of repository IDs to scope the user-to-server access token to. `repository_ids` may not be specified if `repositories` is specified.
    #[serde(rename = "repository_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_ids: Option<Vec<i32>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<AppPermissions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutProjectsAddCollaborator {
    /// The permission to grant the collaborator.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateColumn {
    /// Name of the project column
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdate {
    /// The name of the repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// A short description of the repository.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// A URL with more information about the repository.
    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    /// Either `true` to make the repository private or `false` to make it public. Default: `false`.   **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://help.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private. **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://help.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    /// Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. The `visibility` parameter overrides the `private` parameter when you use both along with the `nebula-preview` preview header.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    /// Updates the default branch for this repository.
    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    /// `true` to archive this repository. **Note**: You cannot unarchive repositories through the API.
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetGithubActionsPermissionsRepository {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<ActionsEnabled>,

    #[serde(rename = "allowed_actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsReviewPendingDeploymentsForRun {
    /// The list of environment ids to approve or reject
    #[serde(rename = "environment_ids")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment_ids: Option<Vec<i32>>,

    /// Whether to approve or reject deployment to the specified environments. Must be one of: `approved` or `rejected`
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// A comment to accompany the deployment review
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateRepoSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get a repository public key](https://docs.github.com/rest/reference/actions#get-a-repository-public-key) endpoint.
    #[serde(rename = "encrypted_value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,

    /// ID of the key you used to encrypt the secret.
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsCreateWorkflowDispatch {
    /// The git reference for the workflow. The reference can be a branch or tag name.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    /// Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted.
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inputs: Option<HashMap<String, String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposUpdateBranchProtection {
    #[serde(rename = "required_status_checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_status_checks: Option<ReposownerrepobranchesbranchprotectionRequiredStatusChecks>,

    /// Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
    #[serde(rename = "enforce_admins")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforce_admins: Option<bool>,

    #[serde(rename = "required_pull_request_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_pull_request_reviews: Option<ReposownerrepobranchesbranchprotectionRequiredPullRequestReviews>,

    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrictions: Option<ReposownerrepobranchesbranchprotectionRestrictions>,

    /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see \"[Requiring a linear commit history](https://help.github.com/github/administering-a-repository/requiring-a-linear-commit-history)\" in the GitHub Help documentation.
    #[serde(rename = "required_linear_history")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<bool>,

    /// Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://help.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.\"
    #[serde(rename = "allow_force_pushes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<bool>,

    /// Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://help.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.
    #[serde(rename = "allow_deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdatePullRequestReviewProtection {
    #[serde(rename = "dismissal_restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions>,

    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(rename = "dismiss_stale_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,

    /// Blocks merging pull requests until [code owners](https://help.github.com/articles/about-code-owners/) have reviewed.
    #[serde(rename = "require_code_owner_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,

    /// Specifies the number of reviewers required to approve pull requests. Use a number between 1 and 6.
    #[serde(rename = "required_approving_review_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateStatusCheckProtection {
    /// Require branches to be up to date before merging.
    #[serde(rename = "strict")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub strict: Option<bool>,

    /// The list of status checks to require in order to merge into this branch
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOauthAuthorizationsCreateAuthorization {
    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// A note to remind you what the OAuth token is for.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    /// A URL to remind you what app the OAuth token is for.
    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

    /// The OAuth app client key for which to create the token.
    #[serde(rename = "client_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,

    /// The OAuth app client secret for which to create the token.
    #[serde(rename = "client_secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,

    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetStatusCheckContexts {
    /// contexts parameter
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddStatusCheckContexts {
    /// contexts parameter
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveStatusCheckContexts {
    /// contexts parameter
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetAppAccessRestrictions {
    /// apps parameter
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddAppAccessRestrictions {
    /// apps parameter
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveAppAccessRestrictions {
    /// apps parameter
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetTeamAccessRestrictions {
    /// teams parameter
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddTeamAccessRestrictions {
    /// teams parameter
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveTeamAccessRestrictions {
    /// teams parameter
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetUserAccessRestrictions {
    /// users parameter
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOauthAuthorizationsGetOrCreateAuthorizationForApp {
    /// The OAuth app client secret for which to create the token.
    #[serde(rename = "client_secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,

    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// A note to remind you what the OAuth token is for.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    /// A URL to remind you what app the OAuth token is for.
    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddUserAccessRestrictions {
    /// users parameter
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveUserAccessRestrictions {
    /// users parameter
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposRenameBranch {
    /// The new name of the branch.
    #[serde(rename = "new_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostChecksCreate {
    /// The name of the check. For example, \"code-coverage\".
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The SHA of the commit.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

    /// The URL of the integrator's site that has the full details of the check. If the integrator does not provide this, then the homepage of the GitHub app is used.
    #[serde(rename = "details_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub details_url: Option<String>,

    /// A reference for the run on the integrator's system.
    #[serde(rename = "external_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<String>,

    /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "completed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,

    #[serde(rename = "output")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output: Option<ReposownerrepocheckrunsOutput>,

    /// Displays a button on GitHub that can be clicked to alert your app to do additional tasks. For example, a code linting app can display a button that automatically fixes detected errors. The button created in this object is displayed after the check run completes. When a user clicks the button, GitHub sends the [`check_run.requested_action` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) to your app. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\" To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\"
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<ReposownerrepocheckrunsActions>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchChecksUpdate {
    /// The name of the check. For example, \"code-coverage\".
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The URL of the integrator's site that has the full details of the check.
    #[serde(rename = "details_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub details_url: Option<String>,

    /// A reference for the run on the integrator's system.
    #[serde(rename = "external_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<String>,

    /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`.   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "completed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,

    #[serde(rename = "output")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output: Option<ReposownerrepocheckrunscheckRunIdOutput>,

    /// Possible further actions the integrator can perform, which a user may trigger. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\"
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<ReposownerrepocheckrunsActions>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostChecksCreateSuite {
    /// The sha of the head commit.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchChecksSetSuitesPreferences {
    /// Enables or disables automatic creation of CheckSuite events upon pushes to the repository. Enabled by default. See the [`auto_trigger_checks` object](https://docs.github.com/rest/reference/checks#auto_trigger_checks-object) description for details.
    #[serde(rename = "auto_trigger_checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_trigger_checks: Option<Vec<ReposownerrepochecksuitespreferencesAutoTriggerChecks>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCodeScanningUpdateAlert {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CodeScanningAlertSetState>,

    #[serde(rename = "dismissed_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCodeScanningUploadSarif {
    #[serde(rename = "commit_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_sha: Option<CodeScanningAnalysisCommitSha>,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<CodeScanningRef>,

    #[serde(rename = "sarif")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sarif: Option<CodeScanningAnalysisSarifFile>,

    /// The base directory used in the analysis, as it appears in the SARIF file. This property is used to convert file paths from absolute to relative, so that alerts can be mapped to their correct location in the repository.
    #[serde(rename = "checkout_uri")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub checkout_uri: Option<String>,

    /// The time that the analysis run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The name of the tool used to generate the code scanning analysis. If this parameter is not used, the tool name defaults to \"API\". If the uploaded SARIF contains a tool GUID, this will be available for filtering using the `tool_guid` parameter of operations such as `GET /repos/{owner}/{repo}/code-scanning/alerts`.
    #[serde(rename = "tool_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tool_name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposAddCollaborator {
    /// The permission to grant the collaborator. **Only valid on organization-owned repositories.** Can be one of:   \\* `pull` - can pull, but not push to or administer this repository.   \\* `push` - can pull and push, but not administer this repository.   \\* `admin` - can pull, push and administer this repository.   \\* `maintain` - Recommended for project managers who need to manage the repository without access to sensitive or destructive actions.   \\* `triage` - Recommended for contributors who need to proactively manage issues and pull requests without write access.
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOauthAuthorizationsGetOrCreateAuthorizationForAppAndFingerprint {
    /// The OAuth app client secret for which to create the token.
    #[serde(rename = "client_secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,

    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// A note to remind you what the OAuth token is for.
    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    /// A URL to remind you what app the OAuth token is for.
    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateCommitComment {
    /// The contents of the comment
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForCommitComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the commit comment.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateCommitComment {
    /// The contents of the comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// Relative path of the file to comment on.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// Line index in the diff to comment on.
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    /// **Deprecated**. Use **position** parameter instead. Line number in the file to comment on.
    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposCreateOrUpdateFileContents {
    /// The commit message.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// The new file content, using Base64 encoding.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    /// **Required if you are updating a file**. The blob SHA of the file being replaced.
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// The branch name. Default: the repository’s default branch (usually `master`)
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepocontentspathCommitter>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepocontentspathAuthor>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposDeleteFile {
    /// The commit message.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// The blob SHA of the file being replaced.
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// The branch name. Default: the repository’s default branch (usually `master`)
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepocontentspathCommitter1>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepocontentspathAuthor1>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeployment {
    /// The ref to deploy. This can be a branch, tag, or SHA.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    /// Specifies a task to execute (e.g., `deploy` or `deploy:migrations`).
    #[serde(rename = "task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,

    /// Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch.
    #[serde(rename = "auto_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_merge: Option<bool>,

    /// The [status](https://docs.github.com/rest/reference/repos#statuses) contexts to verify against commit status checks. If you omit this parameter, GitHub verifies all unique contexts before creating a deployment. To bypass checking entirely, pass an empty array. Defaults to all unique contexts.
    #[serde(rename = "required_contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_contexts: Option<Vec<String>>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<Value>,

    /// Name for the target deployment environment (e.g., `production`, `staging`, `qa`).
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,

    /// Short description of the deployment.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Specifies if the given environment is specific to the deployment and will no longer exist at some point in the future. Default: `false`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(rename = "transient_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,

    /// Specifies if the given environment is one that end-users directly interact with. Default: `true` when `environment` is `production` and `false` otherwise.   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(rename = "production_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentStatus {
    /// The state of the status. Can be one of `error`, `failure`, `inactive`, `in_progress`, `queued` `pending`, or `success`. **Note:** To use the `inactive` state, you must provide the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. To use the `in_progress` and `queued` states, you must provide the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The target URL to associate with this status. This URL should contain output to keep the user updated while the task is running or serve as historical information for what happened in the deployment. **Note:** It's recommended to use the `log_url` parameter, which replaces `target_url`.
    #[serde(rename = "target_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,

    /// The full URL of the deployment's output. This parameter replaces `target_url`. We will continue to accept `target_url` to support legacy uses, but we recommend replacing `target_url` with `log_url`. Setting `log_url` will automatically set `target_url` to the same value. Default: `\"\"`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(rename = "log_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,

    /// A short description of the status. The maximum description length is 140 characters.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. **Note:** This parameter requires you to use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,

    /// Sets the URL for accessing your environment. Default: `\"\"`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(rename = "environment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment_url: Option<String>,

    /// Adds a new `inactive` status to all prior non-transient, non-production environment deployments with the same repository and `environment` name as the created status's deployment. An `inactive` status is only added to deployments that had a `success` state. Default: `true`   **Note:** To add an `inactive` status to `production` environments, you must use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(rename = "auto_inactive")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_inactive: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDispatchEvent {
    /// A custom webhook event name.
    #[serde(rename = "event_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_type: Option<String>,

    /// JSON payload with extra information about the webhook event that your action or worklow may use.
    #[serde(rename = "client_payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_payload: Option<HashMap<String, HashMap<(), ()>>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposCreateOrUpdateEnvironment {
    #[serde(rename = "wait_timer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wait_timer: Option<WaitTimer>,

    /// The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<ReposownerrepoenvironmentsenvironmentNameReviewers>>,

    #[serde(rename = "deployment_branch_policy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateFork {
    /// Optional parameter to specify the organization name if forking into an organization.
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,

}
/// Branch Protection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchProtection {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "required_status_checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_status_checks: Option<BranchprotectionRequiredStatusChecks>,

    #[serde(rename = "enforce_admins")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforce_admins: Option<ProtectedBranchAdminEnforced>,

    #[serde(rename = "required_pull_request_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,

    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,

    #[serde(rename = "required_linear_history")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<BranchprotectionRequiredLinearHistory>,

    #[serde(rename = "allow_force_pushes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<BranchprotectionRequiredLinearHistory>,

    #[serde(rename = "allow_deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<BranchprotectionRequiredLinearHistory>,

    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "protection_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_url: Option<String>,

}
/// Branch Restriction Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchRestrictionPolicy {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "users_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "apps_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps_url: Option<String>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<RepositoryTemplateRepositoryOwner>>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<BranchrestrictionpolicyTeams>>,

    #[serde(rename = "apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<BranchrestrictionpolicyApps>>,

}
/// Branch Short
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchShort {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<BranchshortCommit>,

    #[serde(rename = "protected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protected: Option<bool>,

}
/// Branch With Protection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchWithProtection {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<Commit>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<BranchwithprotectionLinks>,

    #[serde(rename = "protected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protected: Option<bool>,

    #[serde(rename = "protection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection: Option<BranchProtection>,

    #[serde(rename = "protection_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_url: Option<String>,

    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pattern: Option<String>,

    #[serde(rename = "required_approving_review_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchprotectionRequiredLinearHistory {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchprotectionRequiredStatusChecks {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "enforcement_level")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforcement_level: Option<String>,

    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

    #[serde(rename = "contexts_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyApps {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<BranchrestrictionpolicyOwner>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "external_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<BranchrestrictionpolicyPermissions>,

    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyOwner {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "public_members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_members_url: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyPermissions {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,

    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

    #[serde(rename = "issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,

    #[serde(rename = "single_file")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyTeams {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchshortCommit {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchwithprotectionLinks {
    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<String>,

    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<String>,

}
/// Check Annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckAnnotation {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    #[serde(rename = "end_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_line: Option<i64>,

    #[serde(rename = "start_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_column: Option<i64>,

    #[serde(rename = "end_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_column: Option<i64>,

    #[serde(rename = "annotation_level")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation_level: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "raw_details")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_details: Option<String>,

    #[serde(rename = "blob_href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blob_href: Option<String>,

}
/// A check performed on the code of a given code change
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckRun {
    /// The id of the check.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The SHA of the commit that is being checked.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "external_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "details_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub details_url: Option<String>,

    /// The phase of the lifecycle that the check is currently in.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "completed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "output")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub output: Option<CheckrunOutput>,

    /// The name of the check.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "check_suite")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_suite: Option<CheckrunCheckSuite>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<Integration>,

    #[serde(rename = "pull_requests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<Vec<PullRequestMinimal>>,

    #[serde(rename = "deployment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment: Option<DeploymentSimple>,

}
/// A suite of checks performed on the code of a given code change
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSuite {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "head_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_branch: Option<String>,

    /// The SHA of the head commit that is being checked.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "before")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub before: Option<String>,

    #[serde(rename = "after")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub after: Option<String>,

    #[serde(rename = "pull_requests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<Vec<PullRequestMinimal>>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<Integration>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "head_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_commit: Option<SimpleCommit>,

    #[serde(rename = "latest_check_runs_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latest_check_runs_count: Option<i64>,

    #[serde(rename = "check_runs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_runs_url: Option<String>,

}
/// Check suite configuration preferences for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSuitePreference {
    #[serde(rename = "preferences")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferences: Option<ChecksuitepreferencePreferences>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckrunCheckSuite {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckrunOutput {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "annotations_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations_count: Option<i64>,

    #[serde(rename = "annotations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecksuitepreferencePreferences {
    #[serde(rename = "auto_trigger_checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_trigger_checks: Option<Vec<ChecksuitepreferencePreferencesAutoTriggerChecks>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecksuitepreferencePreferencesAutoTriggerChecks {
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<i64>,

    #[serde(rename = "setting")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub setting: Option<bool>,

}
/// Clone Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloneTraffic {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "uniques")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uniques: Option<i64>,

    #[serde(rename = "clones")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clones: Option<Vec<Traffic>>,

}
/// Code Frequency Stat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeFrequencyStat {
}
/// Code Of Conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeOfConduct {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
/// Code of Conduct Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeOfConductSimple {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlert {
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<AlertNumber>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<AlertCreatedAt>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<AlertUrl>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<AlertHtmlUrl>,

    #[serde(rename = "instances_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances_url: Option<AlertInstancesUrl>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CodeScanningAlertState>,

    #[serde(rename = "dismissed_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_by: Option<SimpleUser>,

    #[serde(rename = "dismissed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_at: Option<CodeScanningAlertDismissedAt>,

    #[serde(rename = "dismissed_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,

    #[serde(rename = "rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<CodeScanningAlertRule>,

    #[serde(rename = "tool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tool: Option<CodeScanningAnalysisTool>,

    #[serde(rename = "most_recent_instance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertInstance>,

}
/// A classification of the file. For example to identify it as generated.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum CodeScanningAlertClassification { 
    #[serde(rename = "source")]
    SOURCE,
    #[serde(rename = "generated")]
    GENERATED,
    #[serde(rename = "test")]
    TEST,
    #[serde(rename = "library")]
    LIBRARY,
}

impl Display for CodeScanningAlertClassification {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            CodeScanningAlertClassification::SOURCE => write!(f, "{}", "source"),
            CodeScanningAlertClassification::GENERATED => write!(f, "{}", "generated"),
            CodeScanningAlertClassification::TEST => write!(f, "{}", "test"),
            CodeScanningAlertClassification::LIBRARY => write!(f, "{}", "library"),
        }
    }
}

impl std::str::FromStr for CodeScanningAlertClassification {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source" => Ok(CodeScanningAlertClassification::SOURCE),
            "generated" => Ok(CodeScanningAlertClassification::GENERATED),
            "test" => Ok(CodeScanningAlertClassification::TEST),
            "library" => Ok(CodeScanningAlertClassification::LIBRARY),
            _ => Err(()),
        }
    }
}
/// The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
pub type CodeScanningAlertDismissedAt = DateTime<Utc>;
/// **Required when the state is dismissed.** The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertDismissedReason {
}
/// Identifies the variable values associated with the environment in which the analysis that generated this alert instance was performed, such as the language that was analyzed.
pub type CodeScanningAlertEnvironment = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertInstance {
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<CodeScanningRef>,

    #[serde(rename = "analysis_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub analysis_key: Option<CodeScanningAnalysisAnalysisKey>,

    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<CodeScanningAlertEnvironment>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CodeScanningAlertState>,

    #[serde(rename = "commit_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_sha: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<CodescanningalertinstanceMessage>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<CodeScanningAlertLocation>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// Classifications that have been applied to the file that triggered the alert. For example identifying it as documentation, or a generated file.
    #[serde(rename = "classifications")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifications: Option<Vec<CodeScanningAlertClassification>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertItems {
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<AlertNumber>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<AlertCreatedAt>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<AlertUrl>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<AlertHtmlUrl>,

    #[serde(rename = "instances_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instances_url: Option<AlertInstancesUrl>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CodeScanningAlertState>,

    #[serde(rename = "dismissed_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_by: Option<SimpleUser>,

    #[serde(rename = "dismissed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_at: Option<CodeScanningAlertDismissedAt>,

    #[serde(rename = "dismissed_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,

    #[serde(rename = "rule")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rule: Option<CodeScanningAlertRuleSummary>,

    #[serde(rename = "tool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tool: Option<CodeScanningAnalysisTool>,

    #[serde(rename = "most_recent_instance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertInstance>,

}
/// Describe a region within a file for the alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertLocation {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    #[serde(rename = "end_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_line: Option<i64>,

    #[serde(rename = "start_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_column: Option<i64>,

    #[serde(rename = "end_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_column: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertRule {
    /// A unique identifier for the rule used to detect the alert.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// The name of the rule used to detect the alert.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The severity of the alert.
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub severity: Option<String>,

    /// A short description of the rule used to detect the alert.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// description of the rule used to detect the alert.
    #[serde(rename = "full_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_description: Option<String>,

    /// A set of tags applicable for the rule.
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Detailed documentation for the rule as GitHub Flavored Markdown.
    #[serde(rename = "help")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub help: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertRuleSummary {
    /// A unique identifier for the rule used to detect the alert.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// The name of the rule used to detect the alert.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The severity of the alert.
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub severity: Option<String>,

    /// A short description of the rule used to detect the alert.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
/// Sets the state of the code scanning alert. Can be one of `open` or `dismissed`. You must provide `dismissed_reason` when you set the state to `dismissed`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum CodeScanningAlertSetState { 
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "dismissed")]
    DISMISSED,
}

impl Display for CodeScanningAlertSetState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            CodeScanningAlertSetState::OPEN => write!(f, "{}", "open"),
            CodeScanningAlertSetState::DISMISSED => write!(f, "{}", "dismissed"),
        }
    }
}

impl std::str::FromStr for CodeScanningAlertSetState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(CodeScanningAlertSetState::OPEN),
            "dismissed" => Ok(CodeScanningAlertSetState::DISMISSED),
            _ => Err(()),
        }
    }
}
/// State of a code scanning alert.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum CodeScanningAlertState { 
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "dismissed")]
    DISMISSED,
    #[serde(rename = "fixed")]
    FIXED,
}

impl Display for CodeScanningAlertState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            CodeScanningAlertState::OPEN => write!(f, "{}", "open"),
            CodeScanningAlertState::DISMISSED => write!(f, "{}", "dismissed"),
            CodeScanningAlertState::FIXED => write!(f, "{}", "fixed"),
        }
    }
}

impl std::str::FromStr for CodeScanningAlertState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(CodeScanningAlertState::OPEN),
            "dismissed" => Ok(CodeScanningAlertState::DISMISSED),
            "fixed" => Ok(CodeScanningAlertState::FIXED),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAnalysis {
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<CodeScanningRef>,

    #[serde(rename = "commit_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_sha: Option<CodeScanningAnalysisCommitSha>,

    #[serde(rename = "analysis_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub analysis_key: Option<CodeScanningAnalysisAnalysisKey>,

    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<CodeScanningAnalysisEnvironment>,

    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<CodeScanningAnalysisCreatedAt>,

    /// The total number of results in the analysis.
    #[serde(rename = "results_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub results_count: Option<i64>,

    /// The total number of rules used in the analysis.
    #[serde(rename = "rules_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rules_count: Option<i64>,

    /// Unique identifier for this analysis.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<CodeScanningAnalysisUrl>,

    #[serde(rename = "sarif_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sarif_id: Option<CodeScanningAnalysisSarifId>,

    #[serde(rename = "tool")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tool: Option<CodeScanningAnalysisTool>,

    #[serde(rename = "deletable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletable: Option<bool>,

}
/// Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name.
pub type CodeScanningAnalysisAnalysisKey = String;
/// The SHA of the commit to which the analysis you are uploading relates.
pub type CodeScanningAnalysisCommitSha = String;
/// The time that the analysis was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
pub type CodeScanningAnalysisCreatedAt = DateTime<Utc>;
/// Successful deletion of a code scanning analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAnalysisDeletion {
    /// Next deletable analysis in chain, without last analysis deletion confirmation
    #[serde(rename = "next_analysis_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_analysis_url: Option<String>,

    /// Next deletable analysis in chain, with last analysis deletion confirmation
    #[serde(rename = "confirm_delete_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confirm_delete_url: Option<String>,

}
/// Identifies the variable values associated with the environment in which this analysis was performed.
pub type CodeScanningAnalysisEnvironment = String;
/// A Base64 string representing the SARIF file to upload. You must first compress your SARIF file using [`gzip`](http://www.gnu.org/software/gzip/manual/gzip.html) and then translate the contents of the file into a Base64 encoding string. For more information, see \"[SARIF support for code scanning](https://docs.github.com/github/finding-security-vulnerabilities-and-errors-in-your-code/sarif-support-for-code-scanning).\"
pub type CodeScanningAnalysisSarifFile = String;
/// An identifier for the upload.
pub type CodeScanningAnalysisSarifId = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAnalysisTool {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<CodeScanningAnalysisToolName>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<CodeScanningAnalysisToolVersion>,

    #[serde(rename = "guid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub guid: Option<CodeScanningAnalysisToolGuid>,

}
/// The GUID of the tool used to generate the code scanning analysis, if provided in the uploaded SARIF data.
pub type CodeScanningAnalysisToolGuid = String;
/// The name of the tool used to generate the code scanning analysis.
pub type CodeScanningAnalysisToolName = String;
/// The version of the tool used to generate the code scanning analysis.
pub type CodeScanningAnalysisToolVersion = String;
/// The REST API URL of the analysis resource.
pub type CodeScanningAnalysisUrl = String;
/// The full Git reference, formatted as `refs/heads/<branch name>`.
pub type CodeScanningRef = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningSarifsReceipt {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<CodeScanningAnalysisSarifId>,

    /// The REST API URL for checking the status of the upload.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningSarifsStatus {
    /// `pending` files have not yet been processed, while `complete` means all results in the SARIF have been stored.
    #[serde(rename = "processing_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_status: Option<String>,

    /// The REST API URL for getting the analyses associated with the upload.
    #[serde(rename = "analyses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub analyses_url: Option<String>,

}
/// Code Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSearchResultItem {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "file_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_size: Option<i64>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "last_modified_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "line_numbers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line_numbers: Option<Vec<String>>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodescanningalertinstanceMessage {
    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

}
/// Collaborator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<CollaboratorPermissions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollaboratorPermissions {
    #[serde(rename = "pull")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull: Option<bool>,

    #[serde(rename = "push")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push: Option<bool>,

    #[serde(rename = "admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombinedBillingUsage {
    /// Numbers of days left in billing cycle.
    #[serde(rename = "days_left_in_billing_cycle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub days_left_in_billing_cycle: Option<i64>,

    /// Estimated storage space (GB) used in billing cycle.
    #[serde(rename = "estimated_paid_storage_for_month")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_paid_storage_for_month: Option<i64>,

    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    #[serde(rename = "estimated_storage_for_month")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub estimated_storage_for_month: Option<i64>,

}
/// Combined Commit Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombinedCommitStatus {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "statuses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<Vec<SimpleCommitStatus>>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "commit_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<CommitCommit>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<SimpleUser>,

    #[serde(rename = "parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<CommitParents>>,

    #[serde(rename = "stats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stats: Option<CommitStats>,

    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<Vec<CommitFiles>>,

}
/// Commit Activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitActivity {
    #[serde(rename = "days")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub days: Option<Vec<i32>>,

    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "week")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub week: Option<i64>,

}
/// Commit Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComment {
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommit {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<GitUser>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<GitUser>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "comment_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment_count: Option<i64>,

    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<CommitCommitTree>,

    #[serde(rename = "verification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommitTree {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// Commit Comparison
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComparison {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "permalink_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permalink_url: Option<String>,

    #[serde(rename = "diff_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_url: Option<String>,

    #[serde(rename = "patch_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch_url: Option<String>,

    #[serde(rename = "base_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base_commit: Option<Commit>,

    #[serde(rename = "merge_base_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_base_commit: Option<Commit>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "ahead_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ahead_by: Option<i64>,

    #[serde(rename = "behind_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub behind_by: Option<i64>,

    #[serde(rename = "total_commits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_commits: Option<i64>,

    #[serde(rename = "commits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits: Option<Vec<Commit>>,

    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<Vec<DiffEntry>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitFiles {
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "additions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,

    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,

    #[serde(rename = "changes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub changes: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "raw_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,

    #[serde(rename = "blob_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blob_url: Option<String>,

    #[serde(rename = "patch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "previous_filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_filename: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitParents {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
/// Commit Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitSearchResultItem {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<CommitsearchresultitemCommit>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<GitUser>,

    #[serde(rename = "parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<FilecommitCommitParents>>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitStats {
    #[serde(rename = "additions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,

    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,

    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitsearchresultitemCommit {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<CommitsearchresultitemCommitAuthor>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<GitUser>,

    #[serde(rename = "comment_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment_count: Option<i64>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<ShortbranchCommit>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "verification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitsearchresultitemCommitAuthor {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityHealthFile {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
/// Community Profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityProfile {
    #[serde(rename = "health_percentage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub health_percentage: Option<i64>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation: Option<String>,

    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<CommunityprofileFiles>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "content_reports_enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_reports_enabled: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityprofileFiles {
    #[serde(rename = "code_of_conduct")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_of_conduct: Option<CodeOfConductSimple>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "contributing")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributing: Option<CommunityHealthFile>,

    #[serde(rename = "readme")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub readme: Option<CommunityHealthFile>,

    #[serde(rename = "issue_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_template: Option<CommunityHealthFile>,

    #[serde(rename = "pull_request_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_template: Option<CommunityHealthFile>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerMetadata {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Value>>,

}
/// A list of directory items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentDirectory {
}
/// Content File
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentFile {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

    #[serde(rename = "target")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target: Option<String>,

    #[serde(rename = "submodule_git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submodule_git_url: Option<String>,

}
/// Content Reference attachments allow you to provide context around URLs posted in comments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentReferenceAttachment {
    /// The ID of the attachment
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The title of the attachment
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The body of the attachment
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// The node_id of the content attachment
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

}
/// An object describing a symlink
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSubmodule {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "submodule_git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submodule_git_url: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

}
/// An object describing a symlink
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSymlink {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "target")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

}
/// Content Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTraffic {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "uniques")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uniques: Option<i64>,

}
/// Content Tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTree {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "entries")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub entries: Option<Vec<ContenttreeEntries>>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentdirectoryInner {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<HashMap<(), ()>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContenttreeEntries {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContenttreeLinks {
    #[serde(rename = "git")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git: Option<String>,

    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<String>,

    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<String>,

}
/// Contributor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "contributions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributions: Option<i64>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
/// Contributor Activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributorActivity {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "weeks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub weeks: Option<Vec<ContributoractivityWeeks>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributoractivityWeeks {
    #[serde(rename = "w")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub w: Option<String>,

    #[serde(rename = "a")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub a: Option<i64>,

    #[serde(rename = "d")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub d: Option<i64>,

    #[serde(rename = "c")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub c: Option<i64>,

}
/// Credential Authorization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAuthorization {
    /// User login that owns the underlying credential.
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    /// Unique identifier for the credential.
    #[serde(rename = "credential_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential_id: Option<i64>,

    /// Human-readable description of the credential type.
    #[serde(rename = "credential_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential_type: Option<String>,

    /// Last eight characters of the credential. Only included in responses with credential_type of personal access token.
    #[serde(rename = "token_last_eight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_last_eight: Option<String>,

    /// Date when the credential was authorized for use.
    #[serde(rename = "credential_authorized_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential_authorized_at: Option<chrono::DateTime<chrono::Utc>>,

    /// List of oauth scopes the token has been granted.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Unique string to distinguish the credential. Only included in responses with credential_type of SSH Key.
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

    /// Date when the credential was last accessed. May be null if it was never accessed
    #[serde(rename = "credential_accessed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential_accessed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "authorized_credential_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_id: Option<i64>,

    /// The title given to the ssh key. This will only be present when the credential is an ssh key.
    #[serde(rename = "authorized_credential_title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_title: Option<String>,

    /// The note given to the token. This will only be present when the credential is a token.
    #[serde(rename = "authorized_credential_note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_note: Option<String>,

}
/// An SSH key granting access to a single repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeployKey {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "read_only")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,

}
/// A request for a specific ref(branch,sha,tag) to be deployed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// Unique identifier of the deployment
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// The ref to deploy. This can be a branch, tag, or sha.
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    /// Parameter to specify a task to execute
    #[serde(rename = "task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<HashMap<(), ()>>,

    #[serde(rename = "original_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_environment: Option<String>,

    /// Name for the target deployment environment.
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
    #[serde(rename = "transient_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,

    /// Specifies if the given environment is one that end-users directly interact with. Default: false.
    #[serde(rename = "production_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

}
/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentBranchPolicy {
    /// Whether only branches with branch protection rules can deploy to this environment. If `protected_branches` is `true`, `custom_branch_policies` must be `false`; if `protected_branches` is `false`, `custom_branch_policies` must be `true`.
    #[serde(rename = "protected_branches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protected_branches: Option<bool>,

    /// Whether only branches that match the specified name patterns can deploy to this environment.  If `custom_branch_policies` is `true`, `protected_branches` must be `false`; if `custom_branch_policies` is `false`, `protected_branches` must be `true`.
    #[serde(rename = "custom_branch_policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_branch_policies: Option<bool>,

}
/// The type of reviewer. Must be one of: `User` or `Team`
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum DeploymentReviewerType { 
    #[serde(rename = "User")]
    USER,
    #[serde(rename = "Team")]
    TEAM,
}

impl Display for DeploymentReviewerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            DeploymentReviewerType::USER => write!(f, "{}", "User"),
            DeploymentReviewerType::TEAM => write!(f, "{}", "Team"),
        }
    }
}

impl std::str::FromStr for DeploymentReviewerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(DeploymentReviewerType::USER),
            "Team" => Ok(DeploymentReviewerType::TEAM),
            _ => Err(()),
        }
    }
}
/// A deployment created as the result of an Actions check run from a workflow that references an environment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentSimple {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// Unique identifier of the deployment
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// Parameter to specify a task to execute
    #[serde(rename = "task")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,

    #[serde(rename = "original_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_environment: Option<String>,

    /// Name for the target deployment environment.
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
    #[serde(rename = "transient_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,

    /// Specifies if the given environment is one that end-users directly interact with. Default: false.
    #[serde(rename = "production_environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

}
/// The status of a deployment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatus {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The state of the status.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    /// A short description of the status.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The environment of the deployment that the status is for.
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,

    /// Deprecated: the URL to associate with this status.
    #[serde(rename = "target_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "deployment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    /// The URL for accessing your environment.
    #[serde(rename = "environment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment_url: Option<String>,

    /// The URL to associate with this status.
    #[serde(rename = "log_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

}
/// Diff Entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffEntry {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "additions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,

    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,

    #[serde(rename = "changes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub changes: Option<i64>,

    #[serde(rename = "blob_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blob_url: Option<String>,

    #[serde(rename = "raw_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "patch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<String>,

    #[serde(rename = "previous_filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_filename: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockerMetadata {
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag: Option<Vec<Value>>,

}
/// Email
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

}
/// The policy that controls the organizations in the enterprise that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum EnabledOrganizations { 
    #[serde(rename = "all")]
    ALL,
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "selected")]
    SELECTED,
}

impl Display for EnabledOrganizations {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            EnabledOrganizations::ALL => write!(f, "{}", "all"),
            EnabledOrganizations::NONE => write!(f, "{}", "none"),
            EnabledOrganizations::SELECTED => write!(f, "{}", "selected"),
        }
    }
}

impl std::str::FromStr for EnabledOrganizations {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(EnabledOrganizations::ALL),
            "none" => Ok(EnabledOrganizations::NONE),
            "selected" => Ok(EnabledOrganizations::SELECTED),
            _ => Err(()),
        }
    }
}
/// The policy that controls the repositories in the organization that are allowed to run GitHub Actions. Can be one of: `all`, `none`, or `selected`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum EnabledRepositories { 
    #[serde(rename = "all")]
    ALL,
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "selected")]
    SELECTED,
}

impl Display for EnabledRepositories {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            EnabledRepositories::ALL => write!(f, "{}", "all"),
            EnabledRepositories::NONE => write!(f, "{}", "none"),
            EnabledRepositories::SELECTED => write!(f, "{}", "selected"),
        }
    }
}

impl std::str::FromStr for EnabledRepositories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(EnabledRepositories::ALL),
            "none" => Ok(EnabledRepositories::NONE),
            "selected" => Ok(EnabledRepositories::SELECTED),
            _ => Err(()),
        }
    }
}
/// An enterprise account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enterprise {
    /// A short description of the enterprise.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// The enterprise's website URL.
    #[serde(rename = "website_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub website_url: Option<String>,

    /// Unique identifier of the enterprise
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the enterprise.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The slug url identifier for the enterprise.
    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

}
/// Details of a deployment environment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    /// The id of the environment.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the environment.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// The time that the environment was created, in ISO 8601 format.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The time that the environment was last updated, in ISO 8601 format.
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "protection_rules")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_rules: Option<Vec<Value>>,

    #[serde(rename = "deployment_branch_policy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,

}
/// An entry in the reviews log for environment deployments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentApprovals {
    /// The list of environments that were approved or rejected
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environments: Option<Vec<EnvironmentapprovalsEnvironments>>,

    /// Whether deployment to the environment(s) was approved or rejected
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    /// The comment submitted with the deployment review
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentapprovalsEnvironments {
    /// The id of the environment.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the environment.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// The time that the environment was created, in ISO 8601 format.
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The time that the environment was last updated, in ISO 8601 format.
    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "actor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<Actor>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<EventRepo>,

    #[serde(rename = "org")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub org: Option<Actor>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<EventPayload>,

    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventPayload {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "issue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<IssueSimple>,

    #[serde(rename = "comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<IssueComment>,

    #[serde(rename = "pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<Vec<EventPayloadPages>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventPayloadPages {
    #[serde(rename = "page_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_name: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,

    #[serde(rename = "action")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventRepo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// Feed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    #[serde(rename = "timeline_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,

    #[serde(rename = "user_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_url: Option<String>,

    #[serde(rename = "current_user_public_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_public_url: Option<String>,

    #[serde(rename = "current_user_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_url: Option<String>,

    #[serde(rename = "current_user_actor_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_actor_url: Option<String>,

    #[serde(rename = "current_user_organization_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization_url: Option<String>,

    #[serde(rename = "current_user_organization_urls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization_urls: Option<Vec<String>>,

    #[serde(rename = "security_advisories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_advisories_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<FeedLinks>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedLinks {
    #[serde(rename = "timeline")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline: Option<LinkWithType>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<LinkWithType>,

    #[serde(rename = "security_advisories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_advisories: Option<LinkWithType>,

    #[serde(rename = "current_user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user: Option<LinkWithType>,

    #[serde(rename = "current_user_public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_public: Option<LinkWithType>,

    #[serde(rename = "current_user_actor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_actor: Option<LinkWithType>,

    #[serde(rename = "current_user_organization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization: Option<LinkWithType>,

    #[serde(rename = "current_user_organizations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organizations: Option<Vec<LinkWithType>>,

}
/// File Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCommit {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<FilecommitContent>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<FilecommitCommit>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommit {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<FilecommitCommitAuthor>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<FilecommitCommitAuthor>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<FilecommitCommitTree>,

    #[serde(rename = "parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<FilecommitCommitParents>>,

    #[serde(rename = "verification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<FilecommitCommitVerification>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitAuthor {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitParents {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitTree {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitVerification {
    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "signature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitContent {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<FilecommitContentLinks>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitContentLinks {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<String>,

    #[serde(rename = "git")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git: Option<String>,

    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<String>,

}
/// Full Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullRepository {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    /// Returns whether or not this repository disabled.
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    /// The repository visibility: public, private, or internal.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,

    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "template_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "subscribers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,

    #[serde(rename = "network_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "organization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<SimpleUser>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<Repository>,

    #[serde(rename = "source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<Repository>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    /// Whether anonymous git access is allowed.
    #[serde(rename = "anonymous_access_enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub anonymous_access_enabled: Option<bool>,

    #[serde(rename = "code_of_conduct")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_of_conduct: Option<CodeOfConductSimple>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullrepositoryPermissions {
    #[serde(rename = "admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,

    #[serde(rename = "pull")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull: Option<bool>,

    #[serde(rename = "push")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push: Option<bool>,

}
/// A comment made to a gist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistComment {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The comment text.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

}
/// Gist Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistCommit {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "change_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_status: Option<GistcommitChangeStatus>,

    #[serde(rename = "committed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committed_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Gist Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistSimple {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "git_pull_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_pull_url: Option<String>,

    #[serde(rename = "git_push_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_push_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, GistsimpleFiles>>,

    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistcommitChangeStatus {
    #[serde(rename = "total")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,

    #[serde(rename = "additions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,

    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsFiles {
    /// Content of the file
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsgistIdFiles {
    /// The new content of the file
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    /// The new filename for the file
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsimpleFiles {
    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "raw_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,

    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
/// Low-level Git commit operations within a repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitCommit {
    /// SHA for the commit
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<GitcommitAuthor>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<GitcommitAuthor>,

    /// Message describing the purpose of the commit
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<GitcommitTree>,

    #[serde(rename = "parents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<GitcommitParents>>,

    #[serde(rename = "verification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<GitcommitVerification>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
/// Git references within a repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitRef {
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "object")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object: Option<GitrefObject>,

}
/// Metadata for a Git tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitTag {
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// Name of the tag
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// URL for the tag
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// Message describing the purpose of the tag
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "tagger")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tagger: Option<GittagTagger>,

    #[serde(rename = "object")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object: Option<GittagObject>,

    #[serde(rename = "verification")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,

}
/// The hierarchy between files in a Git repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitTree {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,

    /// Objects specifying a tree structure
    #[serde(rename = "tree")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<Vec<GittreeTree>>,

}
/// Metaproperties for Git author/committer information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitUser {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
/// Identifying information for the git-user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitAuthor {
    /// Timestamp of the commit
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,

    /// Git email address of the user
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// Name of the git user
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitParents {
    /// SHA for the commit
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitTree {
    /// SHA for the commit
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitVerification {
    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "signature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<String>,

}
/// Gitignore Template
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitignoreTemplate {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitrefObject {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    /// SHA for the reference
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittagObject {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittagTagger {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittreeTree {
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "mode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// A unique encryption key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgKey {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "primary_key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_key_id: Option<i64>,

    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    #[serde(rename = "public_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,

    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<GpgkeyEmails>>,

    #[serde(rename = "subkeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subkeys: Option<Vec<GpgkeySubkeys>>,

    #[serde(rename = "can_sign")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_sign: Option<bool>,

    #[serde(rename = "can_encrypt_comms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_comms: Option<bool>,

    #[serde(rename = "can_encrypt_storage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_storage: Option<bool>,

    #[serde(rename = "can_certify")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_certify: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "raw_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_key: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgkeyEmails {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgkeySubkeys {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "primary_key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_key_id: Option<i64>,

    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    #[serde(rename = "public_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,

    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Value>>,

    #[serde(rename = "subkeys")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subkeys: Option<Vec<Value>>,

    #[serde(rename = "can_sign")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_sign: Option<bool>,

    #[serde(rename = "can_encrypt_comms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_comms: Option<bool>,

    #[serde(rename = "can_encrypt_storage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_storage: Option<bool>,

    #[serde(rename = "can_certify")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_certify: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(rename = "raw_key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_key: Option<String>,

}
/// External Groups to be mapped to a team for membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMapping {
    /// Array of groups to be mapped to this team
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<GroupmappingGroups>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupmappingGroups {
    /// The ID of the group
    #[serde(rename = "group_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_id: Option<String>,

    /// The name of the group
    #[serde(rename = "group_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_name: Option<String>,

    /// a description of the group
    #[serde(rename = "group_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_description: Option<String>,

    /// synchronization status for this group mapping
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// the time of the last sync for this group-mapping
    #[serde(rename = "synced_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub synced_at: Option<String>,

}
/// Webhooks for repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    /// Unique identifier of the webhook.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The name of a valid service, use 'web' for a webhook.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Determines whether the hook is actually triggered on pushes.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    /// Determines what events the hook is triggered for. Default: ['push'].
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<HookConfig>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "test_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub test_url: Option<String>,

    #[serde(rename = "ping_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ping_url: Option<String>,

    #[serde(rename = "last_response")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_response: Option<HookResponse>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookConfig {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "room")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room: Option<String>,

    #[serde(rename = "subdomain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subdomain: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "digest")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub digest: Option<String>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookResponse {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<i64>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}
/// Hovercard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hovercard {
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<HovercardContexts>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HovercardContexts {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "octicon")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub octicon: Option<String>,

}
/// A repository import from an external source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    #[serde(rename = "vcs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,

    #[serde(rename = "use_lfs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_lfs: Option<String>,

    /// The URL of the originating repository.
    #[serde(rename = "vcs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_url: Option<String>,

    #[serde(rename = "svc_root")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svc_root: Option<String>,

    #[serde(rename = "tfvc_project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "status_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_text: Option<String>,

    #[serde(rename = "failed_step")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_step: Option<String>,

    #[serde(rename = "error_message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,

    #[serde(rename = "import_percent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_percent: Option<i64>,

    #[serde(rename = "commit_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_count: Option<i64>,

    #[serde(rename = "push_percent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push_percent: Option<i64>,

    #[serde(rename = "has_large_files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_large_files: Option<bool>,

    #[serde(rename = "large_files_size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub large_files_size: Option<i64>,

    #[serde(rename = "large_files_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub large_files_count: Option<i64>,

    #[serde(rename = "project_choices")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_choices: Option<Vec<ImportProjectChoices>>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "authors_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authors_count: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "authors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authors_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    #[serde(rename = "svn_root")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_root: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportProjectChoices {
    #[serde(rename = "vcs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,

    #[serde(rename = "tfvc_project")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,

    #[serde(rename = "human_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub human_name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetaRootResponse200 {
    #[serde(rename = "current_user_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_url: Option<String>,

    #[serde(rename = "current_user_authorizations_html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_authorizations_html_url: Option<String>,

    #[serde(rename = "authorizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorizations_url: Option<String>,

    #[serde(rename = "code_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_search_url: Option<String>,

    #[serde(rename = "commit_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_search_url: Option<String>,

    #[serde(rename = "emails_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails_url: Option<String>,

    #[serde(rename = "emojis_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emojis_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "feeds_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub feeds_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "hub_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hub_url: Option<String>,

    #[serde(rename = "issue_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_search_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "label_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label_search_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "organization_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_url: Option<String>,

    #[serde(rename = "organization_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_repositories_url: Option<String>,

    #[serde(rename = "organization_teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_teams_url: Option<String>,

    #[serde(rename = "public_gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists_url: Option<String>,

    #[serde(rename = "rate_limit_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rate_limit_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    #[serde(rename = "repository_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_search_url: Option<String>,

    #[serde(rename = "current_user_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_repositories_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "starred_gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_gists_url: Option<String>,

    #[serde(rename = "topic_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_search_url: Option<String>,

    #[serde(rename = "user_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_url: Option<String>,

    #[serde(rename = "user_organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_organizations_url: Option<String>,

    #[serde(rename = "user_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_repositories_url: Option<String>,

    #[serde(rename = "user_search_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_search_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsCheckAuthorizationResponse200 {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// A list of scopes that this authorization is in.
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "token_last_eight")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_last_eight: Option<String>,

    #[serde(rename = "hashed_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hashed_token: Option<String>,

    #[serde(rename = "app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app: Option<ApplicationgrantApp>,

    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    #[serde(rename = "note_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "installation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub installation: Option<ScopedInstallation>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListOrgSecretsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secrets: Option<Vec<OrganizationActionsSecret>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelectedReposForOrgSecretResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<MinimalRepository>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListInstallationsForAuthenticatedUserResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "installations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub installations: Option<Vec<Installation>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListWorkflowRunArtifactsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListWorkflowRunsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "workflow_runs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow_runs: Option<Vec<WorkflowRun>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListJobsForWorkflowRunResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<Vec<Job>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListEnvironmentSecretsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secrets: Option<Vec<ActionsSecret>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListRepoWorkflowsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "workflows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflows: Option<Vec<Workflow>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChecksListForRefResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "check_runs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_runs: Option<Vec<CheckRun>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChecksListSuitesForRefResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "check_suites")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_suites: Option<Vec<CheckSuite>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListOrgAccessToSelfHostedRunnerGroupInEnterpriseResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "organizations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations: Option<Vec<OrganizationSimple>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20020 {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReposGetAllEnvironmentsResponse200 {
    /// The number of environments in this repository
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "environments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environments: Option<Vec<Environment>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchCodeResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<CodeSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchCommitsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<CommitSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchIssuesAndPullRequestsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<IssueSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchLabelsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<LabelSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchReposResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<RepoSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchTopicsResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<TopicSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchUsersResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "incomplete_results")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub incomplete_results: Option<bool>,

    #[serde(rename = "items")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<UserSearchResultItem>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsersGetByUsernameResponse200 {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListSelfHostedRunnerGroupsForEnterpriseResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "runner_groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runner_groups: Option<Vec<RunnerGroupsEnterprise>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListInstallationReposForAuthenticatedUserResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnersInGroupForOrgResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<Runner>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListSelfHostedRunnersForEnterpriseResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<Runner>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListReposAccessibleToInstallationResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListRepoAccessToSelfHostedRunnerGroupInOrgResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnerGroupsForOrgResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,

    #[serde(rename = "runner_groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runner_groups: Option<Vec<RunnerGroupsOrg>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnersForRepoResponse200 {
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "runners")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<Runner>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateFromManifestResponse201 {
    /// Unique identifier of the GitHub app
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The slug name of the GitHub app
    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    /// The name of the GitHub app
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "external_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<IntegrationPermissions>,

    /// The list of events for the GitHub app
    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    /// The number of installations associated with the GitHub app
    #[serde(rename = "installations_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub installations_count: Option<i64>,

    #[serde(rename = "client_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,

    #[serde(rename = "client_secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,

    #[serde(rename = "webhook_secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub webhook_secret: Option<String>,

    #[serde(rename = "pem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pem: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentResponse202 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateBranchResponse202 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGistsGetCommentResponse403 {
    #[serde(rename = "block")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub block: Option<InlineResponse403Block>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsLegacyResponse403 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateResponse403 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveCardResponse403 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4033Errors>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse4033Errors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "resource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse403Block {
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposMergeResponse409 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentResponse409 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProjectsListForUserResponse415 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCardResponse422 {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserInOrgResponse422 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse4221Errors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,

    #[serde(rename = "resource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddMemberLegacyResponse422 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserLegacyResponse422 {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchUsersResponse503 {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCardResponse503 {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse5031Errors>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse5031Errors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}
/// Installation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Installation {
    /// The ID of the installation.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<Value>,

    /// Describe whether all repositories have been selected or there's a selection involved
    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

    #[serde(rename = "access_tokens_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_tokens_url: Option<String>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<i64>,

    /// The ID of the user or organization this token is being scoped to.
    #[serde(rename = "target_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<i64>,

    #[serde(rename = "target_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_type: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<InstallationPermissions>,

    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "single_file_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_name: Option<String>,

    #[serde(rename = "has_multiple_single_files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,

    #[serde(rename = "single_file_paths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,

    #[serde(rename = "app_slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_slug: Option<String>,

    #[serde(rename = "suspended_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_by: Option<SimpleUser>,

    #[serde(rename = "suspended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "contact_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_email: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationPermissions {
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<String>,

    #[serde(rename = "checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,

    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

    #[serde(rename = "pull_requests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<String>,

    #[serde(rename = "statuses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<String>,

    #[serde(rename = "issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,

    #[serde(rename = "organization_administration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_administration: Option<String>,

}
/// Authentication token for a GitHub App installed on a user or org.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationToken {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<InstallationtokenPermissions>,

    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

    #[serde(rename = "single_file")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,

    #[serde(rename = "has_multiple_single_files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,

    #[serde(rename = "single_file_paths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationtokenPermissions {
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,

    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,

    #[serde(rename = "single_file")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,

}
/// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Integration {
}
/// The set of permissions for the GitHub app
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationPermissions {
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,

    #[serde(rename = "checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,

    #[serde(rename = "contents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,

    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<String>,

}
/// The duration of the interaction restriction. Can be one of: `one_day`, `three_days`, `one_week`, `one_month`, `six_months`. Default: `one_day`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum InteractionExpiry { 
    #[serde(rename = "one_day")]
    ONE_DAY,
    #[serde(rename = "three_days")]
    THREE_DAYS,
    #[serde(rename = "one_week")]
    ONE_WEEK,
    #[serde(rename = "one_month")]
    ONE_MONTH,
    #[serde(rename = "six_months")]
    SIX_MONTHS,
}

impl Display for InteractionExpiry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            InteractionExpiry::ONE_DAY => write!(f, "{}", "one_day"),
            InteractionExpiry::THREE_DAYS => write!(f, "{}", "three_days"),
            InteractionExpiry::ONE_WEEK => write!(f, "{}", "one_week"),
            InteractionExpiry::ONE_MONTH => write!(f, "{}", "one_month"),
            InteractionExpiry::SIX_MONTHS => write!(f, "{}", "six_months"),
        }
    }
}

impl std::str::FromStr for InteractionExpiry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one_day" => Ok(InteractionExpiry::ONE_DAY),
            "three_days" => Ok(InteractionExpiry::THREE_DAYS),
            "one_week" => Ok(InteractionExpiry::ONE_WEEK),
            "one_month" => Ok(InteractionExpiry::ONE_MONTH),
            "six_months" => Ok(InteractionExpiry::SIX_MONTHS),
            _ => Err(()),
        }
    }
}
/// The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum InteractionGroup { 
    #[serde(rename = "existing_users")]
    EXISTING_USERS,
    #[serde(rename = "contributors_only")]
    CONTRIBUTORS_ONLY,
    #[serde(rename = "collaborators_only")]
    COLLABORATORS_ONLY,
}

impl Display for InteractionGroup {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            InteractionGroup::EXISTING_USERS => write!(f, "{}", "existing_users"),
            InteractionGroup::CONTRIBUTORS_ONLY => write!(f, "{}", "contributors_only"),
            InteractionGroup::COLLABORATORS_ONLY => write!(f, "{}", "collaborators_only"),
        }
    }
}

impl std::str::FromStr for InteractionGroup {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "existing_users" => Ok(InteractionGroup::EXISTING_USERS),
            "contributors_only" => Ok(InteractionGroup::CONTRIBUTORS_ONLY),
            "collaborators_only" => Ok(InteractionGroup::COLLABORATORS_ONLY),
            _ => Err(()),
        }
    }
}
/// Limit interactions to a specific type of user for a specified duration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutInteractionsSetRestrictionsForAuthenticatedUser {
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<InteractionGroup>,

    #[serde(rename = "expiry")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiry: Option<InteractionExpiry>,

}
/// Interaction limit settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionLimitResponse {
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<InteractionGroup>,

    #[serde(rename = "origin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub origin: Option<String>,

    #[serde(rename = "expires_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// URL for the issue
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// Number uniquely identifying the issue within its repository
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// State of the issue; either 'open' or 'closed'
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// Title of the issue
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// Contents of the issue
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    /// Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Value>>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Milestone>,

    #[serde(rename = "locked")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,

    #[serde(rename = "active_lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "closed_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_by: Option<SimpleUser>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "timeline_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

}
/// Comments provide a way for people to collaborate on an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    /// Unique identifier of the issue comment
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// URL for the issue comment
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// Contents of the issue comment
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "issue_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_url: Option<String>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

}
/// Issue Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEvent {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "actor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<SimpleUser>,

    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    #[serde(rename = "commit_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "issue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<IssueSimple>,

    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<IssueEventLabel>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "assigner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assigner: Option<SimpleUser>,

    #[serde(rename = "review_requester")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_requester: Option<SimpleUser>,

    #[serde(rename = "requested_reviewer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewer: Option<SimpleUser>,

    #[serde(rename = "requested_team")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_team: Option<Team>,

    #[serde(rename = "dismissed_review")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_review: Option<IssueEventDismissedReview>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<IssueEventMilestone>,

    #[serde(rename = "project_card")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,

    #[serde(rename = "rename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rename: Option<IssueEventRename>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventDismissedReview {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "review_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_id: Option<i64>,

    #[serde(rename = "dismissal_message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_message: Option<String>,

    #[serde(rename = "dismissal_commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_commit_id: Option<String>,

}
/// Issue Event for Issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventForIssue {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "actor")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<SimpleUser>,

    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    #[serde(rename = "commit_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "issue_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_url: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,

    #[serde(rename = "submitted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_at: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "pull_request_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_url: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

}
/// Issue Event Label
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventLabel {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

}
/// Issue Event Milestone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventMilestone {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

}
/// Issue Event Project Card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventProjectCard {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "project_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_url: Option<String>,

    #[serde(rename = "project_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_id: Option<i64>,

    #[serde(rename = "column_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub column_name: Option<String>,

    #[serde(rename = "previous_column_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_column_name: Option<String>,

}
/// Issue Event Rename
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventRename {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<String>,

}
/// Issue Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueSearchResultItem {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "locked")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,

    #[serde(rename = "active_lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,

    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<IssuesearchresultitemLabels>>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Milestone>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "timeline_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

}
/// Issue Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueSimple {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Label>>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Milestone>,

    #[serde(rename = "locked")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,

    #[serde(rename = "active_lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "timeline_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,

    #[serde(rename = "performed_via_github_app")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuesearchresultitemLabels {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuesimplePullRequest {
    #[serde(rename = "merged_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "diff_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "patch_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// Information of a job execution in a workflow run
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    /// The id of the job.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The id of the associated workflow run.
    #[serde(rename = "run_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_id: Option<i64>,

    #[serde(rename = "run_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The SHA of the commit that is being run.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// The phase of the lifecycle that the job is currently in.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// The outcome of the job.
    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    /// The time that the job started, in ISO 8601 format.
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The time that the job finished, in ISO 8601 format.
    #[serde(rename = "completed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The name of the job.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Steps in this job.
    #[serde(rename = "steps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<Vec<JobSteps>>,

    #[serde(rename = "check_run_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_run_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobSteps {
    /// The phase of the lifecycle that the job is currently in.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// The outcome of the job.
    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    /// The name of the job.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// The time that the step started, in ISO 8601 format.
    #[serde(rename = "started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The time that the job finished, in ISO 8601 format.
    #[serde(rename = "completed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[serde(rename = "key_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "read_only")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,

}
/// Key Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySimple {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

}
/// Color-coded labels help you categorize and filter your issues (just like labels in Gmail).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// URL for the label
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The name of the label.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// 6-character hex code, without the leading #, identifying the color
    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

}
/// Label Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelSearchResultItem {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

}
/// Language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
}
/// License
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "spdx_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spdx_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "implementation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub implementation: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conditions: Option<Vec<String>>,

    #[serde(rename = "limitations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limitations: Option<Vec<String>>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "featured")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub featured: Option<bool>,

}
/// License Content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseContent {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ContenttreeLinks>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

}
/// License Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseSimple {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "spdx_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spdx_id: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
/// Hypermedia Link
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

}
/// Hypermedia Link with Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkWithType {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceAccount {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "organization_billing_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_billing_email: Option<String>,

}
/// Marketplace Listing Plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceListingPlan {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "accounts_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub accounts_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "monthly_price_in_cents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub monthly_price_in_cents: Option<i64>,

    #[serde(rename = "yearly_price_in_cents")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub yearly_price_in_cents: Option<i64>,

    #[serde(rename = "price_model")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub price_model: Option<String>,

    #[serde(rename = "has_free_trial")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_free_trial: Option<bool>,

    #[serde(rename = "unit_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_name: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "bullets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bullets: Option<Vec<String>>,

}
/// Marketplace Purchase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacePurchase {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "organization_billing_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_billing_email: Option<String>,

    #[serde(rename = "marketplace_pending_change")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marketplace_pending_change: Option<MarketplacepurchaseMarketplacePendingChange>,

    #[serde(rename = "marketplace_purchase")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marketplace_purchase: Option<MarketplacepurchaseMarketplacePurchase>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacepurchaseMarketplacePendingChange {
    #[serde(rename = "is_installed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_installed: Option<bool>,

    #[serde(rename = "effective_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub effective_date: Option<String>,

    #[serde(rename = "unit_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_count: Option<i64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacepurchaseMarketplacePurchase {
    #[serde(rename = "billing_cycle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_cycle: Option<String>,

    #[serde(rename = "next_billing_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_billing_date: Option<String>,

    #[serde(rename = "is_installed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_installed: Option<bool>,

    #[serde(rename = "unit_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_count: Option<i64>,

    #[serde(rename = "on_free_trial")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub on_free_trial: Option<bool>,

    #[serde(rename = "free_trial_ends_on")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub free_trial_ends_on: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,

}
/// A migration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Migration {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "guid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub guid: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "lock_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_repositories: Option<bool>,

    #[serde(rename = "exclude_attachments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_attachments: Option<bool>,

    #[serde(rename = "repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "exclude")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<Value>>,

}
/// A collection of related issues and pull requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milestone {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The number of the milestone.
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// The state of the milestone.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The title of the milestone.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "closed_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_issues: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "due_on")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_on: Option<chrono::DateTime<chrono::Utc>>,

}
/// Minimal Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimalRepository {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryTemplateRepositoryPermissions>,

    #[serde(rename = "template_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    #[serde(rename = "subscribers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,

    #[serde(rename = "network_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<MinimalrepositoryLicense>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimalrepositoryLicense {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "spdx_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spdx_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

}
/// Org Hook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgHook {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "ping_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ping_url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "config")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<OrghookConfig>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
/// Org Membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgMembership {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "organization_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_url: Option<String>,

    #[serde(rename = "organization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<OrganizationSimple>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<OrgmembershipPermissions>,

}
/// Secrets for GitHub Actions for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationActionsSecret {
    /// The name of the secret.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Visibility of a secret
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "selected_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,

}
/// Organization Full
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationFull {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "public_members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_members_url: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "twitter_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,

    #[serde(rename = "is_verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_verified: Option<bool>,

    #[serde(rename = "has_organization_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_organization_projects: Option<bool>,

    #[serde(rename = "has_repository_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_repository_projects: Option<bool>,

    #[serde(rename = "public_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_repos: Option<i64>,

    #[serde(rename = "public_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists: Option<i64>,

    #[serde(rename = "followers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers: Option<i64>,

    #[serde(rename = "following")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following: Option<i64>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "total_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_private_repos: Option<i64>,

    #[serde(rename = "owned_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_private_repos: Option<i64>,

    #[serde(rename = "private_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_gists: Option<i64>,

    #[serde(rename = "disk_usage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_usage: Option<i64>,

    #[serde(rename = "collaborators")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,

    #[serde(rename = "billing_email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_email: Option<String>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<OrganizationfullPlan>,

    #[serde(rename = "default_repository_permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_repository_permission: Option<String>,

    #[serde(rename = "members_can_create_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_repositories: Option<bool>,

    #[serde(rename = "two_factor_requirement_enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,

    #[serde(rename = "members_allowed_repository_creation_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,

    #[serde(rename = "members_can_create_public_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,

    #[serde(rename = "members_can_create_private_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,

    #[serde(rename = "members_can_create_internal_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,

    #[serde(rename = "members_can_create_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_pages: Option<bool>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Organization Invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "failed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_at: Option<String>,

    #[serde(rename = "failed_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_reason: Option<String>,

    #[serde(rename = "inviter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inviter: Option<SimpleUser>,

    #[serde(rename = "team_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_count: Option<i64>,

    #[serde(rename = "invitation_team_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitation_team_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "invitation_teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitation_teams_url: Option<String>,

}
/// Organization Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSimple {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "public_members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_members_url: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationfullPlan {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "space")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub space: Option<i64>,

    #[serde(rename = "private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_repos: Option<i64>,

    #[serde(rename = "filled_seats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filled_seats: Option<i64>,

    #[serde(rename = "seats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub seats: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrghookConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<String>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgmembershipPermissions {
    #[serde(rename = "can_create_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_create_repository: Option<bool>,

}
/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorghooksConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

    #[serde(rename = "username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,

}
/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorghookshookIdConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorgteamsteamSlugteamsyncgroupmappingsGroups {
    /// ID of the IdP group.
    #[serde(rename = "group_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_id: Option<String>,

    /// Name of the IdP group.
    #[serde(rename = "group_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_name: Option<String>,

    /// Description of the IdP group.
    #[serde(rename = "group_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_description: Option<String>,

}
/// A software package
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    /// Unique identifier of the package.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The name of the package.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "package_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub package_type: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// The number of versions of the package.
    #[serde(rename = "version_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version_count: Option<i64>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// A version of a software package
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Unique identifier of the package version.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The name of the package version.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "package_html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub package_html_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<PackageVersionMetadata>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionMetadata {
    #[serde(rename = "package_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub package_type: Option<String>,

    #[serde(rename = "container")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<ContainerMetadata>,

    #[serde(rename = "docker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker: Option<DockerMetadata>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackagesBillingUsage {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    #[serde(rename = "total_gigabytes_bandwidth_used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_gigabytes_bandwidth_used: Option<i64>,

    /// Total paid storage space (GB) for GitHuub Packages.
    #[serde(rename = "total_paid_gigabytes_bandwidth_used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_paid_gigabytes_bandwidth_used: Option<i64>,

    /// Free storage space (GB) for GitHub Packages.
    #[serde(rename = "included_gigabytes_bandwidth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub included_gigabytes_bandwidth: Option<i64>,

}
/// The configuration for GitHub Pages for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    /// The API address for accessing this Page resource.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The status of the most recent build of the Page.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    /// The Pages site's custom domain
    #[serde(rename = "cname")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cname: Option<String>,

    /// Whether the Page has a custom 404 page.
    #[serde(rename = "custom_404")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_404: Option<bool>,

    /// The web address the Page can be accessed from.
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "source")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<PagesSourceHash>,

    /// Whether the GitHub Pages site is publicly visible. If set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site.
    #[serde(rename = "public")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,

}
/// Page Build
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBuild {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "error")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<PagebuildError>,

    #[serde(rename = "pusher")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pusher: Option<SimpleUser>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<String>,

    #[serde(rename = "duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Page Build Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBuildStatus {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagebuildError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagesSourceHash {
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipationStats {
    #[serde(rename = "all")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub all: Option<Vec<i32>>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<Vec<i32>>,

}
/// Details of a deployment that is waiting for protection rules to pass
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingDeployment {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<PendingdeploymentEnvironment>,

    /// The set duration of the wait timer
    #[serde(rename = "wait_timer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wait_timer: Option<i64>,

    /// The time that the wait timer began.
    #[serde(rename = "wait_timer_started_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub wait_timer_started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether the currently authenticated user can approve the deployment
    #[serde(rename = "current_user_can_approve")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_can_approve: Option<bool>,

    /// The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
    #[serde(rename = "reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<PendingdeploymentReviewers>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingdeploymentEnvironment {
    /// The id of the environment.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the environment.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingdeploymentReviewers {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<DeploymentReviewerType>,

    #[serde(rename = "reviewer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewer: Option<Value>,

}
/// Porter Author
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PorterAuthor {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "remote_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_id: Option<String>,

    #[serde(rename = "remote_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "import_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_url: Option<String>,

}
/// Porter Large File
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PorterLargeFile {
    #[serde(rename = "ref_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ref_name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "oid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub oid: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

}
/// Private User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateUser {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "hireable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,

    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

    #[serde(rename = "twitter_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,

    #[serde(rename = "public_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_repos: Option<i64>,

    #[serde(rename = "public_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists: Option<i64>,

    #[serde(rename = "followers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers: Option<i64>,

    #[serde(rename = "following")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "private_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_gists: Option<i64>,

    #[serde(rename = "total_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_private_repos: Option<i64>,

    #[serde(rename = "owned_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_private_repos: Option<i64>,

    #[serde(rename = "disk_usage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_usage: Option<i64>,

    #[serde(rename = "collaborators")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,

    #[serde(rename = "two_factor_authentication")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_authentication: Option<bool>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<PrivateuserPlan>,

    #[serde(rename = "suspended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "business_plus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub business_plus: Option<bool>,

    #[serde(rename = "ldap_dn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateuserPlan {
    #[serde(rename = "collaborators")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "space")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub space: Option<i64>,

    #[serde(rename = "private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_repos: Option<i64>,

}
/// Projects are a way to organize columns and cards of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "owner_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "columns_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub columns_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// Name of the project
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Body of the project
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// State of the project; either 'open' or 'closed'
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
    #[serde(rename = "organization_permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,

    /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

}
/// Project cards represent a scope of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCard {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The project card's ID
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "note")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether or not the card is archived
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "column_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub column_url: Option<String>,

    #[serde(rename = "content_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_url: Option<String>,

    #[serde(rename = "project_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_url: Option<String>,

}
/// Project columns contain cards of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectColumn {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "project_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_url: Option<String>,

    #[serde(rename = "cards_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cards_url: Option<String>,

    /// The unique identifier of the project column
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// Name of the project column
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Branch protections protect branches
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranch {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "required_status_checks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_status_checks: Option<StatusCheckPolicy>,

    #[serde(rename = "required_pull_request_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedbranchRequiredPullRequestReviews>,

    #[serde(rename = "required_signatures")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_signatures: Option<ProtectedbranchRequiredSignatures>,

    #[serde(rename = "enforce_admins")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforce_admins: Option<ProtectedbranchEnforceAdmins>,

    #[serde(rename = "required_linear_history")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<ProtectedbranchRequiredLinearHistory>,

    #[serde(rename = "allow_force_pushes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<ProtectedbranchRequiredLinearHistory>,

    #[serde(rename = "allow_deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<ProtectedbranchRequiredLinearHistory>,

    #[serde(rename = "restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,

}
/// Protected Branch Admin Enforced
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchAdminEnforced {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

}
/// Protected Branch Pull Request Review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchPullRequestReview {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "dismissal_restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ProtectedbranchpullrequestreviewDismissalRestrictions>,

    #[serde(rename = "dismiss_stale_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,

    #[serde(rename = "require_code_owner_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,

    #[serde(rename = "required_approving_review_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<u8>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchEnforceAdmins {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredLinearHistory {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredPullRequestReviews {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "dismiss_stale_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,

    #[serde(rename = "require_code_owner_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,

    #[serde(rename = "required_approving_review_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,

    #[serde(rename = "dismissal_restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ProtectedbranchRequiredPullRequestReviewsDismissalRestrictions>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredPullRequestReviewsDismissalRestrictions {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "users_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<SimpleUser>>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<Team>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredSignatures {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchpullrequestreviewDismissalRestrictions {
    /// The list of users with review dismissal access.
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<SimpleUser>>,

    /// The list of teams with review dismissal access.
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<Team>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "users_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

}
/// Public User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "hireable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,

    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

    #[serde(rename = "twitter_username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,

    #[serde(rename = "public_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_repos: Option<i64>,

    #[serde(rename = "public_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists: Option<i64>,

    #[serde(rename = "followers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers: Option<i64>,

    #[serde(rename = "following")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<PrivateuserPlan>,

    #[serde(rename = "suspended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "private_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_gists: Option<i64>,

    #[serde(rename = "total_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_private_repos: Option<i64>,

    #[serde(rename = "owned_private_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_private_repos: Option<i64>,

    #[serde(rename = "disk_usage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_usage: Option<i64>,

    #[serde(rename = "collaborators")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,

}
/// Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "diff_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_url: Option<String>,

    #[serde(rename = "patch_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch_url: Option<String>,

    #[serde(rename = "issue_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "review_comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comments_url: Option<String>,

    #[serde(rename = "review_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comment_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    /// Number uniquely identifying the pull request within its repository.
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "locked")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,

    /// The title of the pull request.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<PullrequestLabels>>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Milestone>,

    #[serde(rename = "active_lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "merged_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "merge_commit_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_commit_sha: Option<String>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,

    #[serde(rename = "requested_reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewers: Option<Vec<SimpleUser>>,

    #[serde(rename = "requested_teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_teams: Option<Vec<TeamSimple>>,

    #[serde(rename = "head")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<PullrequestHead>,

    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<PullrequestBase>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<PullrequestsimpleLinks>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "auto_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_merge: Option<AutoMerge>,

    /// Indicates whether or not the pull request is a draft.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    #[serde(rename = "merged")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged: Option<bool>,

    #[serde(rename = "mergeable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mergeable: Option<bool>,

    #[serde(rename = "rebaseable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rebaseable: Option<bool>,

    #[serde(rename = "mergeable_state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mergeable_state: Option<String>,

    #[serde(rename = "merged_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_by: Option<SimpleUser>,

    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,

    #[serde(rename = "review_comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comments: Option<i64>,

    /// Indicates whether maintainers can modify the pull request.
    #[serde(rename = "maintainer_can_modify")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainer_can_modify: Option<bool>,

    #[serde(rename = "commits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits: Option<i64>,

    #[serde(rename = "additions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,

    #[serde(rename = "deletions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,

    #[serde(rename = "changed_files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub changed_files: Option<i64>,

}
/// Pull Request Merge Result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMergeResult {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "merged")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged: Option<bool>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "head")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<PullrequestminimalHead>,

    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<PullrequestminimalHead>,

}
/// Pull Request Reviews are reviews on pull requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReview {
    /// Unique identifier of the review
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    /// The text of the review.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "pull_request_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_url: Option<String>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<PullrequestreviewLinks>,

    #[serde(rename = "submitted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,

    /// A commit SHA for the review.
    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

}
/// Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewComment {
    /// URL for the pull request review comment
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The ID of the pull request review to which the comment belongs.
    #[serde(rename = "pull_request_review_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_review_id: Option<i64>,

    /// The ID of the pull request review comment.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The node ID of the pull request review comment.
    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The diff of the line that the comment refers to.
    #[serde(rename = "diff_hunk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_hunk: Option<String>,

    /// The relative path of the file to which the comment applies.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// The line index in the diff to which the comment applies.
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    /// The index of the original line in the diff to which the comment applies.
    #[serde(rename = "original_position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_position: Option<i64>,

    /// The SHA of the commit to which the comment applies.
    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    /// The SHA of the original commit to which the comment applies.
    #[serde(rename = "original_commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_commit_id: Option<String>,

    /// The comment ID to reply to.
    #[serde(rename = "in_reply_to_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to_id: Option<i64>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    /// The text of the comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// HTML URL for the pull request review comment.
    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// URL for the pull request that the review comment belongs to.
    #[serde(rename = "pull_request_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_url: Option<String>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<PullrequestreviewcommentLinks>,

    /// The first line of the range for a multi-line comment.
    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    /// The first line of the range for a multi-line comment.
    #[serde(rename = "original_start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_start_line: Option<i64>,

    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "start_side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,

    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "original_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_line: Option<i64>,

    /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
    #[serde(rename = "side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

}
/// Pull Request Review Request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewRequest {
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<SimpleUser>>,

    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<TeamSimple>>,

}
/// Pull Request Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "diff_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_url: Option<String>,

    #[serde(rename = "patch_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch_url: Option<String>,

    #[serde(rename = "issue_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "review_comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comments_url: Option<String>,

    #[serde(rename = "review_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comment_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "locked")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub locked: Option<bool>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<PullrequestsimpleLabels>>,

    #[serde(rename = "milestone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Milestone>,

    #[serde(rename = "active_lock_reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "closed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "merged_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "merge_commit_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_commit_sha: Option<String>,

    #[serde(rename = "assignee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,

    #[serde(rename = "assignees")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,

    #[serde(rename = "requested_reviewers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewers: Option<Vec<SimpleUser>>,

    #[serde(rename = "requested_teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_teams: Option<Vec<TeamSimple>>,

    #[serde(rename = "head")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head: Option<PullrequestsimpleHead>,

    #[serde(rename = "base")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<PullrequestsimpleHead>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<PullrequestsimpleLinks>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "auto_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_merge: Option<AutoMerge>,

    /// Indicates whether or not the pull request is a draft.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestBase {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<PullrequestBaseRepo>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<PullrequestHeadRepoOwner>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestBaseRepo {
    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<PullrequestHeadRepoOwner>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHead {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<PullrequestHeadRepo>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<PullrequestHeadRepoOwner>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepo {
    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<PullrequestHeadRepoOwner>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<PullrequestHeadRepoLicense>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepoLicense {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "spdx_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub spdx_id: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepoOwner {
    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestLabels {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestminimalHead {
    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<PullrequestminimalHeadRepo>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestminimalHeadRepo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewLinks {
    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<PullrequestreviewLinksHtml>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<PullrequestreviewLinksHtml>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewLinksHtml {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinks {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<PullrequestreviewcommentLinksSelf>,

    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<PullrequestreviewcommentLinksHtml>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<PullrequestreviewcommentLinksPullRequest>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksHtml {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksPullRequest {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksSelf {
    #[serde(rename = "href")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub href: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleHead {
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<Repository>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleLabels {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleLinks {
    #[serde(rename = "comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<Link>,

    #[serde(rename = "commits")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits: Option<Link>,

    #[serde(rename = "statuses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<Link>,

    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<Link>,

    #[serde(rename = "issue")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<Link>,

    #[serde(rename = "review_comments")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comments: Option<Link>,

    #[serde(rename = "review_comment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_comment: Option<Link>,

    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<Link>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "remaining")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub remaining: Option<i64>,

    #[serde(rename = "reset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reset: Option<i64>,

}
/// Rate Limit Overview
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitOverview {
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<RatelimitoverviewResources>,

    #[serde(rename = "rate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rate: Option<RateLimit>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatelimitoverviewResources {
    #[serde(rename = "core")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub core: Option<RateLimit>,

    #[serde(rename = "graphql")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub graphql: Option<RateLimit>,

    #[serde(rename = "search")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub search: Option<RateLimit>,

    #[serde(rename = "source_import")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_import: Option<RateLimit>,

    #[serde(rename = "integration_manifest")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub integration_manifest: Option<RateLimit>,

    #[serde(rename = "code_scanning_upload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_scanning_upload: Option<RateLimit>,

}
/// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    /// The reaction to use
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionRollup {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,

    #[serde(rename = "+1")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plus_1: Option<i64>,

    #[serde(rename = "-1")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub minus_1: Option<i64>,

    #[serde(rename = "laugh")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub laugh: Option<i64>,

    #[serde(rename = "confused")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub confused: Option<i64>,

    #[serde(rename = "heart")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub heart: Option<i64>,

    #[serde(rename = "hooray")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooray: Option<i64>,

    #[serde(rename = "eyes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub eyes: Option<i64>,

    #[serde(rename = "rocket")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rocket: Option<i64>,

}
/// Referrer Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerTraffic {
    #[serde(rename = "referrer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub referrer: Option<String>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "uniques")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uniques: Option<i64>,

}
/// A release.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "assets_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assets_url: Option<String>,

    #[serde(rename = "upload_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upload_url: Option<String>,

    #[serde(rename = "tarball_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tarball_url: Option<String>,

    #[serde(rename = "zipball_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zipball_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the tag.
    #[serde(rename = "tag_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag_name: Option<String>,

    /// Specifies the commitish value that determines where the Git tag is created from.
    #[serde(rename = "target_commitish")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    /// true to create a draft (unpublished) release, false to create a published one.
    #[serde(rename = "draft")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,

    /// Whether to identify the release as a prerelease or a full release.
    #[serde(rename = "prerelease")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "published_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    #[serde(rename = "assets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assets: Option<Vec<ReleaseAsset>>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

}
/// Data related to a release.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseAsset {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "browser_download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub browser_download_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The file name of the asset.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// State of the release asset.
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "download_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_count: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "uploader")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uploader: Option<SimpleUser>,

}
/// Repo Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoSearchResultItem {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    /// Returns whether or not this repository disabled.
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

}
/// A git repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier of the repository
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    /// Whether the repository is private or public.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    /// The default branch of the repository.
    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    /// Whether issues are enabled.
    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    /// Whether projects are enabled.
    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    /// Whether the wiki is enabled.
    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    /// Whether the repository is archived.
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    /// Returns whether or not this repository disabled.
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    /// The repository visibility: public, private, or internal.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether to allow rebase merges for pull requests.
    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "template_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<RepositoryTemplateRepository>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    /// Whether to allow squash merges for pull requests.
    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    /// Whether to delete head branches when pull requests are merged
    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    /// Whether to allow merge commits for pull requests.
    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "subscribers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,

    #[serde(rename = "network_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

    #[serde(rename = "starred_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<String>,

}
/// Repository Collaborator Permission
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCollaboratorPermission {
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

}
/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryInvitation {
    /// Unique identifier of the repository invitation.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "invitee")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitee: Option<SimpleUser>,

    #[serde(rename = "inviter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inviter: Option<SimpleUser>,

    /// The permission associated with the invitation.
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether or not the invitation has expired
    #[serde(rename = "expired")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expired: Option<bool>,

    /// URL for the repository invitation
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    #[serde(rename = "admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,

    #[serde(rename = "pull")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull: Option<bool>,

    #[serde(rename = "triage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub triage: Option<bool>,

    #[serde(rename = "push")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push: Option<bool>,

    #[serde(rename = "maintain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintain: Option<bool>,

}
/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositorySubscription {
    /// Determines if notifications should be received from this repository.
    #[serde(rename = "subscribed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribed: Option<bool>,

    /// Determines if all notifications should be blocked from this repository.
    #[serde(rename = "ignored")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepository {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<RepositoryTemplateRepositoryOwner>,

    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryTemplateRepositoryPermissions>,

    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "subscribers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,

    #[serde(rename = "network_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepositoryOwner {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepositoryPermissions {
    #[serde(rename = "admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,

    #[serde(rename = "push")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push: Option<bool>,

    #[serde(rename = "pull")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull: Option<bool>,

}
/// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredPullRequestReviews {
    #[serde(rename = "dismissal_restrictions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions>,

    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(rename = "dismiss_stale_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,

    /// Blocks merging pull requests until [code owners](https://help.github.com/articles/about-code-owners/) review them.
    #[serde(rename = "require_code_owner_reviews")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,

    /// Specify the number of reviewers required to approve pull requests. Use a number between 1 and 6.
    #[serde(rename = "required_approving_review_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,

}
/// Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions {
    /// The list of user `login`s with dismissal access
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,

    /// The list of team `slug`s with dismissal access
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,

}
/// Require status checks to pass before merging. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredStatusChecks {
    /// Require branches to be up to date before merging.
    #[serde(rename = "strict")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub strict: Option<bool>,

    /// The list of status checks to require in order to merge into this branch
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

}
/// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRestrictions {
    /// The list of user `login`s with push access
    #[serde(rename = "users")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,

    /// The list of team `slug`s with push access
    #[serde(rename = "teams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,

    /// The list of app `slug`s with push access
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsActions {
    /// The text to be displayed on a button in the web UI. The maximum size is 20 characters.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// A short explanation of what this action would do. The maximum size is 40 characters.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// A reference for the action on the integrator's system. The maximum size is 20 characters.
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identifier: Option<String>,

}
/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutput {
    /// The title of the check run.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// The summary of the check run. This parameter supports Markdown.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,

    /// The details of the check run. This parameter supports Markdown.
    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

    /// Adds information from your analysis to specific lines of code. Annotations are visible on GitHub in the **Checks** and **Files changed** tab of the pull request. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about how you can view annotations on GitHub, see \"[About status checks](https://help.github.com/articles/about-status-checks#checks)\". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object) description for details about how to use this parameter.
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations: Option<Vec<ReposownerrepocheckrunsOutputAnnotations>>,

    /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#images-object) description for details.
    #[serde(rename = "images")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<Vec<ReposownerrepocheckrunsOutputImages>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutputAnnotations {
    /// The path of the file to add an annotation to. For example, `assets/css/main.css`.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// The start line of the annotation.
    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    /// The end line of the annotation.
    #[serde(rename = "end_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_line: Option<i64>,

    /// The start column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
    #[serde(rename = "start_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_column: Option<i64>,

    /// The end column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
    #[serde(rename = "end_column")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_column: Option<i64>,

    /// The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
    #[serde(rename = "annotation_level")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation_level: Option<String>,

    /// A short description of the feedback for these lines of code. The maximum size is 64 KB.
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    /// The title that represents the annotation. The maximum size is 255 characters.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// Details about this annotation. The maximum size is 64 KB.
    #[serde(rename = "raw_details")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_details: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutputImages {
    /// The alternative text for the image.
    #[serde(rename = "alt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alt: Option<String>,

    /// The full URL of the image.
    #[serde(rename = "image_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url: Option<String>,

    /// A short image description.
    #[serde(rename = "caption")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,

}
/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunscheckRunIdOutput {
    /// **Required**.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    /// Can contain Markdown.
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,

    /// Can contain Markdown.
    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

    /// Adds information from your analysis to specific lines of code. Annotations are visible in GitHub's pull request UI. Annotations are visible in GitHub's pull request UI. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about annotations in the UI, see \"[About status checks](https://help.github.com/articles/about-status-checks#checks)\". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations: Option<Vec<ReposownerrepocheckrunsOutputAnnotations>>,

    /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
    #[serde(rename = "images")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<Vec<ReposownerrepocheckrunsOutputImages>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepochecksuitespreferencesAutoTriggerChecks {
    /// The `id` of the GitHub App.
    #[serde(rename = "app_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub app_id: Option<i64>,

    /// Set to `true` to enable automatic creation of CheckSuite events upon pushes to the repository, or `false` to disable them.
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub setting: Option<bool>,

}
/// The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathAuthor {
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
/// object containing information about the author.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathAuthor1 {
    /// The name of the author (or committer) of the commit
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author (or committer) of the commit
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

}
/// The person that committed the file. Default: the authenticated user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathCommitter {
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
/// object containing information about the committer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathCommitter1 {
    /// The name of the author (or committer) of the commit
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author (or committer) of the commit
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepoenvironmentsenvironmentNameReviewers {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<DeploymentReviewerType>,

    /// The id of the user or team who can review the deployment
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

}
/// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogitcommitsAuthor {
    /// The name of the author (or committer) of the commit
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author (or committer) of the commit
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
/// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogitcommitsCommitter {
    /// The name of the author (or committer) of the commit
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author (or committer) of the commit
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
/// An object with information about the individual creating the tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogittagsTagger {
    /// The name of the author of the tag
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The email of the author of the tag
    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    /// When this object was tagged. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogittreesTree {
    /// The file referenced in the tree.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,

    /// Either `blob`, `tree`, or `commit`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    /// The SHA1 checksum ID of the object in the tree. Also called `tree.sha`. If the value is `null` then the file will be deleted.      **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    /// The content you want this file to have. GitHub will write this blob out and use that SHA for this entry. Use either this, or `tree.sha`.      **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(rename = "content")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

}
/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepohooksConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

    #[serde(rename = "token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,

    #[serde(rename = "digest")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub digest: Option<String>,

}
/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepohookshookIdConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

    #[serde(rename = "address")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "room")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub room: Option<String>,

}
/// The source branch and directory used to publish your Pages site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepopagesSource {
    /// The repository branch used to publish your site's source files.
    #[serde(rename = "branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,

    /// The repository directory that includes the source files for the Pages site. Allowed paths are `/` or `/docs`. Default: `/`
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepopullspullNumberreviewsComments {
    /// The relative path to the file that necessitates a review comment.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    /// The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. For help finding the position value, read the note below.
    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    /// Text of the review comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

    #[serde(rename = "side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,

    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    #[serde(rename = "start_side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,

}
/// Legacy Review Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewComment {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "pull_request_review_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_review_id: Option<i64>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "diff_hunk")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub diff_hunk: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,

    #[serde(rename = "original_position")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_position: Option<i64>,

    #[serde(rename = "commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,

    #[serde(rename = "original_commit_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_commit_id: Option<String>,

    #[serde(rename = "in_reply_to_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to_id: Option<i64>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "pull_request_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_url: Option<String>,

    #[serde(rename = "author_association")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,

    #[serde(rename = "_links")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<ReviewcommentLinks>,

    #[serde(rename = "body_text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,

    /// The side of the first line of the range for a multi-line comment.
    #[serde(rename = "start_side")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,

    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,

    /// The original line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(rename = "original_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_line: Option<i64>,

    /// The first line of the range for a multi-line comment.
    #[serde(rename = "start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,

    /// The original first line of the range for a multi-line comment.
    #[serde(rename = "original_start_line")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_start_line: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewcommentLinks {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<Link>,

    #[serde(rename = "html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<Link>,

    #[serde(rename = "pull_request")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<Link>,

}
/// A self hosted runner
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Runner {
    /// The id of the runner.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The name of the runner.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The Operating System of the runner.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub os: Option<String>,

    /// The status of the runner.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "busy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub busy: Option<bool>,

    #[serde(rename = "labels")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<RunnerLabels>>,

}
/// Runner Application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerApplication {
    #[serde(rename = "os")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub os: Option<String>,

    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub architecture: Option<String>,

    #[serde(rename = "download_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,

    #[serde(rename = "filename")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerGroupsEnterprise {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<f64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

    #[serde(rename = "selected_organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organizations_url: Option<String>,

    #[serde(rename = "runners_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners_url: Option<String>,

    #[serde(rename = "allows_public_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allows_public_repositories: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerGroupsOrg {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<f64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "default")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,

    /// Link to the selected repositories resource for this runner group. Not present unless visibility was set to `selected`
    #[serde(rename = "selected_repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,

    #[serde(rename = "runners_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners_url: Option<String>,

    #[serde(rename = "inherited")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inherited: Option<bool>,

    #[serde(rename = "inherited_allows_public_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inherited_allows_public_repositories: Option<bool>,

    #[serde(rename = "allows_public_repositories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allows_public_repositories: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerLabels {
    /// Unique identifier of the label.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// Name of the label.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// The type of label. Read-only labels are applied automatically when the runner is configured.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimEnterpriseGroup {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<ScimgrouplistenterpriseMembers>>,

    #[serde(rename = "meta")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimEnterpriseUser {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ScimuserlistenterpriseName>,

    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<ScimenterpriseuserEmails>>,

    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "meta")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,

}
/// Scim Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "detail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,

    #[serde(rename = "scimType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scim_type: Option<String>,

    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimGroupListEnterprise {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_results: Option<f64>,

    #[serde(rename = "itemsPerPage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items_per_page: Option<f64>,

    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_index: Option<f64>,

    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<Vec<ScimgrouplistenterpriseResources>>,

}
/// SCIM /Users provisioning endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUser {
    /// SCIM schema used.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    /// Unique identifier of an external identity
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    /// The ID of the User.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ScimuserName>,

    /// user emails
    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<ScimuserEmails>>,

    /// The active status of the User.
    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "meta")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimuserMeta>,

    /// The ID of the organization.
    #[serde(rename = "organization_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_id: Option<i64>,

    /// Set of operations to be performed
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operations: Option<Vec<ScimuserOperations>>,

    /// associated groups
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<ScimuserGroups>>,

}
/// SCIM User List
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUserList {
    /// SCIM schema used.
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_results: Option<i64>,

    #[serde(rename = "itemsPerPage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items_per_page: Option<i64>,

    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_index: Option<i64>,

    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<Vec<ScimUser>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUserListEnterprise {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "totalResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_results: Option<f64>,

    #[serde(rename = "itemsPerPage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items_per_page: Option<f64>,

    #[serde(rename = "startIndex")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_index: Option<f64>,

    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resources: Option<Vec<ScimuserlistenterpriseResources>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimenterpriseuserEmails {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseMembers {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,

    #[serde(rename = "display")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseMeta {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<String>,

    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified: Option<String>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseResources {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<ScimgrouplistenterpriseMembers>>,

    #[serde(rename = "meta")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserEmails {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserGroups {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "display")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserMeta {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserName {
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub given_name: Option<String>,

    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_name: Option<String>,

    #[serde(rename = "formatted")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub formatted: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserOperations {
    #[serde(rename = "op")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub op: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserlistenterpriseEmails {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserlistenterpriseName {
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub given_name: Option<String>,

    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserlistenterpriseResources {
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,

    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ScimuserlistenterpriseName>,

    #[serde(rename = "emails")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<ScimuserlistenterpriseEmails>>,

    #[serde(rename = "groups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,

    #[serde(rename = "active")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,

    #[serde(rename = "meta")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseGroupsMembers {
    /// The SCIM user ID for a user.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseGroupsscimGroupIdOperations {
    #[serde(rename = "op")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub op: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersEmails {
    /// The email address.
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    /// The type of email address.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    /// Whether this email address is the primary address.
    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersGroups {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersName {
    /// The first name of the user.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub given_name: Option<String>,

    /// The last name of the user.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_name: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersEmails {
    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersName {
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub given_name: Option<String>,

    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub family_name: Option<String>,

    #[serde(rename = "formatted")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub formatted: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersscimUserIdEmails {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "primary")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersscimUserIdOperations {
    #[serde(rename = "op")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub op: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopedInstallation {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<AppPermissions>,

    /// Describe whether all repositories have been selected or there's a selection involved
    #[serde(rename = "repository_selection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,

    #[serde(rename = "single_file_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_name: Option<String>,

    #[serde(rename = "has_multiple_single_files")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,

    #[serde(rename = "single_file_paths")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<SimpleUser>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultTextMatches {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchresulttextmatchesInner {
    #[serde(rename = "object_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_url: Option<String>,

    #[serde(rename = "object_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_type: Option<String>,

    #[serde(rename = "property")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub property: Option<String>,

    #[serde(rename = "fragment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fragment: Option<String>,

    #[serde(rename = "matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub matches: Option<Vec<Value>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlert {
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<AlertNumber>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<AlertCreatedAt>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<AlertUrl>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<AlertHtmlUrl>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<SecretScanningAlertState>,

    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,

    /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(rename = "resolved_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "resolved_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolved_by: Option<SimpleUser>,

    /// The type of secret that secret scanning detected.
    #[serde(rename = "secret_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_type: Option<String>,

    /// The secret that was detected.
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<String>,

}
/// **Required when the `state` is `resolved`.** The reason for resolving the alert. Can be one of `false_positive`, `wont_fix`, `revoked`, or `used_in_tests`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlertResolution {
}
/// Sets the state of the secret scanning alert. Can be either `open` or `resolved`. You must provide `resolution` when you set the state to `resolved`.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum SecretScanningAlertState { 
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "resolved")]
    RESOLVED,
}

impl Display for SecretScanningAlertState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self { 
            SecretScanningAlertState::OPEN => write!(f, "{}", "open"),
            SecretScanningAlertState::RESOLVED => write!(f, "{}", "resolved"),
        }
    }
}

impl std::str::FromStr for SecretScanningAlertState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(SecretScanningAlertState::OPEN),
            "resolved" => Ok(SecretScanningAlertState::RESOLVED),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetAllowedActionsRepository {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    #[serde(rename = "github_owned_allowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub github_owned_allowed: Option<bool>,

    /// Whether actions in GitHub Marketplace from verified creators are allowed. Set to `true` to allow all GitHub Marketplace actions by verified creators.
    #[serde(rename = "verified_allowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified_allowed: Option<bool>,

    /// Specifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/_*`.\"
    #[serde(rename = "patterns_allowed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub patterns_allowed: Option<Vec<String>>,

}
/// The API URL to use to get or set the actions that are allowed to run, when `allowed_actions` is set to `selected`.
pub type SelectedActionsUrl = String;
/// Short Blob
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortBlob {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

}
/// Short Branch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortBranch {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<ShortbranchCommit>,

    #[serde(rename = "protected")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protected: Option<bool>,

    #[serde(rename = "protection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection: Option<BranchProtection>,

    #[serde(rename = "protection_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortbranchCommit {
    #[serde(rename = "sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

}
/// Simple Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleCommit {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "tree_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree_id: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimplecommitAuthor>,

    #[serde(rename = "committer")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<SimplecommitAuthor>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleCommitStatus {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,

    #[serde(rename = "target_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,

    #[serde(rename = "required")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<bool>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Simple User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "starred_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplecommitAuthor {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

}
/// Stargazer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stargazer {
    #[serde(rename = "starred_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "user")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,

}
/// Starred Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarredRepository {
    #[serde(rename = "starred_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "repo")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<Repository>,

}
/// The status of a commit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "target_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,

    #[serde(rename = "context")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

}
/// Status Check Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCheckPolicy {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "strict")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub strict: Option<bool>,

    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,

    #[serde(rename = "contexts_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts_url: Option<String>,

}
/// Tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<ShortbranchCommit>,

    #[serde(rename = "zipball_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub zipball_url: Option<String>,

    #[serde(rename = "tarball_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tarball_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

}
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<TeamSimple>,

}
/// A team discussion is a persistent record of a free-form conversation within a team.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamDiscussion {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    /// The main text of the discussion.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
    #[serde(rename = "body_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_version: Option<String>,

    #[serde(rename = "comments_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_count: Option<i64>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "last_edited_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_edited_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The unique sequence number of a team discussion.
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    /// Whether or not this discussion should be pinned for easy retrieval.
    #[serde(rename = "pinned")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pinned: Option<bool>,

    /// Whether or not this discussion should be restricted to team members and organization administrators.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "team_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_url: Option<String>,

    /// The title of the discussion.
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

}
/// A reply to a discussion within a team.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamDiscussionComment {
    #[serde(rename = "author")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<SimpleUser>,

    /// The main text of the comment.
    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "body_html")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,

    /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
    #[serde(rename = "body_version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_version: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "last_edited_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_edited_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "discussion_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub discussion_url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The unique sequence number of a team discussion comment.
    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,

}
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamFull {
    /// Unique identifier of the team
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// URL for the team
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    /// Name of the team
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// The level of privacy this team should have
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    /// Permission that the team will have for its repositories
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<TeamSimple>,

    #[serde(rename = "members_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_count: Option<i64>,

    #[serde(rename = "repos_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_count: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "organization")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<OrganizationFull>,

    /// Distinguished Name (DN) that team maps to within LDAP environment
    #[serde(rename = "ldap_dn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,

}
/// Team Membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamMembership {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    /// The role of the user in the team.
    #[serde(rename = "role")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

}
/// A team's access to a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamProject {
    #[serde(rename = "owner_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_url: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "columns_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub columns_url: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "body")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "creator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator: Option<SimpleUser>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    /// The organization permission for this project. Only present when owner is an organization.
    #[serde(rename = "organization_permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,

    /// Whether the project is private or not. Only present when owner is an organization.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<TeamprojectPermissions>,

}
/// A team's access to a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRepository {
    /// Unique identifier of the repository
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// The name of the repository.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "full_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<LicenseSimple>,

    #[serde(rename = "forks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,

    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,

    #[serde(rename = "owner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,

    /// Whether the repository is private or public.
    #[serde(rename = "private")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "fork")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "archive_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,

    #[serde(rename = "assignees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,

    #[serde(rename = "blobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,

    #[serde(rename = "branches_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,

    #[serde(rename = "collaborators_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,

    #[serde(rename = "comments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,

    #[serde(rename = "commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,

    #[serde(rename = "compare_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,

    #[serde(rename = "contents_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,

    #[serde(rename = "contributors_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,

    #[serde(rename = "deployments_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,

    #[serde(rename = "downloads_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "forks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,

    #[serde(rename = "git_commits_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,

    #[serde(rename = "git_refs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,

    #[serde(rename = "git_tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,

    #[serde(rename = "git_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,

    #[serde(rename = "issue_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,

    #[serde(rename = "issue_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,

    #[serde(rename = "issues_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,

    #[serde(rename = "keys_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,

    #[serde(rename = "labels_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,

    #[serde(rename = "languages_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,

    #[serde(rename = "merges_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,

    #[serde(rename = "milestones_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,

    #[serde(rename = "notifications_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,

    #[serde(rename = "pulls_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,

    #[serde(rename = "releases_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,

    #[serde(rename = "ssh_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,

    #[serde(rename = "stargazers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,

    #[serde(rename = "statuses_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,

    #[serde(rename = "subscribers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

    #[serde(rename = "tags_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,

    #[serde(rename = "teams_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,

    #[serde(rename = "trees_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,

    #[serde(rename = "clone_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,

    #[serde(rename = "mirror_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,

    #[serde(rename = "hooks_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,

    #[serde(rename = "svn_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,

    #[serde(rename = "homepage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "forks_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,

    #[serde(rename = "stargazers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,

    #[serde(rename = "watchers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,

    #[serde(rename = "size")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,

    /// The default branch of the repository.
    #[serde(rename = "default_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,

    #[serde(rename = "open_issues_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,

    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(rename = "is_template")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,

    #[serde(rename = "topics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,

    /// Whether issues are enabled.
    #[serde(rename = "has_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,

    /// Whether projects are enabled.
    #[serde(rename = "has_projects")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,

    /// Whether the wiki is enabled.
    #[serde(rename = "has_wiki")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,

    #[serde(rename = "has_pages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,

    /// Whether downloads are enabled.
    #[serde(rename = "has_downloads")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,

    /// Whether the repository is archived.
    #[serde(rename = "archived")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,

    /// Returns whether or not this repository disabled.
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,

    /// The repository visibility: public, private, or internal.
    #[serde(rename = "visibility")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,

    #[serde(rename = "pushed_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Whether to allow rebase merges for pull requests.
    #[serde(rename = "allow_rebase_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,

    #[serde(rename = "template_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,

    #[serde(rename = "temp_clone_token")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,

    /// Whether to allow squash merges for pull requests.
    #[serde(rename = "allow_squash_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,

    /// Whether to delete head branches when pull requests are merged
    #[serde(rename = "delete_branch_on_merge")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,

    /// Whether to allow merge commits for pull requests.
    #[serde(rename = "allow_merge_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,

    #[serde(rename = "subscribers_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,

    #[serde(rename = "network_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,

    #[serde(rename = "open_issues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,

    #[serde(rename = "watchers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,

    #[serde(rename = "master_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,

}
/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSimple {
    /// Unique identifier of the team
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    /// URL for the team
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "members_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,

    /// Name of the team
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    /// Description of the team
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    /// Permission that the team will have for its repositories
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,

    /// The level of privacy this team should have
    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "repositories_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,

    #[serde(rename = "slug")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,

    /// Distinguished Name (DN) that team maps to within LDAP environment
    #[serde(rename = "ldap_dn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamprojectPermissions {
    #[serde(rename = "read")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<bool>,

    #[serde(rename = "write")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub write: Option<bool>,

    #[serde(rename = "admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamsteamIdteamsyncgroupmappingsGroups {
    /// ID of the IdP group.
    #[serde(rename = "group_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_id: Option<String>,

    /// Name of the IdP group.
    #[serde(rename = "group_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_name: Option<String>,

    /// Description of the IdP group.
    #[serde(rename = "group_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub group_description: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}
/// Thread
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "subject")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subject: Option<ThreadSubject>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "unread")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unread: Option<bool>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(rename = "last_read_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_read_at: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "subscription_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadSubject {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "latest_comment_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub latest_comment_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

}
/// Thread Subscription
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadSubscription {
    #[serde(rename = "subscribed")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribed: Option<bool>,

    #[serde(rename = "ignored")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "thread_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub thread_url: Option<String>,

    #[serde(rename = "repository_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,

}
/// A topic aggregates entities that are related to a subject.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<String>>,

}
/// Topic Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicSearchResultItem {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "display_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "short_description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_description: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "created_by")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_by: Option<String>,

    #[serde(rename = "released")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub released: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "featured")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub featured: Option<bool>,

    #[serde(rename = "curated")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub curated: Option<bool>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "repository_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_count: Option<i64>,

    #[serde(rename = "logo_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

    #[serde(rename = "related")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub related: Option<Vec<TopicsearchresultitemRelated>>,

    #[serde(rename = "aliases")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub aliases: Option<Vec<TopicsearchresultitemRelated>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsearchresultitemRelated {
    #[serde(rename = "topic_relation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_relation: Option<TopicsearchresultitemTopicRelation>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsearchresultitemTopicRelation {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "topic_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_id: Option<i64>,

    #[serde(rename = "relation_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub relation_type: Option<String>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Traffic {
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "uniques")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uniques: Option<i64>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

}
/// User Marketplace Purchase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMarketplacePurchase {
    #[serde(rename = "billing_cycle")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_cycle: Option<String>,

    #[serde(rename = "next_billing_date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_billing_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "unit_count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_count: Option<i64>,

    #[serde(rename = "on_free_trial")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub on_free_trial: Option<bool>,

    #[serde(rename = "free_trial_ends_on")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub free_trial_ends_on: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "account")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account: Option<MarketplaceAccount>,

    #[serde(rename = "plan")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,

}
/// User Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchResultItem {
    #[serde(rename = "login")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "avatar_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(rename = "gravatar_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "followers_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,

    #[serde(rename = "subscriptions_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,

    #[serde(rename = "organizations_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,

    #[serde(rename = "repos_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,

    #[serde(rename = "received_events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,

    #[serde(rename = "score")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub score: Option<i64>,

    #[serde(rename = "following_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,

    #[serde(rename = "gists_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,

    #[serde(rename = "starred_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,

    #[serde(rename = "events_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,

    #[serde(rename = "public_repos")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_repos: Option<i64>,

    #[serde(rename = "public_gists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists: Option<i64>,

    #[serde(rename = "followers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers: Option<i64>,

    #[serde(rename = "following")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub following: Option<i64>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "bio")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "location")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "site_admin")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,

    #[serde(rename = "hireable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,

    #[serde(rename = "text_matches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,

    #[serde(rename = "blog")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,

    #[serde(rename = "company")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,

    #[serde(rename = "suspended_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Validation Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<ValidationerrorErrors>>,

}
/// Validation Error Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationErrorSimple {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "documentation_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,

    #[serde(rename = "errors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<String>>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationerrorErrors {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "field")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,

    #[serde(rename = "message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,

    #[serde(rename = "code")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,

    #[serde(rename = "index")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index: Option<i64>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    #[serde(rename = "verified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<String>,

    #[serde(rename = "signature")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,

}
/// View Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewTraffic {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "uniques")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub uniques: Option<i64>,

    #[serde(rename = "views")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub views: Option<Vec<Traffic>>,

}
/// The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days).
pub type WaitTimer = i32;
/// Configuration object of the webhook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookConfig {
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,

    #[serde(rename = "content_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,

    #[serde(rename = "secret")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,

    #[serde(rename = "insecure_ssl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,

}
/// The media type used to serialize the payloads. Supported values include `json` and `form`. The default is `form`.
pub type WebhookConfigContentType = String;
/// Determines whether the SSL certificate of the host for `url` will be verified when delivering payloads. Supported values include `0` (verification is performed) and `1` (verification is not performed). The default is `0`. **We strongly recommend not setting this to `1` as you are subject to man-in-the-middle and other attacks.**
pub type WebhookConfigInsecureSsl = String;
/// If provided, the `secret` will be used as the `key` to generate the HMAC hex digest value for [delivery signature headers](https://docs.github.com/webhooks/event-payloads/#delivery-headers).
pub type WebhookConfigSecret = String;
/// The URL to which the payloads will be delivered.
pub type WebhookConfigUrl = String;
/// A GitHub Actions workflow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "badge_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub badge_url: Option<String>,

    #[serde(rename = "deleted_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

}
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
#[serde(untagged)]
pub enum WorkflowId { 
    Int(i32),
    Str(String),
}

impl Display for WorkflowId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self { 
            WorkflowId::Int(value) => write!(f, "{}", value),
            WorkflowId::Str(value) => write!(f, "{}", value),
        }
    }
}
/// An invocation of a workflow
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRun {
    /// The ID of the workflow run.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,

    /// The name of the workflow run.
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "node_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,

    #[serde(rename = "head_branch")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_branch: Option<String>,

    /// The SHA of the head commit that points to the version of the worflow being run.
    #[serde(rename = "head_sha")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_sha: Option<String>,

    /// The auto incrementing run number for the workflow run.
    #[serde(rename = "run_number")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_number: Option<i64>,

    #[serde(rename = "event")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "conclusion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,

    /// The ID of the parent workflow.
    #[serde(rename = "workflow_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow_id: Option<i64>,

    /// The URL to the workflow run.
    #[serde(rename = "url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "html_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,

    #[serde(rename = "pull_requests")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<Vec<PullRequestMinimal>>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "updated_at")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The URL to the jobs for the workflow run.
    #[serde(rename = "jobs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs_url: Option<String>,

    /// The URL to download the logs for the workflow run.
    #[serde(rename = "logs_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub logs_url: Option<String>,

    /// The URL to the associated check suite.
    #[serde(rename = "check_suite_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub check_suite_url: Option<String>,

    /// The URL to the artifacts for the workflow run.
    #[serde(rename = "artifacts_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub artifacts_url: Option<String>,

    /// The URL to cancel the workflow run.
    #[serde(rename = "cancel_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cancel_url: Option<String>,

    /// The URL to rerun the workflow run.
    #[serde(rename = "rerun_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub rerun_url: Option<String>,

    /// The URL to the workflow.
    #[serde(rename = "workflow_url")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflow_url: Option<String>,

    #[serde(rename = "head_commit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_commit: Option<SimpleCommit>,

    #[serde(rename = "repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,

    #[serde(rename = "head_repository")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_repository: Option<MinimalRepository>,

    #[serde(rename = "head_repository_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_repository_id: Option<i64>,

}
/// Workflow Run Usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunUsage {
    #[serde(rename = "billable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billable: Option<WorkflowrunusageBillable>,

    #[serde(rename = "run_duration_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub run_duration_ms: Option<i64>,

}
/// Workflow Usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowUsage {
    #[serde(rename = "billable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub billable: Option<WorkflowusageBillable>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowrunusageBillable {
    #[serde(rename = "UBUNTU")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ubuntu: Option<WorkflowrunusageBillableUbuntu>,

    #[serde(rename = "MACOS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub macos: Option<WorkflowrunusageBillableUbuntu>,

    #[serde(rename = "WINDOWS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub windows: Option<WorkflowrunusageBillableUbuntu>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowrunusageBillableUbuntu {
    #[serde(rename = "total_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_ms: Option<i64>,

    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub jobs: Option<i64>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowusageBillable {
    #[serde(rename = "UBUNTU")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ubuntu: Option<WorkflowusageBillableUbuntu>,

    #[serde(rename = "MACOS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub macos: Option<WorkflowusageBillableUbuntu>,

    #[serde(rename = "WINDOWS")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub windows: Option<WorkflowusageBillableUbuntu>,

}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowusageBillableUbuntu {
    #[serde(rename = "total_ms")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_ms: Option<i64>,

}
