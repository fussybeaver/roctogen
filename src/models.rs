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
    pub total_minutes_used: i64,
    /// The total paid GitHub Actions minutes used.
    pub total_paid_minutes_used: i64,
    /// The amount of free GitHub Actions minutes available.
    pub included_minutes: i64,
    pub minutes_used_breakdown: ActionsbillingusageMinutesUsedBreakdown,
}

/// Whether GitHub Actions is enabled on the repository.
pub type ActionsEnabled = bool;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsEnterprisePermissions {
    pub enabled_organizations: EnabledOrganizations,
    /// The API URL to use to get or set the selected organizations that are allowed to run GitHub Actions, when `enabled_organizations` is set to `selected`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organizations_url: Option<String>,
    pub allowed_actions: AllowedActions,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsOrganizationPermissions {
    pub enabled_repositories: EnabledRepositories,
    /// The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,
    pub allowed_actions: AllowedActions,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,
}

/// The public key used for setting Actions Secrets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsPublicKey {
    /// The identifier for the key.
    pub key_id: String,
    /// The Base64 encoded public key.
    pub key: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsRepositoryPermissions {
    pub enabled: ActionsEnabled,
    pub allowed_actions: AllowedActions,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_actions_url: Option<SelectedActionsUrl>,
}

/// Set secrets for GitHub Actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsSecret {
    /// The name of the secret.
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub id: i64,
    pub login: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_login: Option<String>,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
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
    pub verifiable_password_authentication: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_key_fingerprints: Option<ApioverviewSshKeyFingerprints>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub web: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub api: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub importer: Option<Vec<String>>,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<String>,
    /// The level of permission to grant the access token for repository creation, deletion, settings, teams, and collaborators creation. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub administration: Option<String>,
    /// The level of permission to grant the access token for checks on code. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,
    /// The level of permission to grant the access token for notification of content references and creation content attachments. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_references: Option<String>,
    /// The level of permission to grant the access token for repository contents, commits, branches, downloads, releases, and merges. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,
    /// The level of permission to grant the access token for deployments and deployment statuses. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<String>,
    /// The level of permission to grant the access token for managing repository environments. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub environments: Option<String>,
    /// The level of permission to grant the access token for issues and related comments, assignees, labels, and milestones. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,
    /// The level of permission to grant the access token to search repositories, list collaborators, and access repository metadata. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,
    /// The level of permission to grant the access token for packages published to GitHub Packages. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub packages: Option<String>,
    /// The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<String>,
    /// The level of permission to grant the access token for pull requests and related comments, assignees, labels, milestones, and merges. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<String>,
    /// The level of permission to grant the access token to manage the post-receive hooks for a repository. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_hooks: Option<String>,
    /// The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_projects: Option<String>,
    /// The level of permission to grant the access token to view and manage secret scanning alerts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_scanning_alerts: Option<String>,
    /// The level of permission to grant the access token to manage repository secrets. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub secrets: Option<String>,
    /// The level of permission to grant the access token to view and manage security events like code scanning alerts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_events: Option<String>,
    /// The level of permission to grant the access token to manage just a single file. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,
    /// The level of permission to grant the access token for commit statuses. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<String>,
    /// The level of permission to grant the access token to retrieve Dependabot alerts. Can be one of: `read`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vulnerability_alerts: Option<String>,
    /// The level of permission to grant the access token to update GitHub Actions workflow files. Can be one of: `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub workflows: Option<String>,
    /// The level of permission to grant the access token for organization teams and members. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<String>,
    /// The level of permission to grant the access token to manage access to an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_administration: Option<String>,
    /// The level of permission to grant the access token to manage the post-receive hooks for an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_hooks: Option<String>,
    /// The level of permission to grant the access token for viewing an organization's plan. Can be one of: `read`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_plan: Option<String>,
    /// The level of permission to grant the access token to manage organization projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_projects: Option<String>,
    /// The level of permission to grant the access token to manage organization secrets. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_secrets: Option<String>,
    /// The level of permission to grant the access token to view and manage GitHub Actions self-hosted runners available to an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_self_hosted_runners: Option<String>,
    /// The level of permission to grant the access token to view and manage users blocked by the organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_user_blocking: Option<String>,
    /// The level of permission to grant the access token to manage team discussions and related comments. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_discussions: Option<String>,
}

/// The authorization associated with an OAuth Access.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGrant {
    pub id: i64,
    pub url: String,
    pub app: ApplicationgrantApp,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub scopes: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationgrantApp {
    pub client_id: String,
    pub name: String,
    pub url: String,
}

/// An artifact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    pub id: i64,
    pub node_id: String,
    /// The name of the artifact.
    pub name: String,
    /// The size in bytes of the artifact.
    pub size_in_bytes: i64,
    pub url: String,
    pub archive_download_url: String,
    /// Whether or not the artifact has expired.
    pub expired: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditLogEvent {
    /// The time the audit log event occurred, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
    #[serde(rename = "@timestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub timestamp: Option<i64>,
    /// The name of the action that was performed, for example `user.login` or `repo.create`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_was: Option<bool>,
    /// The actor who performed the action.
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<String>,
    /// The username of the account being blocked.
    #[serde(skip_serializing_if="Option::is_none")]
    pub blocked_user: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub business: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub config_was: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,
    /// The time the audit log event was recorded, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deploy_key_fingerprint: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_were: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hook_id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub limited_availability: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub old_user: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub openssh_public_key: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub org: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_visibility: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,
    /// The name of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo: Option<String>,
    /// The name of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_public: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_login: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub team: Option<String>,
    /// The type of protocol (for example, HTTP or SSH) used to transfer Git data.
    #[serde(skip_serializing_if="Option::is_none")]
    pub transport_protocol: Option<i64>,
    /// A human readable name for the protocol (for example, HTTP or SSH) used to transfer Git data.
    #[serde(skip_serializing_if="Option::is_none")]
    pub transport_protocol_name: Option<String>,
    /// The user that was affected by the action performed (if available).
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
    /// The repository visibility, for example `public` or `private`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
}

/// Authentication Token
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationToken {
    /// The token used for authentication
    pub token: String,
    /// The time this token expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<HashMap<(), ()>>,
    /// The repositories this token has access to
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,
    /// Describe whether all repositories have been selected or there's a selection involved
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
    pub id: i64,
    pub url: String,
    /// A list of scopes that this authorization is in.
    pub scopes: Vec<String>,
    pub token: String,
    pub token_last_eight: String,
    pub hashed_token: String,
    pub app: ApplicationgrantApp,
    pub note: String,
    pub note_url: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fingerprint: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub installation: Option<ScopedInstallation>,
}

/// The status of auto merging a pull request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoMerge {
    pub enabled_by: SimpleUser,
    /// The merge method to use.
    pub merge_method: String,
    /// Title for the merge commit message.
    pub commit_title: String,
    /// Commit message for the merge commit.
    pub commit_message: String,
}

/// Base Gist
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseGist {
    pub url: String,
    pub forks_url: String,
    pub commits_url: String,
    pub id: String,
    pub node_id: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub html_url: String,
    pub files: HashMap<String, BasegistFiles>,
    pub public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub comments: i64,
    pub user: SimpleUser,
    pub comments_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub history: Option<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasegistFiles {
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
}

/// Basic Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicError {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

/// Blob
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blob {
    pub content: String,
    pub encoding: String,
    pub url: String,
    pub sha: String,
    pub size: i64,
    pub node_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub highlighted_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchAppsUpdateWebhookConfigForApp {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateInstallationAccessToken {
    /// List of repository names that the token should have access to
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,
    /// List of repository IDs that the token should have access to
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<AppPermissions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOauthAuthorizationsUpdateAuthorization {
    /// A list of scopes that this authorization is in.
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// A list of scopes to add to this authorization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_scopes: Option<Vec<String>>,
    /// A list of scopes to remove from this authorization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_scopes: Option<Vec<String>>,
    /// A note to remind you what the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    /// A URL to remind you what app the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,
    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateBlob {
    /// The new blob's content.
    pub content: String,
    /// The encoding used for `content`. Currently, `\"utf-8\"` and `\"base64\"` are supported.
    #[serde(skip_serializing_if="Option::is_none")]
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateCommit {
    /// The commit message
    pub message: String,
    /// The SHA of the tree object this commit points to
    pub tree: String,
    /// The SHAs of the commits that were the parents of this commit. If omitted or empty, the commit will be written as a root commit. For a single parent, an array of one SHA should be provided; for a merge commit, an array of more than one should be provided.
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepogitcommitsAuthor>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepogitcommitsCommitter>,
    /// The [PGP signature](https://en.wikipedia.org/wiki/Pretty_Good_Privacy) of the commit. GitHub adds the signature to the `gpgsig` header of the created commit. For a commit signature to be verifiable by Git or GitHub, it must be an ASCII-armored detached PGP signature over the string commit as it would be written to the object database. To pass a `signature` parameter, you need to first manually create a valid PGP signature, which can be complicated. You may find it easier to [use the command line](https://git-scm.com/book/id/v2/Git-Tools-Signing-Your-Work) to create signed commits.
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateRef {
    /// The name of the fully qualified reference (ie: `refs/heads/master`). If it doesn't start with 'refs' and have at least two slashes, it will be rejected.
    #[serde(rename = "ref")]
    pub _ref: String,
    /// The SHA1 value for this reference.
    pub sha: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGitUpdateRef {
    /// The SHA1 value to set this reference to
    pub sha: String,
    /// Indicates whether to force the update or to make sure the update is a fast-forward update. Leaving this out or setting it to `false` will make sure you're not overwriting work.
    #[serde(skip_serializing_if="Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateTag {
    /// The tag's name. This is typically a version (e.g., \"v0.0.1\").
    pub tag: String,
    /// The tag message.
    pub message: String,
    /// The SHA of the git object this is tagging.
    pub object: String,
    /// The type of the object we're tagging. Normally this is a `commit` but it can also be a `tree` or a `blob`.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tagger: Option<ReposownerrepogittagsTagger>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGitCreateTree {
    /// Objects (of `path`, `mode`, `type`, and `sha`) specifying a tree structure.
    pub tree: Vec<ReposownerrepogittreesTree>,
    /// The SHA1 of an existing Git tree object which will be used as the base for the new tree. If provided, a new Git tree object will be created from entries in the Git tree object pointed to by `base_tree` and entries defined in the `tree` parameter. Entries defined in the `tree` parameter will overwrite items from `base_tree` with the same `path`. If you're creating new changes on a branch, then normally you'd set `base_tree` to the SHA1 of the Git tree object of the current latest commit on the branch you're working on. If not provided, GitHub will create a new Git tree object from only the entries defined in the `tree` parameter. If you create a new commit pointing to such a tree, then all files which were a part of the parent commit's tree and were not defined in the `tree` parameter will be listed as deleted by the new commit. 
    #[serde(skip_serializing_if="Option::is_none")]
    pub base_tree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateWebhook {
    /// Use `web` to create a webhook. Default: `web`. This parameter only accepts the value `web`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    pub config: ReposownerrepohooksConfig,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateWebhook {
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<ReposownerrepohookshookIdConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for. This replaces the entire array of events.
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines a list of events to be added to the list of events that the Hook triggers for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub add_events: Option<Vec<String>>,
    /// Determines a list of events to be removed from the list of events that the Hook triggers for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub remove_events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateWebhookConfigForRepo {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMigrationsStartImport {
    /// The URL of the originating repository.
    pub vcs_url: String,
    /// The originating VCS type. Can be one of `subversion`, `git`, `mercurial`, or `tfvc`. Please be aware that without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,
    /// If authentication is required, the username to provide to `vcs_url`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_username: Option<String>,
    /// If authentication is required, the password to provide to `vcs_url`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_password: Option<String>,
    /// For a tfvc import, the name of the project that is being imported.
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateContentAttachment {
    /// The title of the attachment
    pub title: String,
    /// The body of the attachment
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsUpdateImport {
    /// The username to provide to the originating repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_username: Option<String>,
    /// The password to provide to the originating repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs_password: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsMapCommitAuthor {
    /// The new Git author email.
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// The new Git author name.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMigrationsSetLfsPreference {
    /// Can be one of `opt_in` (large files will be stored using Git LFS) or `opt_out` (large files will be removed during the import).
    pub use_lfs: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateInvitation {
    /// The permissions that the associated user will have on the repository. Valid values are `read`, `write`, `maintain`, `triage`, and `admin`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreate {
    /// The title of the issue.
    pub title: Value,
    /// The contents of the issue.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is deprecated.**_
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Value>,
    /// Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Value>>,
    /// Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateComment {
    /// The contents of the comment.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForIssueComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue comment.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdate {
    /// The title of the issue.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<Value>,
    /// The contents of the issue.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// Login for the user that this issue should be assigned to. **This field is deprecated.**
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<String>,
    /// State of the issue. Either `open` or `closed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<Value>,
    /// Labels to associate with this issue. Pass one or more Labels to _replace_ the set of Labels on this Issue. Send an empty array (`[]`) to clear all Labels from the Issue. _NOTE: Only users with push access can set labels for issues. Labels are silently dropped otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<Value>>,
    /// Logins for Users to assign to this issue. Pass one or more user logins to _replace_ the set of assignees on this Issue. Send an empty array (`[]`) to clear all assignees from the Issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesAddAssignees {
    /// Usernames of people to assign this issue to. _NOTE: Only users with push access can add assignees to an issue. Assignees are silently ignored otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteIssuesRemoveAssignees {
    /// Usernames of assignees to remove from an issue. _NOTE: Only users with push access can remove assignees from an issue. Assignees are silently ignored otherwise._
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetGithubActionsPermissionsEnterprise {
    pub enabled_organizations: EnabledOrganizations,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateComment {
    /// The contents of the comment.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutIssuesSetLabels {
    /// The names of the labels to add to the issue. You can pass an empty array to remove all labels. **Note:** Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key.
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesAddLabels {
    /// The name of the label to add to the issue. Must contain at least one label. **Note:** Alternatively, you can pass a single label as a `string` or an `array` of labels directly, but GitHub recommends passing an object with the `labels` key.
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutIssuesLock {
    /// The reason for locking the issue or pull request conversation. Lock will fail if you don't use one of these reasons:   \\* `off-topic`   \\* `too heated`   \\* `resolved`   \\* `spam`
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForIssue {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the issue.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeployKey {
    /// A name for the key.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The contents of the key.
    pub key: String,
    /// If `true`, the key will only be able to read repository contents. Otherwise, the key will be able to read and write.      Deploy keys with write access can perform the same actions as an organization member with admin access, or a collaborator on a personal repository. For more information, see \"[Repository permission levels for an organization](https://help.github.com/articles/repository-permission-levels-for-an-organization/)\" and \"[Permission levels for a user account repository](https://help.github.com/articles/permission-levels-for-a-user-account-repository/).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateLabel {
    /// The name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png \":strawberry:\"). For a full list of available emoji and codes, see [emoji-cheat-sheet.com](http://emoji-cheat-sheet.com/).
    pub name: String,
    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    /// A short description of the label.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateLabel {
    /// The new name of the label. Emoji can be added to label names, using either native emoji or colon-style markup. For example, typing `:strawberry:` will render the emoji ![:strawberry:](https://github.githubassets.com/images/icons/emoji/unicode/1f353.png \":strawberry:\"). For a full list of available emoji and codes, see [emoji-cheat-sheet.com](http://emoji-cheat-sheet.com/).
    #[serde(skip_serializing_if="Option::is_none")]
    pub new_name: Option<String>,
    /// The [hexadecimal color code](http://www.color-hex.com/) for the label, without the leading `#`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    /// A short description of the label.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposMerge {
    /// The name of the base branch that the head will be merged into.
    pub base: String,
    /// The head to merge. This can be a branch name or a commit SHA1.
    pub head: String,
    /// Commit message to use for the merge commit. If omitted, a default message will be used.
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostIssuesCreateMilestone {
    /// The title of the milestone.
    pub title: String,
    /// The state of the milestone. Either `open` or `closed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    /// A description of the milestone.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetSelectedOrganizationsEnabledGithubActionsEnterprise {
    /// List of organization IDs to enable for GitHub Actions.
    pub selected_organization_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchIssuesUpdateMilestone {
    /// The title of the milestone.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The state of the milestone. Either `open` or `closed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    /// A description of the milestone.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// The milestone due date. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivityMarkRepoNotificationsAsRead {
    /// Describes the last point that notifications were checked. Anything updated since this time will not be marked as read. If you omit this parameter, all notifications are marked as read. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Default: The current timestamp.
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_read_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposUpdateInformationAboutPagesSite {
    /// Specify a custom domain for the repository. Sending a `null` value will remove the custom domain. For more about custom domains, see \"[Using a custom domain with GitHub Pages](https://help.github.com/articles/using-a-custom-domain-with-github-pages/).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub cname: Option<String>,
    /// Configures access controls for the GitHub Pages site. If public is set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site. This includes anyone in your Enterprise if the repository is set to `internal` visibility. This feature is only available to repositories in an organization on an Enterprise plan.
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,
    pub source: Value,
}

/// The source branch and directory used to publish your Pages site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreatePagesSite {
    pub source: ReposownerrepopagesSource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForRepo {
    /// The name of the project.
    pub name: String,
    /// The description of the project.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreate {
    /// The title of the new pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace `head` with a user like this: `username:branch`.
    pub head: String,
    /// The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository.
    pub base: String,
    /// The contents of the pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// Indicates whether [maintainers can modify](https://help.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    /// Indicates whether the pull request is a draft. See \"[Draft Pull Requests](https://help.github.com/en/articles/about-pull-requests#draft-pull-requests)\" in the GitHub Help documentation to learn more.
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchPullsUpdateReviewComment {
    /// The text of the reply to the review comment.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForPullRequestReviewComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the pull request review comment.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchPullsUpdate {
    /// The title of the pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The contents of the pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// State of this Pull Request. Either `open` or `closed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    /// The name of the branch you want your changes pulled into. This should be an existing branch on the current repository. You cannot update the base branch on a pull request to point to another repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub base: Option<String>,
    /// Indicates whether [maintainers can modify](https://help.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request.
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReviewComment {
    /// The text of the review comment.
    pub body: String,
    /// The SHA of the commit needing a comment. Not using the latest commit SHA may render your comment outdated if a subsequent commit modifies the line you specify as the `position`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,
    /// The relative path to the file that necessitates a comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    /// **Required without `comfort-fade` preview**. The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. For help finding the position value, read the note above.
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,
    /// **Required with `comfort-fade` preview**. In a split diff view, the side of the diff that the pull request's changes appear on. Can be `LEFT` or `RIGHT`. Use `LEFT` for deletions that appear in red. Use `RIGHT` for additions that appear in green or unchanged lines that appear in white and are shown for context. For a multi-line comment, side represents whether the last line of the comment range is a deletion or addition. For more information, see \"[Diff view options](https://help.github.com/en/articles/about-comparing-branches-in-pull-requests#diff-view-options)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,
    /// **Required with `comfort-fade` preview**. The line of the blob in the pull request diff that the comment applies to. For a multi-line comment, the last line of the range that your comment applies to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,
    /// **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_line` is the first line in the pull request diff that your multi-line comment applies to. To learn more about multi-line comments, see \"[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,
    /// **Required when using multi-line comments**. To create multi-line comments, you must use the `comfort-fade` preview header. The `start_side` is the starting side of the diff that the comment applies to. Can be `LEFT` or `RIGHT`. To learn more about multi-line comments, see \"[Commenting on a pull request](https://help.github.com/en/articles/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request)\" in the GitHub Help documentation. See `side` in this table for additional context.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminCreateSelfHostedRunnerGroupForEnterprise {
    /// Name of the runner group.
    pub name: String,
    /// Visibility of a runner group. You can select all organizations or select individual organization. Can be one of: `all` or `selected`
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    /// List of organization IDs that can access the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organization_ids: Option<Vec<i32>>,
    /// List of runner IDs to add to the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReplyForReviewComment {
    /// The text of the review comment.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsMerge {
    /// Title for the automatic commit message.
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_title: Option<String>,
    /// Extra detail to append to automatic commit message.
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_message: Option<String>,
    /// SHA that pull request head must match to allow merge.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    /// Merge method to use. Possible values are `merge`, `squash` or `rebase`. Default is `merge`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub merge_method: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsRequestReviewers {
    /// An array of user `login`s that will be requested.
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<String>>,
    /// An array of team `slug`s that will be requested.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_reviewers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletePullsRemoveRequestedReviewers {
    /// An array of user `login`s that will be removed.
    pub reviewers: Vec<String>,
    /// An array of team `slug`s that will be removed.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_reviewers: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsCreateReview {
    /// The SHA of the commit that needs a review. Not using the latest commit SHA may render your review comment outdated if a subsequent commit modifies the line you specify as the `position`. Defaults to the most recent commit in the pull request when you do not specify a value.
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,
    /// **Required** when using `REQUEST_CHANGES` or `COMMENT` for the `event` parameter. The body text of the pull request review.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. By leaving this blank, you set the review action state to `PENDING`, which means you will need to [submit the pull request review](https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request) when you are ready.
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,
    /// Use the following table to specify the location, destination, and contents of the draft review comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<Vec<ReposownerrepopullspullNumberreviewsComments>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateReview {
    /// The body text of the pull request review.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsDismissReview {
    /// The message for the pull request review dismissal
    pub message: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostPullsSubmitReview {
    /// The body text of the pull request review
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// The review action you want to perform. The review actions include: `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`. When you leave this blank, the API returns _HTTP 422 (Unrecognizable entity)_ and sets the review action state to `PENDING`, which means you will need to re-submit the pull request review using a review action.
    pub event: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateBranch {
    /// The expected SHA of the pull request's HEAD ref. This is the most recent commit on the pull request's branch. If the expected SHA does not match the pull request's HEAD, you will receive a `422 Unprocessable Entity` status. You can use the \"[List commits](https://docs.github.com/rest/reference/repos#list-commits)\" endpoint to find the most recent commit SHA. Default: SHA of the pull request's current HEAD ref.
    #[serde(skip_serializing_if="Option::is_none")]
    pub expected_head_sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateRelease {
    /// The name of the tag.
    pub tag_name: String,
    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch (usually `master`).
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,
    /// The name of the release.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// Text describing the contents of the tag.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// `true` to create a draft (unpublished) release, `false` to create a published one.
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    /// `true` to identify the release as a prerelease. `false` to identify the release as a full release.
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateSelfHostedRunnerGroupForEnterprise {
    /// Name of the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// Visibility of a runner group. You can select all organizations or select individual organizations. Can be one of: `all` or `selected`
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateReleaseAsset {
    /// The file name of the asset.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// An alternate short description of the asset. Used in place of the filename.
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateRelease {
    /// The name of the tag.
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag_name: Option<String>,
    /// Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch (usually `master`).
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_commitish: Option<String>,
    /// The name of the release.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// Text describing the contents of the tag.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// `true` makes the release a draft, and `false` publishes the release.
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    /// `true` to identify the release as a prerelease, `false` to identify the release as a full release.
    #[serde(skip_serializing_if="Option::is_none")]
    pub prerelease: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchSecretScanningUpdateAlert {
    pub state: SecretScanningAlertState,
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateCommitStatus {
    /// The state of the status. Can be one of `error`, `failure`, `pending`, or `success`.
    pub state: String,
    /// The target URL to associate with this status. This URL will be linked from the GitHub UI to allow users to easily see the source of the status.   For example, if your continuous integration system is posting build status, you would want to provide the deep link for the build output for this specific SHA:   `http://ci.example.com/user/repo/build/sha`
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,
    /// A short description of the status.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// A string label to differentiate this status from the status of other systems. This field is case-insensitive.
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivitySetRepoSubscription {
    /// Determines if notifications should be received from this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribed: Option<bool>,
    /// Determines if all notifications should be blocked from this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposReplaceAllTopics {
    /// An array of topics to add to the repository. Pass one or more topics to _replace_ the set of existing topics. Send an empty array (`[]`) to clear all topics from the repository. **Note:** Topic `names` cannot contain uppercase letters.
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposTransfer {
    /// The username or organization name the repository will be transferred to.
    pub new_owner: String,
    /// ID of the team or teams to add to the repository. Teams can only be added to organization-owned repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateUsingTemplate {
    /// The organization or person who will own the new repository. To create a new repository in an organization, the authenticated user must be a member of the specified organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<String>,
    /// The name of the new repository.
    pub name: String,
    /// A short description of the new repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Set to `true` to include the directory structure and files from all branches in the template repository, and not just the default branch. Default: `false`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub include_all_branches: Option<bool>,
    /// Either `true` to create a new private repository or `false` to create a new public one.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateEnvironmentSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an environment public key](https://docs.github.com/rest/reference/actions#get-an-environment-public-key) endpoint.
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminProvisionAndInviteEnterpriseGroup {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// The name of the SCIM group. This must match the GitHub organization that the group maps to.
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<Scimv2enterprisesenterpriseGroupsMembers>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetOrgAccessToSelfHostedRunnerGroupInEnterprise {
    /// List of organization IDs that can access the runner group.
    pub selected_organization_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetInformationForProvisionedEnterpriseGroup {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// The name of the SCIM group. This must match the GitHub organization that the group maps to.
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<Scimv2enterprisesenterpriseGroupsMembers>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateAttributeForEnterpriseGroup {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    #[serde(rename = "Operations")]
    pub operations: Vec<Scimv2enterprisesenterpriseGroupsscimGroupIdOperations>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnterpriseAdminProvisionAndInviteEnterpriseUser {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// The username for the user.
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: Scimv2enterprisesenterpriseUsersName,
    /// List of user emails.
    pub emails: Vec<Scimv2enterprisesenterpriseUsersEmails>,
    /// List of SCIM group IDs the user is a member of.
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetInformationForProvisionedEnterpriseUser {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// The username for the user.
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: Scimv2enterprisesenterpriseUsersName,
    /// List of user emails.
    pub emails: Vec<Scimv2enterprisesenterpriseUsersEmails>,
    /// List of SCIM group IDs the user is a member of.
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchEnterpriseAdminUpdateAttributeForEnterpriseUser {
    /// The SCIM schema URIs.
    pub schemas: Vec<String>,
    /// Array of [SCIM operations](https://tools.ietf.org/html/rfc7644#section-3.5.2).
    #[serde(rename = "Operations")]
    pub operations: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimProvisionAndInviteUser {
    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    pub user_name: String,
    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    pub name: Scimv2organizationsorgUsersName,
    /// user emails
    pub emails: Vec<Scimv2organizationsorgUsersEmails>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimSetInformationForProvisionedUser {
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: Scimv2organizationsorgUsersName,
    /// user emails
    pub emails: Vec<Scimv2organizationsorgUsersscimUserIdEmails>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimUpdateAttributeForUser {
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// Set of operations to be performed
    #[serde(rename = "Operations")]
    pub operations: Vec<Scimv2organizationsorgUsersscimUserIdOperations>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateLegacy {
    /// The name of the team.
    pub name: String,
    /// The description of the team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
    /// The ID of a team to set as the parent team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionLegacy {
    /// The discussion post's title.
    pub title: String,
    /// The discussion post's body text.
    pub body: String,
    /// Private posts are only visible to team members, organization owners, and team maintainers. Public posts are visible to all members of the organization. Set to `true` to create a private post.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnterpriseAdminSetSelfHostedRunnersInGroupForEnterprise {
    /// List of runner IDs to add to the runner group.
    pub runners: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionLegacy {
    /// The discussion post's title.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The discussion post's body text.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionCommentLegacy {
    /// The discussion comment's body text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionCommentLegacy {
    /// The discussion comment's body text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionCommentLegacy {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionLegacy {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserLegacy {
    /// The role that this user should have in the team. Can be one of:   \\* `member` - a normal member of the team.   \\* `maintainer` - a team maintainer. Able to add/remove other team members, promote other team members to team maintainer, and edit the team's name and description.
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsLegacy {
    /// The permission to grant to the team for this project. Can be one of:   \\* `read` - team members can read, but not write to or administer this project.   \\* `write` - team members can read and write, but not administer this project.   \\* `admin` - team members can read, write and administer this project.   Default: the team's `permission` attribute will be used to determine what permission to grant the team on this project. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateRepoPermissionsLegacy {
    /// The permission to grant the team on this repository. Can be one of:   \\* `pull` - team members can pull, but not push to or administer this repository.   \\* `push` - team members can pull and push, but not administer this repository.   \\* `admin` - team members can pull, push and administer this repository.      If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsCreateOrUpdateIdpGroupConnectionsLegacy {
    /// The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
    pub groups: Vec<TeamsteamIdteamsyncgroupmappingsGroups>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub synced_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchUsersUpdateAuthenticated {
    /// The new name of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The publicly visible email address of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// The new blog URL of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,
    /// The new Twitter username of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,
    /// The new company of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,
    /// The new location of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    /// The new hiring availability of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,
    /// The new short biography of the user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGistsCreate {
    /// Description of the gist
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Names and content for the files that make up the gist
    pub files: HashMap<String, GistsFiles>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchUsersSetPrimaryEmailVisibilityForAuthenticated {
    /// An email address associated with the GitHub user account to manage.
    pub email: String,
    /// Denotes whether an email is publically visible.
    pub visibility: String,
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
    pub armored_public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostUsersCreatePublicSshKeyForAuthenticated {
    /// A descriptive name for the new key.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The public SSH key to add to your GitHub account.
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateMembershipForAuthenticatedUser {
    /// The state that the membership should be in. Only `\"active\"` will be accepted.
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMigrationsStartForAuthenticatedUser {
    /// Lock the repositories being migrated at the start of the migration
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_repositories: Option<bool>,
    /// Do not include attachments in the migration
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_attachments: Option<bool>,
    /// Exclude attributes from the API response to improve performance
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<String>>,
    pub repositories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForAuthenticatedUser {
    /// Name of the project
    pub name: String,
    /// Body of the project
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateForAuthenticatedUser {
    /// The name of the repository.
    pub name: String,
    /// A short description of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    /// Whether the repository is private.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    /// Whether issues are enabled.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    /// Whether projects are enabled.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    /// Whether the wiki is enabled.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_id: Option<i64>,
    /// Whether the repository is initialized with a minimal README.
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_init: Option<bool>,
    /// The desired language or platform to apply to the .gitignore.
    #[serde(skip_serializing_if="Option::is_none")]
    pub gitignore_template: Option<String>,
    /// The license keyword of the open source license for this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub license_template: Option<String>,
    /// Whether to allow squash merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Whether downloads are enabled.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGistsUpdate {
    /// Description of the gist
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Names of files to be updated
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, GistsgistIdFiles>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteAppsDeleteAuthorization {
    /// The OAuth access token used to authenticate to the GitHub API.
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostGistsCreateComment {
    /// The comment text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchGistsUpdateComment {
    /// The comment text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMarkdownRender {
    /// The Markdown text to render in HTML.
    pub text: String,
    /// The rendering mode.
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,
    /// The repository context to use when creating references in `gfm` mode.
    #[serde(skip_serializing_if="Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivityMarkNotificationsAsRead {
    /// Describes the last point that notifications were checked.
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether the notification has been read.
    #[serde(skip_serializing_if="Option::is_none")]
    pub read: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActivitySetThreadSubscription {
    /// Whether to block all notifications from a thread.
    #[serde(skip_serializing_if="Option::is_none")]
    pub ignored: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdate {
    /// Billing email address. This address is not publicized.
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_email: Option<String>,
    /// The company name.
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,
    /// The publicly visible email address.
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// The Twitter username of the company.
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,
    /// The location.
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    /// The shorthand name of the company.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The description of the company.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Toggles whether an organization can use organization projects.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_organization_projects: Option<bool>,
    /// Toggles whether repositories that belong to the organization can use repository projects.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_repository_projects: Option<bool>,
    /// Default permission level members have for organization repositories:   \\* `read` - can pull, but not push to or administer this repository.   \\* `write` - can pull and push, but not administer this repository.   \\* `admin` - can pull, push, and administer this repository.   \\* `none` - no permissions granted by default.
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_repository_permission: Option<String>,
    /// Toggles the ability of non-admin organization members to create repositories. Can be one of:   \\* `true` - all organization members can create repositories.   \\* `false` - only organization owners can create repositories.   Default: `true`   **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details. **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    /// Toggles whether organization members can create internal repositories, which are visible to all enterprise members. You can only allow members to create internal repositories if your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. Can be one of:   \\* `true` - all organization members can create internal repositories.   \\* `false` - only organization owners can create internal repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    /// Toggles whether organization members can create private repositories, which are visible to organization members with permission. Can be one of:   \\* `true` - all organization members can create private repositories.   \\* `false` - only organization owners can create private repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    /// Toggles whether organization members can create public repositories, which are visible to anyone. Can be one of:   \\* `true` - all organization members can create public repositories.   \\* `false` - only organization owners can create public repositories.   Default: `true`. For more information, see \"[Restricting repository creation in your organization](https://help.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    /// Specifies which types of repositories non-admin organization members can create. Can be one of:   \\* `all` - all organization members can create public and private repositories.   \\* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.   \\* `none` - only admin members can create repositories.   **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    /// Toggles whether organization members can create GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create GitHub Pages sites.   \\* `false` - no organization members can create GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    /// Toggles whether organization members can create public GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create public GitHub Pages sites.   \\* `false` - no organization members can create public GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_pages: Option<bool>,
    /// Toggles whether organization members can create private GitHub Pages sites. Can be one of:   \\* `true` - all organization members can create private GitHub Pages sites.   \\* `false` - no organization members can create private GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_pages: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetGithubActionsPermissionsOrganization {
    pub enabled_repositories: EnabledRepositories,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelectedRepositoriesEnabledGithubActionsOrganization {
    /// List of repository IDs to enable for GitHub Actions.
    pub selected_repository_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsCreateSelfHostedRunnerGroupForOrg {
    /// Name of the runner group.
    pub name: String,
    /// Visibility of a runner group. You can select all repositories, select individual repositories, or limit access to private repositories. Can be one of: `all`, `selected`, or `private`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    /// List of repository IDs that can access the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,
    /// List of runner IDs to add to the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchActionsUpdateSelfHostedRunnerGroupForOrg {
    /// Name of the runner group.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// Visibility of a runner group. You can select all repositories, select individual repositories, or all private repositories. Can be one of: `all`, `selected`, or `private`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCheckToken {
    /// The access_token of the OAuth application.
    pub access_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetRepoAccessToSelfHostedRunnerGroupInOrg {
    /// List of repository IDs that can access the runner group.
    pub selected_repository_ids: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelfHostedRunnersInGroupForOrg {
    /// List of runner IDs to add to the runner group.
    pub runners: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateOrgSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an organization public key](https://docs.github.com/rest/reference/actions#get-an-organization-public-key) endpoint.
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,
    /// Configures the access that repositories have to the organization secret. Can be one of:   \\- `all` - All repositories in an organization can access the secret.   \\- `private` - Private repositories in an organization can access the secret.   \\- `selected` - Only specific repositories can access the secret.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can manage the list of selected repositories using the [List selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#list-selected-repositories-for-an-organization-secret), [Set selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret), and [Remove selected repository from an organization secret](https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetSelectedReposForOrgSecret {
    /// An array of repository ids that can access the organization secret. You can only provide a list of repository ids when the `visibility` is set to `selected`. You can add and remove individual repositories using the [Set selected repositories for an organization secret](https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret) and [Remove selected repository from an organization secret](https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret) endpoints.
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repository_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOrgsCreateWebhook {
    /// Must be passed as \"web\".
    pub name: String,
    pub config: OrgsorghooksConfig,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateWebhook {
    #[serde(skip_serializing_if="Option::is_none")]
    pub config: Option<OrgsorghookshookIdConfig>,
    /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchOrgsUpdateWebhookConfigForOrg {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOrgsCreateInvitation {
    /// **Required unless you provide `email`**. GitHub user ID for the person you are inviting.
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitee_id: Option<i64>,
    /// **Required unless you provide `invitee_id`**. Email address of the person you are inviting, which can be an existing GitHub user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// Specify role for new member. Can be one of:   \\* `admin` - Organization owners with full administrative rights to the organization and complete access to all repositories and teams.   \\* `direct_member` - Non-owner organization members with ability to see other members and join teams by invitation.   \\* `billing_manager` - Non-owner organization members with ability to manage the billing settings of your organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
    /// Specify IDs for the teams you want to invite new members to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOrgsSetMembershipForUser {
    /// The role to give the user in the organization. Can be one of:   \\* `admin` - The user will become an owner of the organization.   \\* `member` - The user will become a non-owner member of the organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMigrationsStartForOrg {
    /// A list of arrays indicating which repositories should be migrated.
    pub repositories: Vec<String>,
    /// Indicates whether repositories should be locked (to prevent manipulation) while migrating data.
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_repositories: Option<bool>,
    /// Indicates whether attachments should be excluded from the migration (to reduce migration archive file size).
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_attachments: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteAppsDeleteToken {
    /// The OAuth access token used to authenticate to the GitHub API.
    pub access_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateForOrg {
    /// The name of the project.
    pub name: String,
    /// The description of the project.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateInOrg {
    /// The name of the repository.
    pub name: String,
    /// A short description of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    /// Whether the repository is private.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    /// Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. Note: For GitHub Enterprise Server and GitHub AE, this endpoint will only list repositories available to all users on the enterprise. For more information, see \"[Creating an internal repository](https://help.github.com/en/github/creating-cloning-and-archiving-repositories/about-repository-visibility#about-internal-repositories)\" in the GitHub Help documentation.   The `visibility` parameter overrides the `private` parameter when you use both parameters with the `nebula-preview` preview header.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub team_id: Option<i64>,
    /// Pass `true` to create an initial commit with empty README.
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_init: Option<bool>,
    /// Desired language or platform [.gitignore template](https://github.com/github/gitignore) to apply. Use the name of the template without the extension. For example, \"Haskell\".
    #[serde(skip_serializing_if="Option::is_none")]
    pub gitignore_template: Option<String>,
    /// Choose an [open source license template](https://choosealicense.com/) that best suits your needs, and then use the [license keyword](https://help.github.com/articles/licensing-a-repository/#searching-github-by-license-type) as the `license_template` string. For example, \"mit\" or \"mpl-2.0\".
    #[serde(skip_serializing_if="Option::is_none")]
    pub license_template: Option<String>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreate {
    /// The name of the team.
    pub name: String,
    /// The description of the team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// List GitHub IDs for organization members who will become team maintainers.
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintainers: Option<Vec<String>>,
    /// The full name (e.g., \"organization-name/repository-name\") of repositories to add the team to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// The level of privacy this team should have. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   Default: `secret`   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.   Default for child team: `closed`
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
    /// The ID of a team to set as the parent team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateInOrg {
    /// The name of the team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The description of the team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// The level of privacy this team should have. Editing teams without specifying this parameter leaves `privacy` intact. When a team is nested, the `privacy` for parent teams cannot be `secret`. The options are:   **For a non-nested team:**   \\* `secret` - only visible to organization owners and members of this team.   \\* `closed` - visible to all members of this organization.   **For a parent or child team:**   \\* `closed` - visible to all members of this organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    /// **Deprecated**. The permission that new repositories will be added to the team with when none is specified. Can be one of:   \\* `pull` - team members can pull, but not push to or administer newly-added repositories.   \\* `push` - team members can pull and push, but not administer newly-added repositories.   \\* `admin` - team members can pull, push and administer newly-added repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
    /// The ID of a team to set as the parent team.
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent_team_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionInOrg {
    /// The discussion post's title.
    pub title: String,
    /// The discussion post's body text.
    pub body: String,
    /// Private posts are only visible to team members, organization owners, and team maintainers. Public posts are visible to all members of the organization. Set to `true` to create a private post.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionInOrg {
    /// The discussion post's title.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// The discussion post's body text.
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTeamsCreateDiscussionCommentInOrg {
    /// The discussion comment's body text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsUpdateDiscussionCommentInOrg {
    /// The discussion comment's body text.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionCommentInOrg {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion comment.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForTeamDiscussionInOrg {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the team discussion.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchAppsResetToken {
    /// The access_token of the OAuth application.
    pub access_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserInOrg {
    /// The role that this user should have in the team. Can be one of:   \\* `member` - a normal member of the team.   \\* `maintainer` - a team maintainer. Able to add/remove other team members, promote other team members to team maintainer, and edit the team's name and description.
    #[serde(skip_serializing_if="Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsInOrg {
    /// The permission to grant to the team for this project. Can be one of:   \\* `read` - team members can read, but not write to or administer this project.   \\* `write` - team members can read and write, but not administer this project.   \\* `admin` - team members can read, write and administer this project.   Default: the team's `permission` attribute will be used to determine what permission to grant the team on this project. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateRepoPermissionsInOrg {
    /// The permission to grant the team on this repository. Can be one of:   \\* `pull` - team members can pull, but not push to or administer this repository.   \\* `push` - team members can pull and push, but not administer this repository.   \\* `admin` - team members can pull, push and administer this repository.   \\* `maintain` - team members can manage the repository without access to sensitive or destructive actions. Recommended for project managers. Only applies to repositories owned by organizations.   \\* `triage` - team members can proactively manage issues and pull requests without write access. Recommended for contributors who triage a repository. Only applies to repositories owned by organizations.      If no permission is specified, the team's `permission` attribute will be used to determine what permission to grant the team on this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchTeamsCreateOrUpdateIdpGroupConnectionsInOrg {
    /// The IdP groups you want to connect to a GitHub team. When updating, the new `groups` object will replace the original one. You must include any existing groups that you don't want to remove.
    pub groups: Vec<OrgsorgteamsteamSlugteamsyncgroupmappingsGroups>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateCard {
    /// The project card's note
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    /// Whether or not the card is archived
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveCard {
    /// The position of the card in a column
    pub position: String,
    /// The unique identifier of the column the card should be moved to
    #[serde(skip_serializing_if="Option::is_none")]
    pub column_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateColumn {
    /// Name of the project column
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCard {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveColumn {
    /// The position of the column in a project
    pub position: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdate {
    /// Name of the project
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// Body of the project
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// State of the project; either 'open' or 'closed'
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    /// The baseline permission that all organization members have on this project
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,
    /// Whether or not this project can be seen by everyone.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsScopeToken {
    /// **Required.** The OAuth access token used to authenticate to the GitHub API.
    pub access_token: String,
    /// The name of the user or organization to scope the user-to-server access token to. **Required** unless `target_id` is specified.
    #[serde(skip_serializing_if="Option::is_none")]
    pub target: Option<String>,
    /// The ID of the user or organization to scope the user-to-server access token to. **Required** unless `target` is specified.
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_id: Option<i64>,
    /// The list of repository names to scope the user-to-server access token to. `repositories` may not be specified if `repository_ids` is specified.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<String>>,
    /// The list of repository IDs to scope the user-to-server access token to. `repository_ids` may not be specified if `repositories` is specified.
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<AppPermissions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutProjectsAddCollaborator {
    /// The permission to grant the collaborator.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateColumn {
    /// Name of the project column
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdate {
    /// The name of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// A short description of the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// A URL with more information about the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    /// Either `true` to make the repository private or `false` to make it public. Default: `false`.   **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://help.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private. **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://help.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    /// Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`. The `visibility` parameter overrides the `private` parameter when you use both along with the `nebula-preview` preview header.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    /// Either `true` to enable issues for this repository or `false` to disable them.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    /// Either `true` to enable the wiki for this repository or `false` to disable it.
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    /// Either `true` to make this repo available as a template repository or `false` to prevent it.
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    /// Updates the default branch for this repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,
    /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// `true` to archive this repository. **Note**: You cannot unarchive repositories through the API.
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsSetGithubActionsPermissionsRepository {
    pub enabled: ActionsEnabled,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_actions: Option<AllowedActions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsReviewPendingDeploymentsForRun {
    /// The list of environment ids to approve or reject
    pub environment_ids: Vec<i32>,
    /// Whether to approve or reject deployment to the specified environments. Must be one of: `approved` or `rejected`
    pub state: String,
    /// A comment to accompany the deployment review
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutActionsCreateOrUpdateRepoSecret {
    /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get a repository public key](https://docs.github.com/rest/reference/actions#get-a-repository-public-key) endpoint.
    #[serde(skip_serializing_if="Option::is_none")]
    pub encrypted_value: Option<String>,
    /// ID of the key you used to encrypt the secret.
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostActionsCreateWorkflowDispatch {
    /// The git reference for the workflow. The reference can be a branch or tag name.
    #[serde(rename = "ref")]
    pub _ref: String,
    /// Input keys and values configured in the workflow file. The maximum number of properties is 10. Any default properties configured in the workflow file will be used when `inputs` are omitted.
    #[serde(skip_serializing_if="Option::is_none")]
    pub inputs: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposUpdateBranchProtection {
    pub required_status_checks: ReposownerrepobranchesbranchprotectionRequiredStatusChecks,
    /// Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
    pub enforce_admins: bool,
    pub required_pull_request_reviews: ReposownerrepobranchesbranchprotectionRequiredPullRequestReviews,
    pub restrictions: ReposownerrepobranchesbranchprotectionRestrictions,
    /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see \"[Requiring a linear commit history](https://help.github.com/github/administering-a-repository/requiring-a-linear-commit-history)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<bool>,
    /// Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://help.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<bool>,
    /// Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see \"[Enabling force pushes to a protected branch](https://help.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)\" in the GitHub Help documentation.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdatePullRequestReviewProtection {
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions>,
    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    /// Blocks merging pull requests until [code owners](https://help.github.com/articles/about-code-owners/) have reviewed.
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    /// Specifies the number of reviewers required to approve pull requests. Use a number between 1 and 6.
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateStatusCheckProtection {
    /// Require branches to be up to date before merging.
    #[serde(skip_serializing_if="Option::is_none")]
    pub strict: Option<bool>,
    /// The list of status checks to require in order to merge into this branch
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostOauthAuthorizationsCreateAuthorization {
    /// A list of scopes that this authorization is in.
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// A note to remind you what the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    /// A URL to remind you what app the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,
    /// The OAuth app client key for which to create the token.
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_id: Option<String>,
    /// The OAuth app client secret for which to create the token.
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_secret: Option<String>,
    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetStatusCheckContexts {
    /// contexts parameter
    pub contexts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddStatusCheckContexts {
    /// contexts parameter
    pub contexts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveStatusCheckContexts {
    /// contexts parameter
    pub contexts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetAppAccessRestrictions {
    /// apps parameter
    pub apps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddAppAccessRestrictions {
    /// apps parameter
    pub apps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveAppAccessRestrictions {
    /// apps parameter
    pub apps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetTeamAccessRestrictions {
    /// teams parameter
    pub teams: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddTeamAccessRestrictions {
    /// teams parameter
    pub teams: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveTeamAccessRestrictions {
    /// teams parameter
    pub teams: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposSetUserAccessRestrictions {
    /// users parameter
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOauthAuthorizationsGetOrCreateAuthorizationForApp {
    /// The OAuth app client secret for which to create the token.
    pub client_secret: String,
    /// A list of scopes that this authorization is in.
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// A note to remind you what the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    /// A URL to remind you what app the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,
    /// A unique string to distinguish an authorization from others created for the same client ID and user.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposAddUserAccessRestrictions {
    /// users parameter
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposRemoveUserAccessRestrictions {
    /// users parameter
    pub users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposRenameBranch {
    /// The new name of the branch.
    pub new_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostChecksCreate {
    /// The name of the check. For example, \"code-coverage\".
    pub name: String,
    /// The SHA of the commit.
    pub head_sha: String,
    /// The URL of the integrator's site that has the full details of the check. If the integrator does not provide this, then the homepage of the GitHub app is used.
    #[serde(skip_serializing_if="Option::is_none")]
    pub details_url: Option<String>,
    /// A reference for the run on the integrator's system.
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    /// The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<String>,
    /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,
    /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub output: Option<ReposownerrepocheckrunsOutput>,
    /// Displays a button on GitHub that can be clicked to alert your app to do additional tasks. For example, a code linting app can display a button that automatically fixes detected errors. The button created in this object is displayed after the check run completes. When a user clicks the button, GitHub sends the [`check_run.requested_action` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) to your app. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\" To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<ReposownerrepocheckrunsActions>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchChecksUpdate {
    /// The name of the check. For example, \"code-coverage\".
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The URL of the integrator's site that has the full details of the check.
    #[serde(skip_serializing_if="Option::is_none")]
    pub details_url: Option<String>,
    /// A reference for the run on the integrator's system.
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<String>,
    /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`.   **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
    #[serde(skip_serializing_if="Option::is_none")]
    pub conclusion: Option<String>,
    /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub output: Option<ReposownerrepocheckrunscheckRunIdOutput>,
    /// Possible further actions the integrator can perform, which a user may trigger. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see \"[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions).\"
    #[serde(skip_serializing_if="Option::is_none")]
    pub actions: Option<Vec<ReposownerrepocheckrunsActions>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostChecksCreateSuite {
    /// The sha of the head commit.
    pub head_sha: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchChecksSetSuitesPreferences {
    /// Enables or disables automatic creation of CheckSuite events upon pushes to the repository. Enabled by default. See the [`auto_trigger_checks` object](https://docs.github.com/rest/reference/checks#auto_trigger_checks-object) description for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_trigger_checks: Option<Vec<ReposownerrepochecksuitespreferencesAutoTriggerChecks>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCodeScanningUpdateAlert {
    pub state: CodeScanningAlertSetState,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_reason: Option<CodeScanningAlertDismissedReason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCodeScanningUploadSarif {
    pub commit_sha: CodeScanningAnalysisCommitSha,
    #[serde(rename = "ref")]
    pub _ref: CodeScanningRef,
    pub sarif: CodeScanningAnalysisSarifFile,
    /// The base directory used in the analysis, as it appears in the SARIF file. This property is used to convert file paths from absolute to relative, so that alerts can be mapped to their correct location in the repository.
    #[serde(skip_serializing_if="Option::is_none")]
    pub checkout_uri: Option<String>,
    /// The time that the analysis run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the tool used to generate the code scanning analysis. If this parameter is not used, the tool name defaults to \"API\". If the uploaded SARIF contains a tool GUID, this will be available for filtering using the `tool_guid` parameter of operations such as `GET /repos/{owner}/{repo}/code-scanning/alerts`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub tool_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposAddCollaborator {
    /// The permission to grant the collaborator. **Only valid on organization-owned repositories.** Can be one of:   \\* `pull` - can pull, but not push to or administer this repository.   \\* `push` - can pull and push, but not administer this repository.   \\* `admin` - can pull, push and administer this repository.   \\* `maintain` - Recommended for project managers who need to manage the repository without access to sensitive or destructive actions.   \\* `triage` - Recommended for contributors who need to proactively manage issues and pull requests without write access.
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutOauthAuthorizationsGetOrCreateAuthorizationForAppAndFingerprint {
    /// The OAuth app client secret for which to create the token.
    pub client_secret: String,
    /// A list of scopes that this authorization is in.
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// A note to remind you what the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    /// A URL to remind you what app the OAuth token is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub note_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReposUpdateCommitComment {
    /// The contents of the comment
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReactionsCreateForCommitComment {
    /// The [reaction type](https://docs.github.com/rest/reference/reactions#reaction-types) to add to the commit comment.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateCommitComment {
    /// The contents of the comment.
    pub body: String,
    /// Relative path of the file to comment on.
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    /// Line index in the diff to comment on.
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,
    /// **Deprecated**. Use **position** parameter instead. Line number in the file to comment on.
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposCreateOrUpdateFileContents {
    /// The commit message.
    pub message: String,
    /// The new file content, using Base64 encoding.
    pub content: String,
    /// **Required if you are updating a file**. The blob SHA of the file being replaced.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    /// The branch name. Default: the repository’s default branch (usually `master`)
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepocontentspathCommitter>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepocontentspathAuthor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteReposDeleteFile {
    /// The commit message.
    pub message: String,
    /// The blob SHA of the file being replaced.
    pub sha: String,
    /// The branch name. Default: the repository’s default branch (usually `master`)
    #[serde(skip_serializing_if="Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<ReposownerrepocontentspathCommitter1>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<ReposownerrepocontentspathAuthor1>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeployment {
    /// The ref to deploy. This can be a branch, tag, or SHA.
    #[serde(rename = "ref")]
    pub _ref: String,
    /// Specifies a task to execute (e.g., `deploy` or `deploy:migrations`).
    #[serde(skip_serializing_if="Option::is_none")]
    pub task: Option<String>,
    /// Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch.
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_merge: Option<bool>,
    /// The [status](https://docs.github.com/rest/reference/repos#statuses) contexts to verify against commit status checks. If you omit this parameter, GitHub verifies all unique contexts before creating a deployment. To bypass checking entirely, pass an empty array. Defaults to all unique contexts.
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_contexts: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<Value>,
    /// Name for the target deployment environment (e.g., `production`, `staging`, `qa`).
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,
    /// Short description of the deployment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Specifies if the given environment is specific to the deployment and will no longer exist at some point in the future. Default: `false`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,
    /// Specifies if the given environment is one that end-users directly interact with. Default: `true` when `environment` is `production` and `false` otherwise.   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentStatus {
    /// The state of the status. Can be one of `error`, `failure`, `inactive`, `in_progress`, `queued` `pending`, or `success`. **Note:** To use the `inactive` state, you must provide the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. To use the `in_progress` and `queued` states, you must provide the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub.
    pub state: String,
    /// The target URL to associate with this status. This URL should contain output to keep the user updated while the task is running or serve as historical information for what happened in the deployment. **Note:** It's recommended to use the `log_url` parameter, which replaces `target_url`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_url: Option<String>,
    /// The full URL of the deployment's output. This parameter replaces `target_url`. We will continue to accept `target_url` to support legacy uses, but we recommend replacing `target_url` with `log_url`. Setting `log_url` will automatically set `target_url` to the same value. Default: `\"\"`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,
    /// A short description of the status. The maximum description length is 140 characters.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. **Note:** This parameter requires you to use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,
    /// Sets the URL for accessing your environment. Default: `\"\"`   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type. **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment_url: Option<String>,
    /// Adds a new `inactive` status to all prior non-transient, non-production environment deployments with the same repository and `environment` name as the created status's deployment. An `inactive` status is only added to deployments that had a `success` state. Default: `true`   **Note:** To add an `inactive` status to `production` environments, you must use the [`application/vnd.github.flash-preview+json`](https://docs.github.com/rest/overview/api-previews#deployment-statuses) custom media type.   **Note:** This parameter requires you to use the [`application/vnd.github.ant-man-preview+json`](https://docs.github.com/rest/overview/api-previews#enhanced-deployments) custom media type.
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_inactive: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDispatchEvent {
    /// A custom webhook event name.
    pub event_type: String,
    /// JSON payload with extra information about the webhook event that your action or worklow may use.
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_payload: Option<HashMap<String, HashMap<(), ()>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutReposCreateOrUpdateEnvironment {
    #[serde(skip_serializing_if="Option::is_none")]
    pub wait_timer: Option<WaitTimer>,
    /// The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewers: Option<Vec<ReposownerrepoenvironmentsenvironmentNameReviewers>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateFork {
    /// Optional parameter to specify the organization name if forking into an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
}

/// Branch Protection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchProtection {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    pub required_status_checks: BranchprotectionRequiredStatusChecks,
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforce_admins: Option<ProtectedBranchAdminEnforced>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedBranchPullRequestReview>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<BranchprotectionRequiredLinearHistory>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<BranchprotectionRequiredLinearHistory>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<BranchprotectionRequiredLinearHistory>,
    pub enabled: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_url: Option<String>,
}

/// Branch Restriction Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchRestrictionPolicy {
    pub url: String,
    pub users_url: String,
    pub teams_url: String,
    pub apps_url: String,
    pub users: Vec<RepositoryTemplateRepositoryOwner>,
    pub teams: Vec<BranchrestrictionpolicyTeams>,
    pub apps: Vec<BranchrestrictionpolicyApps>,
}

/// Branch Short
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchShort {
    pub name: String,
    pub commit: BranchshortCommit,
    pub protected: bool,
}

/// Branch With Protection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchWithProtection {
    pub name: String,
    pub commit: Commit,
    pub _links: BranchwithprotectionLinks,
    pub protected: bool,
    pub protection: BranchProtection,
    pub protection_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchprotectionRequiredLinearHistory {
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchprotectionRequiredStatusChecks {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    pub enforcement_level: String,
    pub contexts: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contexts_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyApps {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<BranchrestrictionpolicyOwner>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<BranchrestrictionpolicyPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyOwner {
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_members_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchrestrictionpolicyTeams {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchshortCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BranchwithprotectionLinks {
    pub html: String,
    #[serde(rename = "self")]
    pub _self: String,
}

/// Check Annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckAnnotation {
    pub path: String,
    pub start_line: i64,
    pub end_line: i64,
    pub start_column: i64,
    pub end_column: i64,
    pub annotation_level: String,
    pub title: String,
    pub message: String,
    pub raw_details: String,
    pub blob_href: String,
}

/// A check performed on the code of a given code change
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckRun {
    /// The id of the check.
    pub id: i64,
    /// The SHA of the commit that is being checked.
    pub head_sha: String,
    pub node_id: String,
    pub external_id: String,
    pub url: String,
    pub html_url: String,
    pub details_url: String,
    /// The phase of the lifecycle that the check is currently in.
    pub status: String,
    pub conclusion: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub output: CheckrunOutput,
    /// The name of the check.
    pub name: String,
    pub check_suite: CheckrunCheckSuite,
    pub app: Integration,
    pub pull_requests: Vec<PullRequestMinimal>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment: Option<DeploymentSimple>,
}

/// A suite of checks performed on the code of a given code change
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSuite {
    pub id: i64,
    pub node_id: String,
    pub head_branch: String,
    /// The SHA of the head commit that is being checked.
    pub head_sha: String,
    pub status: String,
    pub conclusion: String,
    pub url: String,
    pub before: String,
    pub after: String,
    pub pull_requests: Vec<PullRequestMinimal>,
    pub app: Integration,
    pub repository: MinimalRepository,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub head_commit: SimpleCommit,
    pub latest_check_runs_count: i64,
    pub check_runs_url: String,
}

/// Check suite configuration preferences for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSuitePreference {
    pub preferences: ChecksuitepreferencePreferences,
    pub repository: Repository,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckrunCheckSuite {
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckrunOutput {
    pub title: String,
    pub summary: String,
    pub text: String,
    pub annotations_count: i64,
    pub annotations_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecksuitepreferencePreferences {
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_trigger_checks: Option<Vec<ChecksuitepreferencePreferencesAutoTriggerChecks>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecksuitepreferencePreferencesAutoTriggerChecks {
    pub app_id: i64,
    pub setting: bool,
}

/// Clone Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloneTraffic {
    pub count: i64,
    pub uniques: i64,
    pub clones: Vec<Traffic>,
}

/// Code Frequency Stat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeFrequencyStat {
}

/// Code Of Conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeOfConduct {
    pub key: String,
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    pub html_url: String,
}

/// Code of Conduct Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeOfConductSimple {
    pub url: String,
    pub key: String,
    pub name: String,
    pub html_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlert {
    pub number: AlertNumber,
    pub created_at: AlertCreatedAt,
    pub url: AlertUrl,
    pub html_url: AlertHtmlUrl,
    pub instances_url: AlertInstancesUrl,
    pub state: CodeScanningAlertState,
    pub dismissed_by: SimpleUser,
    pub dismissed_at: CodeScanningAlertDismissedAt,
    pub dismissed_reason: CodeScanningAlertDismissedReason,
    pub rule: CodeScanningAlertRule,
    pub tool: CodeScanningAnalysisTool,
    pub most_recent_instance: CodeScanningAlertInstance,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub analysis_key: Option<CodeScanningAnalysisAnalysisKey>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<CodeScanningAlertEnvironment>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<CodeScanningAlertState>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<CodescanningalertinstanceMessage>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<CodeScanningAlertLocation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    /// Classifications that have been applied to the file that triggered the alert. For example identifying it as documentation, or a generated file.
    #[serde(skip_serializing_if="Option::is_none")]
    pub classifications: Option<Vec<CodeScanningAlertClassification>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertItems {
    pub number: AlertNumber,
    pub created_at: AlertCreatedAt,
    pub url: AlertUrl,
    pub html_url: AlertHtmlUrl,
    pub instances_url: AlertInstancesUrl,
    pub state: CodeScanningAlertState,
    pub dismissed_by: SimpleUser,
    pub dismissed_at: CodeScanningAlertDismissedAt,
    pub dismissed_reason: CodeScanningAlertDismissedReason,
    pub rule: CodeScanningAlertRuleSummary,
    pub tool: CodeScanningAnalysisTool,
    pub most_recent_instance: CodeScanningAlertInstance,
}

/// Describe a region within a file for the alert.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertLocation {
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_line: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_column: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_column: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertRule {
    /// A unique identifier for the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    /// The name of the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The severity of the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub severity: Option<String>,
    /// A short description of the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    /// description of the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_description: Option<String>,
    /// A set of tags applicable for the rule.
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Detailed documentation for the rule as GitHub Flavored Markdown.
    #[serde(skip_serializing_if="Option::is_none")]
    pub help: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAlertRuleSummary {
    /// A unique identifier for the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    /// The name of the rule used to detect the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The severity of the alert.
    #[serde(skip_serializing_if="Option::is_none")]
    pub severity: Option<String>,
    /// A short description of the rule used to detect the alert.
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
    pub _ref: CodeScanningRef,
    pub commit_sha: CodeScanningAnalysisCommitSha,
    pub analysis_key: CodeScanningAnalysisAnalysisKey,
    pub environment: CodeScanningAnalysisEnvironment,
    pub error: String,
    pub created_at: CodeScanningAnalysisCreatedAt,
    /// The total number of results in the analysis.
    pub results_count: i64,
    /// The total number of rules used in the analysis.
    pub rules_count: i64,
    /// Unique identifier for this analysis.
    pub id: i64,
    pub url: CodeScanningAnalysisUrl,
    pub sarif_id: CodeScanningAnalysisSarifId,
    pub tool: CodeScanningAnalysisTool,
    pub deletable: bool,
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
    pub next_analysis_url: String,
    /// Next deletable analysis in chain, with last analysis deletion confirmation
    pub confirm_delete_url: String,
}

/// Identifies the variable values associated with the environment in which this analysis was performed.
pub type CodeScanningAnalysisEnvironment = String;

/// A Base64 string representing the SARIF file to upload. You must first compress your SARIF file using [`gzip`](http://www.gnu.org/software/gzip/manual/gzip.html) and then translate the contents of the file into a Base64 encoding string. For more information, see \"[SARIF support for code scanning](https://docs.github.com/github/finding-security-vulnerabilities-and-errors-in-your-code/sarif-support-for-code-scanning).\"
pub type CodeScanningAnalysisSarifFile = String;

/// An identifier for the upload.
pub type CodeScanningAnalysisSarifId = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningAnalysisTool {
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<CodeScanningAnalysisToolName>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<CodeScanningAnalysisToolVersion>,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<CodeScanningAnalysisSarifId>,
    /// The REST API URL for checking the status of the upload.
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeScanningSarifsStatus {
    /// `pending` files have not yet been processed, while `complete` means all results in the SARIF have been stored.
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_status: Option<String>,
    /// The REST API URL for getting the analyses associated with the upload.
    #[serde(skip_serializing_if="Option::is_none")]
    pub analyses_url: Option<String>,
}

/// Code Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSearchResultItem {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub repository: MinimalRepository,
    pub score: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub line_numbers: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodescanningalertinstanceMessage {
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
}

/// Collaborator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<CollaboratorPermissions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollaboratorPermissions {
    pub pull: bool,
    pub push: bool,
    pub admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombinedBillingUsage {
    /// Numbers of days left in billing cycle.
    pub days_left_in_billing_cycle: i64,
    /// Estimated storage space (GB) used in billing cycle.
    pub estimated_paid_storage_for_month: i64,
    /// Estimated sum of free and paid storage space (GB) used in billing cycle.
    pub estimated_storage_for_month: i64,
}

/// Combined Commit Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombinedCommitStatus {
    pub state: String,
    pub statuses: Vec<SimpleCommitStatus>,
    pub sha: String,
    pub total_count: i64,
    pub repository: MinimalRepository,
    pub commit_url: String,
    pub url: String,
}

/// Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitCommit,
    pub author: SimpleUser,
    pub committer: SimpleUser,
    pub parents: Vec<CommitParents>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stats: Option<CommitStats>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<Vec<CommitFiles>>,
}

/// Commit Activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitActivity {
    pub days: Vec<i32>,
    pub total: i64,
    pub week: i64,
}

/// Commit Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComment {
    pub html_url: String,
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub body: String,
    pub path: String,
    pub position: i64,
    pub line: i64,
    pub commit_id: String,
    pub user: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommit {
    pub url: String,
    pub author: GitUser,
    pub committer: GitUser,
    pub message: String,
    pub comment_count: i64,
    pub tree: CommitCommitTree,
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommitTree {
    pub sha: String,
    pub url: String,
}

/// Commit Comparison
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitComparison {
    pub url: String,
    pub html_url: String,
    pub permalink_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub base_commit: Commit,
    pub merge_base_commit: Commit,
    pub status: String,
    pub ahead_by: i64,
    pub behind_by: i64,
    pub total_commits: i64,
    pub commits: Vec<Commit>,
    pub files: Vec<DiffEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitFiles {
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub changes: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub blob_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitParents {
    pub sha: String,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
}

/// Commit Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitSearchResultItem {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitsearchresultitemCommit,
    pub author: SimpleUser,
    pub committer: GitUser,
    pub parents: Vec<FilecommitCommitParents>,
    pub repository: MinimalRepository,
    pub score: i64,
    pub node_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitStats {
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitsearchresultitemCommit {
    pub author: CommitsearchresultitemCommitAuthor,
    pub committer: GitUser,
    pub comment_count: i64,
    pub message: String,
    pub tree: ShortbranchCommit,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitsearchresultitemCommitAuthor {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityHealthFile {
    pub url: String,
    pub html_url: String,
}

/// Community Profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityProfile {
    pub health_percentage: i64,
    pub description: String,
    pub documentation: String,
    pub files: CommunityprofileFiles,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_reports_enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommunityprofileFiles {
    pub code_of_conduct: CodeOfConductSimple,
    pub license: LicenseSimple,
    pub contributing: CommunityHealthFile,
    pub readme: CommunityHealthFile,
    pub issue_template: CommunityHealthFile,
    pub pull_request_template: CommunityHealthFile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerMetadata {
    pub tags: Vec<Value>,
}

/// A list of directory items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentDirectory {
}

/// Content File
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentFile {
    #[serde(rename = "type")]
    pub _type: String,
    pub encoding: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: ContenttreeLinks,
    #[serde(skip_serializing_if="Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub submodule_git_url: Option<String>,
}

/// Content Reference attachments allow you to provide context around URLs posted in comments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentReferenceAttachment {
    /// The ID of the attachment
    pub id: i64,
    /// The title of the attachment
    pub title: String,
    /// The body of the attachment
    pub body: String,
    /// The node_id of the content attachment
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
}

/// An object describing a symlink
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSubmodule {
    #[serde(rename = "type")]
    pub _type: String,
    pub submodule_git_url: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: ContenttreeLinks,
}

/// An object describing a symlink
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSymlink {
    #[serde(rename = "type")]
    pub _type: String,
    pub target: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: ContenttreeLinks,
}

/// Content Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTraffic {
    pub path: String,
    pub title: String,
    pub count: i64,
    pub uniques: i64,
}

/// Content Tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentTree {
    #[serde(rename = "type")]
    pub _type: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub entries: Option<Vec<ContenttreeEntries>>,
    pub _links: ContenttreeLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentdirectoryInner {
    #[serde(rename = "type")]
    pub _type: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: HashMap<(), ()>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContenttreeEntries {
    #[serde(rename = "type")]
    pub _type: String,
    pub size: i64,
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    pub _links: ContenttreeLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContenttreeLinks {
    pub git: String,
    pub html: String,
    #[serde(rename = "self")]
    pub _self: String,
}

/// Contributor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,
    pub contributions: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

/// Contributor Activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributorActivity {
    pub author: SimpleUser,
    pub total: i64,
    pub weeks: Vec<ContributoractivityWeeks>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributoractivityWeeks {
    #[serde(skip_serializing_if="Option::is_none")]
    pub w: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub a: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub d: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub c: Option<i64>,
}

/// Credential Authorization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialAuthorization {
    /// User login that owns the underlying credential.
    pub login: String,
    /// Unique identifier for the credential.
    pub credential_id: i64,
    /// Human-readable description of the credential type.
    pub credential_type: String,
    /// Last eight characters of the credential. Only included in responses with credential_type of personal access token.
    #[serde(skip_serializing_if="Option::is_none")]
    pub token_last_eight: Option<String>,
    /// Date when the credential was authorized for use.
    pub credential_authorized_at: chrono::DateTime<chrono::Utc>,
    /// List of oauth scopes the token has been granted.
    #[serde(skip_serializing_if="Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// Unique string to distinguish the credential. Only included in responses with credential_type of SSH Key.
    #[serde(skip_serializing_if="Option::is_none")]
    pub fingerprint: Option<String>,
    /// Date when the credential was last accessed. May be null if it was never accessed
    #[serde(skip_serializing_if="Option::is_none")]
    pub credential_accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_id: Option<i64>,
    /// The title given to the ssh key. This will only be present when the credential is an ssh key.
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_title: Option<String>,
    /// The note given to the token. This will only be present when the credential is a token.
    #[serde(skip_serializing_if="Option::is_none")]
    pub authorized_credential_note: Option<String>,
}

/// An SSH key granting access to a single repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeployKey {
    pub id: i64,
    pub key: String,
    pub url: String,
    pub title: String,
    pub verified: bool,
    pub created_at: String,
    pub read_only: bool,
}

/// A request for a specific ref(branch,sha,tag) to be deployed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    pub url: String,
    /// Unique identifier of the deployment
    pub id: i64,
    pub node_id: String,
    pub sha: String,
    /// The ref to deploy. This can be a branch, tag, or sha.
    #[serde(rename = "ref")]
    pub _ref: String,
    /// Parameter to specify a task to execute
    pub task: String,
    pub payload: HashMap<(), ()>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_environment: Option<String>,
    /// Name for the target deployment environment.
    pub environment: String,
    pub description: String,
    pub creator: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub statuses_url: String,
    pub repository_url: String,
    /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,
    /// Specifies if the given environment is one that end-users directly interact with. Default: false.
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
}

/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentBranchPolicy {
    /// Whether only branches with branch protection rules can deploy to this environment. If `protected_branches` is `true`, `custom_branch_policies` must be `false`; if `protected_branches` is `false`, `custom_branch_policies` must be `true`.
    pub protected_branches: bool,
    /// Whether only branches that match the specified name patterns can deploy to this environment.  If `custom_branch_policies` is `true`, `protected_branches` must be `false`; if `custom_branch_policies` is `false`, `protected_branches` must be `true`.
    pub custom_branch_policies: bool,
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
    pub url: String,
    /// Unique identifier of the deployment
    pub id: i64,
    pub node_id: String,
    /// Parameter to specify a task to execute
    pub task: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_environment: Option<String>,
    /// Name for the target deployment environment.
    pub environment: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub statuses_url: String,
    pub repository_url: String,
    /// Specifies if the given environment is will no longer exist at some point in the future. Default: false.
    #[serde(skip_serializing_if="Option::is_none")]
    pub transient_environment: Option<bool>,
    /// Specifies if the given environment is one that end-users directly interact with. Default: false.
    #[serde(skip_serializing_if="Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
}

/// The status of a deployment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatus {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    /// The state of the status.
    pub state: String,
    pub creator: SimpleUser,
    /// A short description of the status.
    pub description: String,
    /// The environment of the deployment that the status is for.
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment: Option<String>,
    /// Deprecated: the URL to associate with this status.
    pub target_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deployment_url: String,
    pub repository_url: String,
    /// The URL for accessing your environment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub environment_url: Option<String>,
    /// The URL to associate with this status.
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
}

/// Diff Entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffEntry {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockerMetadata {
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag: Option<Vec<Value>>,
}

/// Email
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
    pub visibility: String,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    pub html_url: String,
    /// The enterprise's website URL.
    #[serde(skip_serializing_if="Option::is_none")]
    pub website_url: Option<String>,
    /// Unique identifier of the enterprise
    pub id: i64,
    pub node_id: String,
    /// The name of the enterprise.
    pub name: String,
    /// The slug url identifier for the enterprise.
    pub slug: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub avatar_url: String,
}

/// Details of a deployment environment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    /// The id of the environment.
    pub id: i64,
    pub node_id: String,
    /// The name of the environment.
    pub name: String,
    pub url: String,
    pub html_url: String,
    /// The time that the environment was created, in ISO 8601 format.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The time that the environment was last updated, in ISO 8601 format.
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_rules: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployment_branch_policy: Option<DeploymentBranchPolicy>,
}

/// An entry in the reviews log for environment deployments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentApprovals {
    /// The list of environments that were approved or rejected
    pub environments: Vec<EnvironmentapprovalsEnvironments>,
    /// Whether deployment to the environment(s) was approved or rejected
    pub state: String,
    pub user: SimpleUser,
    /// The comment submitted with the deployment review
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentapprovalsEnvironments {
    /// The id of the environment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    /// The name of the environment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    /// The time that the environment was created, in ISO 8601 format.
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The time that the environment was last updated, in ISO 8601 format.
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub actor: Actor,
    pub repo: EventRepo,
    #[serde(skip_serializing_if="Option::is_none")]
    pub org: Option<Actor>,
    pub payload: EventPayload,
    pub public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventPayload {
    pub action: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<IssueComment>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<Vec<EventPayloadPages>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventPayloadPages {
    #[serde(skip_serializing_if="Option::is_none")]
    pub page_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventRepo {
    pub id: i64,
    pub name: String,
    pub url: String,
}

/// Feed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub timeline_url: String,
    pub user_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_public_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_actor_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_advisories_url: Option<String>,
    pub _links: FeedLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedLinks {
    pub timeline: LinkWithType,
    pub user: LinkWithType,
    #[serde(skip_serializing_if="Option::is_none")]
    pub security_advisories: Option<LinkWithType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user: Option<LinkWithType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_public: Option<LinkWithType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_actor: Option<LinkWithType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organization: Option<LinkWithType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub current_user_organizations: Option<Vec<LinkWithType>>,
}

/// File Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileCommit {
    pub content: FilecommitContent,
    pub commit: FilecommitCommit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommit {
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author: Option<FilecommitCommitAuthor>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub committer: Option<FilecommitCommitAuthor>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tree: Option<FilecommitCommitTree>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parents: Option<Vec<FilecommitCommitParents>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<FilecommitCommitVerification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitAuthor {
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitParents {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitTree {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitCommitVerification {
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitContent {
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub _links: Option<FilecommitContentLinks>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilecommitContentLinks {
    #[serde(rename = "self")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _self: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html: Option<String>,
}

/// Full Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullRepository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: SimpleUser,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: String,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub language: String,
    pub forks_count: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub size: i64,
    pub default_branch: String,
    pub open_issues_count: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    pub subscribers_count: i64,
    pub network_count: i64,
    pub license: LicenseSimple,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<Repository>,
    pub forks: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
    pub open_issues: i64,
    pub watchers: i64,
    /// Whether anonymous git access is allowed.
    #[serde(skip_serializing_if="Option::is_none")]
    pub anonymous_access_enabled: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_of_conduct: Option<CodeOfConductSimple>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullrepositoryPermissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

/// A comment made to a gist.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistComment {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    /// The comment text.
    pub body: String,
    pub user: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_association: AuthorAssociation,
}

/// Gist Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistCommit {
    pub url: String,
    pub version: String,
    pub user: SimpleUser,
    pub change_status: GistcommitChangeStatus,
    pub committed_at: chrono::DateTime<chrono::Utc>,
}

/// Gist Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistSimple {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_pull_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_push_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub files: Option<HashMap<String, GistsimpleFiles>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistcommitChangeStatus {
    #[serde(skip_serializing_if="Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub additions: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletions: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsFiles {
    /// Content of the file
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsgistIdFiles {
    /// The new content of the file
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
    /// The new filename for the file
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GistsimpleFiles {
    #[serde(skip_serializing_if="Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
}

/// Low-level Git commit operations within a repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitCommit {
    /// SHA for the commit
    pub sha: String,
    pub node_id: String,
    pub url: String,
    pub author: GitcommitAuthor,
    pub committer: GitcommitAuthor,
    /// Message describing the purpose of the commit
    pub message: String,
    pub tree: GitcommitTree,
    pub parents: Vec<GitcommitParents>,
    pub verification: GitcommitVerification,
    pub html_url: String,
}

/// Git references within a repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitRef {
    #[serde(rename = "ref")]
    pub _ref: String,
    pub node_id: String,
    pub url: String,
    pub object: GitrefObject,
}

/// Metadata for a Git tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitTag {
    pub node_id: String,
    /// Name of the tag
    pub tag: String,
    pub sha: String,
    /// URL for the tag
    pub url: String,
    /// Message describing the purpose of the tag
    pub message: String,
    pub tagger: GittagTagger,
    pub object: GittagObject,
    #[serde(skip_serializing_if="Option::is_none")]
    pub verification: Option<Verification>,
}

/// The hierarchy between files in a Git repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitTree {
    pub sha: String,
    pub url: String,
    pub truncated: bool,
    /// Objects specifying a tree structure
    pub tree: Vec<GittreeTree>,
}

/// Metaproperties for Git author/committer information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitUser {
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

/// Identifying information for the git-user
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitAuthor {
    /// Timestamp of the commit
    pub date: chrono::DateTime<chrono::Utc>,
    /// Git email address of the user
    pub email: String,
    /// Name of the git user
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitParents {
    /// SHA for the commit
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitTree {
    /// SHA for the commit
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitcommitVerification {
    pub verified: bool,
    pub reason: String,
    pub signature: String,
    pub payload: String,
}

/// Gitignore Template
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitignoreTemplate {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitrefObject {
    #[serde(rename = "type")]
    pub _type: String,
    /// SHA for the reference
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittagObject {
    pub sha: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittagTagger {
    pub date: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GittreeTree {
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

/// A unique encryption key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgKey {
    pub id: i64,
    pub primary_key_id: i64,
    pub key_id: String,
    pub public_key: String,
    pub emails: Vec<GpgkeyEmails>,
    pub subkeys: Vec<GpgkeySubkeys>,
    pub can_sign: bool,
    pub can_encrypt_comms: bool,
    pub can_encrypt_storage: bool,
    pub can_certify: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub raw_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgkeyEmails {
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpgkeySubkeys {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary_key_id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subkeys: Option<Vec<Value>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_sign: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_comms: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_encrypt_storage: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub can_certify: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_key: Option<String>,
}

/// External Groups to be mapped to a team for membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupMapping {
    /// Array of groups to be mapped to this team
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<GroupmappingGroups>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupmappingGroups {
    /// The ID of the group
    pub group_id: String,
    /// The name of the group
    pub group_name: String,
    /// a description of the group
    pub group_description: String,
    /// synchronization status for this group mapping
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    /// the time of the last sync for this group-mapping
    #[serde(skip_serializing_if="Option::is_none")]
    pub synced_at: Option<String>,
}

/// Webhooks for repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    #[serde(rename = "type")]
    pub _type: String,
    /// Unique identifier of the webhook.
    pub id: i64,
    /// The name of a valid service, use 'web' for a webhook.
    pub name: String,
    /// Determines whether the hook is actually triggered on pushes.
    pub active: bool,
    /// Determines what events the hook is triggered for. Default: ['push'].
    pub events: Vec<String>,
    pub config: HookConfig,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub test_url: String,
    pub ping_url: String,
    pub last_response: HookResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookConfig {
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub room: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subdomain: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookResponse {
    pub code: i64,
    pub status: String,
    pub message: String,
}

/// Hovercard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hovercard {
    pub contexts: Vec<HovercardContexts>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HovercardContexts {
    pub message: String,
    pub octicon: String,
}

/// A repository import from an external source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub vcs: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub use_lfs: Option<String>,
    /// The URL of the originating repository.
    pub vcs_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub svc_root: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub status_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_step: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub import_percent: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub push_percent: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_large_files: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub large_files_size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub large_files_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_choices: Option<Vec<ImportProjectChoices>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub authors_count: Option<i64>,
    pub url: String,
    pub html_url: String,
    pub authors_url: String,
    pub repository_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportProjectChoices {
    #[serde(skip_serializing_if="Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tfvc_project: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub human_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetaRootResponse200 {
    pub current_user_url: String,
    pub current_user_authorizations_html_url: String,
    pub authorizations_url: String,
    pub code_search_url: String,
    pub commit_search_url: String,
    pub emails_url: String,
    pub emojis_url: String,
    pub events_url: String,
    pub feeds_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub hub_url: String,
    pub issue_search_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub label_search_url: String,
    pub notifications_url: String,
    pub organization_url: String,
    pub organization_repositories_url: String,
    pub organization_teams_url: String,
    pub public_gists_url: String,
    pub rate_limit_url: String,
    pub repository_url: String,
    pub repository_search_url: String,
    pub current_user_repositories_url: String,
    pub starred_url: String,
    pub starred_gists_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_search_url: Option<String>,
    pub user_url: String,
    pub user_organizations_url: String,
    pub user_repositories_url: String,
    pub user_search_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsCheckAuthorizationResponse200 {
    pub id: i64,
    pub url: String,
    /// A list of scopes that this authorization is in.
    pub scopes: Vec<String>,
    pub token: String,
    pub token_last_eight: String,
    pub hashed_token: String,
    pub app: ApplicationgrantApp,
    pub note: String,
    pub note_url: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fingerprint: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub user: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub installation: Option<ScopedInstallation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListOrgSecretsResponse200 {
    pub total_count: i64,
    pub secrets: Vec<OrganizationActionsSecret>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelectedReposForOrgSecretResponse200 {
    pub total_count: i64,
    pub repositories: Vec<MinimalRepository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListInstallationsForAuthenticatedUserResponse200 {
    pub total_count: i64,
    pub installations: Vec<Installation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListWorkflowRunArtifactsResponse200 {
    pub total_count: i64,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListWorkflowRunsResponse200 {
    pub total_count: i64,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListJobsForWorkflowRunResponse200 {
    pub total_count: i64,
    pub jobs: Vec<Job>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListEnvironmentSecretsResponse200 {
    pub total_count: i64,
    pub secrets: Vec<ActionsSecret>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListRepoWorkflowsResponse200 {
    pub total_count: i64,
    pub workflows: Vec<Workflow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChecksListForRefResponse200 {
    pub total_count: i64,
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChecksListSuitesForRefResponse200 {
    pub total_count: i64,
    pub check_suites: Vec<CheckSuite>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListOrgAccessToSelfHostedRunnerGroupInEnterpriseResponse200 {
    pub total_count: f64,
    pub organizations: Vec<OrganizationSimple>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20020 {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReposGetAllEnvironmentsResponse200 {
    /// The number of environments in this repository
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub environments: Option<Vec<Environment>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchCodeResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<CodeSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchCommitsResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<CommitSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchIssuesAndPullRequestsResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<IssueSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchLabelsResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<LabelSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchReposResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<RepoSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchTopicsResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<TopicSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchUsersResponse200 {
    pub total_count: i64,
    pub incomplete_results: bool,
    pub items: Vec<UserSearchResultItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsersGetByUsernameResponse200 {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListSelfHostedRunnerGroupsForEnterpriseResponse200 {
    pub total_count: f64,
    pub runner_groups: Vec<RunnerGroupsEnterprise>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListInstallationReposForAuthenticatedUserResponse200 {
    pub total_count: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnersInGroupForOrgResponse200 {
    pub total_count: f64,
    pub runners: Vec<Runner>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnterpriseAdminListSelfHostedRunnersForEnterpriseResponse200 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_count: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub runners: Option<Vec<Runner>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppsListReposAccessibleToInstallationResponse200 {
    pub total_count: i64,
    pub repositories: Vec<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListRepoAccessToSelfHostedRunnerGroupInOrgResponse200 {
    pub total_count: f64,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnerGroupsForOrgResponse200 {
    pub total_count: f64,
    pub runner_groups: Vec<RunnerGroupsOrg>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetActionsListSelfHostedRunnersForRepoResponse200 {
    pub total_count: i64,
    pub runners: Vec<Runner>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostAppsCreateFromManifestResponse201 {
    /// Unique identifier of the GitHub app
    pub id: i64,
    /// The slug name of the GitHub app
    #[serde(skip_serializing_if="Option::is_none")]
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: SimpleUser,
    /// The name of the GitHub app
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub html_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub permissions: IntegrationPermissions,
    /// The list of events for the GitHub app
    pub events: Vec<String>,
    /// The number of installations associated with the GitHub app
    #[serde(skip_serializing_if="Option::is_none")]
    pub installations_count: Option<i64>,
    pub client_id: String,
    pub client_secret: String,
    pub webhook_secret: String,
    pub pem: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentResponse202 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutPullsUpdateBranchResponse202 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGistsGetCommentResponse403 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub block: Option<InlineResponse403Block>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateProjectPermissionsLegacyResponse403 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectsUpdateResponse403 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsMoveCardResponse403 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4033Errors>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse4033Errors {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse403Block {
    #[serde(skip_serializing_if="Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposMergeResponse409 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostReposCreateDeploymentResponse409 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProjectsListForUserResponse415 {
    pub message: String,
    pub documentation_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCardResponse422 {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserInOrgResponse422 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse4221Errors {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddMemberLegacyResponse422 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutTeamsAddOrUpdateMembershipForUserLegacyResponse422 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse4221Errors>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchUsersResponse503 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostProjectsCreateCardResponse503 {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<InlineResponse5031Errors>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse5031Errors {
    #[serde(skip_serializing_if="Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
}

/// Installation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Installation {
    /// The ID of the installation.
    pub id: i64,
    pub account: Value,
    /// Describe whether all repositories have been selected or there's a selection involved
    pub repository_selection: String,
    pub access_tokens_url: String,
    pub repositories_url: String,
    pub html_url: String,
    pub app_id: i64,
    /// The ID of the user or organization this token is being scoped to.
    pub target_id: i64,
    pub target_type: String,
    pub permissions: InstallationPermissions,
    pub events: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub single_file_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,
    pub app_slug: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_by: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_requests: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_administration: Option<String>,
}

/// Authentication token for a GitHub App installed on a user or org.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationToken {
    pub token: String,
    pub expires_at: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<InstallationtokenPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_selection: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallationtokenPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub checks: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents: Option<String>,
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
    pub limit: InteractionGroup,
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiry: Option<InteractionExpiry>,
}

/// Interaction limit settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionLimitResponse {
    pub limit: InteractionGroup,
    pub origin: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub node_id: String,
    /// URL for the issue
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    /// Number uniquely identifying the issue within its repository
    pub number: i64,
    /// State of the issue; either 'open' or 'closed'
    pub state: String,
    /// Title of the issue
    pub title: String,
    /// Contents of the issue
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    pub user: SimpleUser,
    /// Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
    pub labels: Vec<Value>,
    pub assignee: SimpleUser,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    pub milestone: Milestone,
    pub locked: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_by: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}

/// Comments provide a way for people to collaborate on an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueComment {
    /// Unique identifier of the issue comment
    pub id: i64,
    pub node_id: String,
    /// URL for the issue comment
    pub url: String,
    /// Contents of the issue comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    pub html_url: String,
    pub user: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub issue_url: String,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}

/// Issue Event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEvent {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub actor: SimpleUser,
    pub event: String,
    pub commit_id: String,
    pub commit_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue: Option<IssueSimple>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<IssueEventLabel>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignee: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assigner: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub review_requester: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewer: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_team: Option<Team>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissed_review: Option<IssueEventDismissedReview>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestone: Option<IssueEventMilestone>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub project_card: Option<IssueEventProjectCard>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rename: Option<IssueEventRename>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventDismissedReview {
    pub state: String,
    pub review_id: i64,
    pub dismissal_message: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_commit_id: Option<String>,
}

/// Issue Event for Issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventForIssue {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub actor: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub author_association: Option<AuthorAssociation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub lock_reason: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
}

/// Issue Event Label
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventLabel {
    pub name: String,
    pub color: String,
}

/// Issue Event Milestone
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventMilestone {
    pub title: String,
}

/// Issue Event Project Card
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventProjectCard {
    pub url: String,
    pub id: i64,
    pub project_url: String,
    pub project_id: i64,
    pub column_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub previous_column_name: Option<String>,
}

/// Issue Event Rename
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueEventRename {
    pub from: String,
    pub to: String,
}

/// Issue Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueSearchResultItem {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub locked: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    pub user: SimpleUser,
    pub labels: Vec<IssuesearchresultitemLabels>,
    pub state: String,
    pub assignee: SimpleUser,
    pub milestone: Milestone,
    pub comments: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    pub score: i64,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
}

/// Issue Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueSimple {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    pub user: SimpleUser,
    pub labels: Vec<Label>,
    pub assignee: SimpleUser,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    pub milestone: Milestone,
    pub locked: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull_request: Option<IssuesimplePullRequest>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub performed_via_github_app: Option<Integration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuesearchresultitemLabels {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssuesimplePullRequest {
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    pub diff_url: String,
    pub html_url: String,
    pub patch_url: String,
    pub url: String,
}

/// Information of a job execution in a workflow run
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    /// The id of the job.
    pub id: i64,
    /// The id of the associated workflow run.
    pub run_id: i64,
    pub run_url: String,
    pub node_id: String,
    /// The SHA of the commit that is being run.
    pub head_sha: String,
    pub url: String,
    pub html_url: String,
    /// The phase of the lifecycle that the job is currently in.
    pub status: String,
    /// The outcome of the job.
    pub conclusion: String,
    /// The time that the job started, in ISO 8601 format.
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The time that the job finished, in ISO 8601 format.
    pub completed_at: chrono::DateTime<chrono::Utc>,
    /// The name of the job.
    pub name: String,
    /// Steps in this job.
    #[serde(skip_serializing_if="Option::is_none")]
    pub steps: Option<Vec<JobSteps>>,
    pub check_run_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobSteps {
    /// The phase of the lifecycle that the job is currently in.
    pub status: String,
    /// The outcome of the job.
    pub conclusion: String,
    /// The name of the job.
    pub name: String,
    pub number: i64,
    /// The time that the step started, in ISO 8601 format.
    #[serde(skip_serializing_if="Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The time that the job finished, in ISO 8601 format.
    #[serde(skip_serializing_if="Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub key_id: String,
    pub key: String,
    pub id: i64,
    pub url: String,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
    pub read_only: bool,
}

/// Key Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySimple {
    pub id: i64,
    pub key: String,
}

/// Color-coded labels help you categorize and filter your issues (just like labels in Gmail).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub node_id: String,
    /// URL for the label
    pub url: String,
    /// The name of the label.
    pub name: String,
    pub description: String,
    /// 6-character hex code, without the leading #, identifying the color
    pub color: String,
    pub default: bool,
}

/// Label Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelSearchResultItem {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: String,
    pub score: i64,
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
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
    pub html_url: String,
    pub description: String,
    pub implementation: String,
    pub permissions: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub body: String,
    pub featured: bool,
}

/// License Content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub content: String,
    pub encoding: String,
    pub _links: ContenttreeLinks,
    pub license: LicenseSimple,
}

/// License Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseSimple {
    pub key: String,
    pub name: String,
    pub url: String,
    pub spdx_id: String,
    pub node_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
}

/// Hypermedia Link
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}

/// Hypermedia Link with Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkWithType {
    pub href: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceAccount {
    pub url: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    pub login: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_billing_email: Option<String>,
}

/// Marketplace Listing Plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceListingPlan {
    pub url: String,
    pub accounts_url: String,
    pub id: i64,
    pub number: i64,
    pub name: String,
    pub description: String,
    pub monthly_price_in_cents: i64,
    pub yearly_price_in_cents: i64,
    pub price_model: String,
    pub has_free_trial: bool,
    pub unit_name: String,
    pub state: String,
    pub bullets: Vec<String>,
}

/// Marketplace Purchase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacePurchase {
    pub url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub id: i64,
    pub login: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_billing_email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub marketplace_pending_change: Option<MarketplacepurchaseMarketplacePendingChange>,
    pub marketplace_purchase: MarketplacepurchaseMarketplacePurchase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacepurchaseMarketplacePendingChange {
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketplacepurchaseMarketplacePurchase {
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_installed: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub unit_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub on_free_trial: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub free_trial_ends_on: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<MarketplaceListingPlan>,
}

/// A migration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Migration {
    pub id: i64,
    pub owner: SimpleUser,
    pub guid: String,
    pub state: String,
    pub lock_repositories: bool,
    pub exclude_attachments: bool,
    pub repositories: Vec<Repository>,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub node_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude: Option<Vec<Value>>,
}

/// A collection of related issues and pull requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    /// The number of the milestone.
    pub number: i64,
    /// The state of the milestone.
    pub state: String,
    /// The title of the milestone.
    pub title: String,
    pub description: String,
    pub creator: SimpleUser,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    pub due_on: chrono::DateTime<chrono::Utc>,
}

/// Minimal Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimalRepository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: SimpleUser,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryTemplateRepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<MinimalrepositoryLicense>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinimalrepositoryLicense {
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub spdx_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
}

/// Org Hook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgHook {
    pub id: i64,
    pub url: String,
    pub ping_url: String,
    pub name: String,
    pub events: Vec<String>,
    pub active: bool,
    pub config: OrghookConfig,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub _type: String,
}

/// Org Membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgMembership {
    pub url: String,
    pub state: String,
    pub role: String,
    pub organization_url: String,
    pub organization: OrganizationSimple,
    pub user: SimpleUser,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<OrgmembershipPermissions>,
}

/// Secrets for GitHub Actions for an organization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationActionsSecret {
    /// The name of the secret.
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Visibility of a secret
    pub visibility: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

/// Organization Full
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationFull {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_verified: Option<bool>,
    pub has_organization_projects: bool,
    pub has_repository_projects: bool,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub html_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_private_repos: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_private_repos: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<OrganizationfullPlan>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_repository_permission: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members_can_create_pages: Option<bool>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Organization Invitation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    pub id: i64,
    pub login: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub failed_reason: Option<String>,
    pub inviter: SimpleUser,
    pub team_count: i64,
    pub invitation_team_url: String,
    pub node_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub invitation_teams_url: Option<String>,
}

/// Organization Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSimple {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationfullPlan {
    pub name: String,
    pub space: i64,
    pub private_repos: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub filled_seats: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub seats: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrghookConfig {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgmembershipPermissions {
    pub can_create_repository: bool,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorghooksConfig {
    pub url: WebhookConfigUrl,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#update-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorghookshookIdConfig {
    pub url: WebhookConfigUrl,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrgsorgteamsteamSlugteamsyncgroupmappingsGroups {
    /// ID of the IdP group.
    pub group_id: String,
    /// Name of the IdP group.
    pub group_name: String,
    /// Description of the IdP group.
    pub group_description: String,
}

/// A software package
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    /// Unique identifier of the package.
    pub id: i64,
    /// The name of the package.
    pub name: String,
    pub package_type: String,
    pub url: String,
    pub html_url: String,
    /// The number of versions of the package.
    pub version_count: i64,
    pub visibility: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<SimpleUser>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository: Option<MinimalRepository>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A version of a software package
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageVersion {
    /// Unique identifier of the package version.
    pub id: i64,
    /// The name of the package version.
    pub name: String,
    pub url: String,
    pub package_html_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<PackageVersionMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageVersionMetadata {
    pub package_type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<ContainerMetadata>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub docker: Option<DockerMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackagesBillingUsage {
    /// Sum of the free and paid storage space (GB) for GitHuub Packages.
    pub total_gigabytes_bandwidth_used: i64,
    /// Total paid storage space (GB) for GitHuub Packages.
    pub total_paid_gigabytes_bandwidth_used: i64,
    /// Free storage space (GB) for GitHub Packages.
    pub included_gigabytes_bandwidth: i64,
}

/// The configuration for GitHub Pages for a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    /// The API address for accessing this Page resource.
    pub url: String,
    /// The status of the most recent build of the Page.
    pub status: String,
    /// The Pages site's custom domain
    pub cname: String,
    /// Whether the Page has a custom 404 page.
    pub custom_404: bool,
    /// The web address the Page can be accessed from.
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub source: Option<PagesSourceHash>,
    /// Whether the GitHub Pages site is publicly visible. If set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site.
    pub public: bool,
}

/// Page Build
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBuild {
    pub url: String,
    pub status: String,
    pub error: PagebuildError,
    pub pusher: SimpleUser,
    pub commit: String,
    pub duration: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Page Build Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBuildStatus {
    pub url: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagebuildError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PagesSourceHash {
    pub branch: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipationStats {
    pub all: Vec<i32>,
    pub owner: Vec<i32>,
}

/// Details of a deployment that is waiting for protection rules to pass
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingDeployment {
    pub environment: PendingdeploymentEnvironment,
    /// The set duration of the wait timer
    pub wait_timer: i64,
    /// The time that the wait timer began.
    pub wait_timer_started_at: chrono::DateTime<chrono::Utc>,
    /// Whether the currently authenticated user can approve the deployment
    pub current_user_can_approve: bool,
    /// The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
    pub reviewers: Vec<PendingdeploymentReviewers>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingdeploymentEnvironment {
    /// The id of the environment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    /// The name of the environment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingdeploymentReviewers {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<DeploymentReviewerType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reviewer: Option<Value>,
}

/// Porter Author
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PorterAuthor {
    pub id: i64,
    pub remote_id: String,
    pub remote_name: String,
    pub email: String,
    pub name: String,
    pub url: String,
    pub import_url: String,
}

/// Porter Large File
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PorterLargeFile {
    pub ref_name: String,
    pub path: String,
    pub oid: String,
    pub size: i64,
}

/// Private User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: bool,
    pub bio: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub private_gists: i64,
    pub total_private_repos: i64,
    pub owned_private_repos: i64,
    pub disk_usage: i64,
    pub collaborators: i64,
    pub two_factor_authentication: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<PrivateuserPlan>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub business_plus: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateuserPlan {
    pub collaborators: i64,
    pub name: String,
    pub space: i64,
    pub private_repos: i64,
}

/// Projects are a way to organize columns and cards of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub owner_url: String,
    pub url: String,
    pub html_url: String,
    pub columns_url: String,
    pub id: i64,
    pub node_id: String,
    /// Name of the project
    pub name: String,
    /// Body of the project
    pub body: String,
    pub number: i64,
    /// State of the project; either 'open' or 'closed'
    pub state: String,
    pub creator: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The baseline permission that all organization members have on this project. Only present if owner is an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,
    /// Whether or not this project can be seen by everyone. Only present if owner is an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
}

/// Project cards represent a scope of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectCard {
    pub url: String,
    /// The project card's ID
    pub id: i64,
    pub node_id: String,
    pub note: String,
    pub creator: SimpleUser,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether or not the card is archived
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,
    pub column_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_url: Option<String>,
    pub project_url: String,
}

/// Project columns contain cards of work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectColumn {
    pub url: String,
    pub project_url: String,
    pub cards_url: String,
    /// The unique identifier of the project column
    pub id: i64,
    pub node_id: String,
    /// Name of the project column
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Branch protections protect branches
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranch {
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_status_checks: Option<StatusCheckPolicy>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_pull_request_reviews: Option<ProtectedbranchRequiredPullRequestReviews>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_signatures: Option<ProtectedbranchRequiredSignatures>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub enforce_admins: Option<ProtectedbranchEnforceAdmins>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_linear_history: Option<ProtectedbranchRequiredLinearHistory>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_force_pushes: Option<ProtectedbranchRequiredLinearHistory>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_deletions: Option<ProtectedbranchRequiredLinearHistory>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrictions: Option<BranchRestrictionPolicy>,
}

/// Protected Branch Admin Enforced
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchAdminEnforced {
    pub url: String,
    pub enabled: bool,
}

/// Protected Branch Pull Request Review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedBranchPullRequestReview {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ProtectedbranchpullrequestreviewDismissalRestrictions>,
    pub dismiss_stale_reviews: bool,
    pub require_code_owner_reviews: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchEnforceAdmins {
    pub url: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredLinearHistory {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredPullRequestReviews {
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ProtectedbranchRequiredPullRequestReviewsDismissalRestrictions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredPullRequestReviewsDismissalRestrictions {
    pub url: String,
    pub users_url: String,
    pub teams_url: String,
    pub users: Vec<SimpleUser>,
    pub teams: Vec<Team>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchRequiredSignatures {
    pub url: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectedbranchpullrequestreviewDismissalRestrictions {
    /// The list of users with review dismissal access.
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<SimpleUser>>,
    /// The list of teams with review dismissal access.
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<Team>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub users_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,
}

/// Public User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: String,
    pub hireable: bool,
    pub bio: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub twitter_username: Option<String>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub plan: Option<PrivateuserPlan>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_private_repos: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owned_private_repos: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators: Option<i64>,
}

/// Pull requests let you tell others about changes you've pushed to a repository on GitHub. Once a pull request is sent, interested parties can review the set of changes, discuss potential modifications, and even push follow-up commits if necessary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    /// Number uniquely identifying the pull request within its repository.
    pub number: i64,
    /// State of this Pull Request. Either `open` or `closed`.
    pub state: String,
    pub locked: bool,
    /// The title of the pull request.
    pub title: String,
    pub user: SimpleUser,
    pub body: String,
    pub labels: Vec<PullrequestLabels>,
    pub milestone: Milestone,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    pub merged_at: chrono::DateTime<chrono::Utc>,
    pub merge_commit_sha: String,
    pub assignee: SimpleUser,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_teams: Option<Vec<TeamSimple>>,
    pub head: PullrequestHead,
    pub base: PullrequestBase,
    pub _links: PullrequestsimpleLinks,
    pub author_association: AuthorAssociation,
    pub auto_merge: AutoMerge,
    /// Indicates whether or not the pull request is a draft.
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
    pub merged: bool,
    pub mergeable: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: SimpleUser,
    pub comments: i64,
    pub review_comments: i64,
    /// Indicates whether maintainers can modify the pull request.
    pub maintainer_can_modify: bool,
    pub commits: i64,
    pub additions: i64,
    pub deletions: i64,
    pub changed_files: i64,
}

/// Pull Request Merge Result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMergeResult {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestMinimal {
    pub id: i64,
    pub number: i64,
    pub url: String,
    pub head: PullrequestminimalHead,
    pub base: PullrequestminimalHead,
}

/// Pull Request Reviews are reviews on pull requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReview {
    /// Unique identifier of the review
    pub id: i64,
    pub node_id: String,
    pub user: SimpleUser,
    /// The text of the review.
    pub body: String,
    pub state: String,
    pub html_url: String,
    pub pull_request_url: String,
    pub _links: PullrequestreviewLinks,
    #[serde(skip_serializing_if="Option::is_none")]
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A commit SHA for the review.
    pub commit_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    pub author_association: AuthorAssociation,
}

/// Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewComment {
    /// URL for the pull request review comment
    pub url: String,
    /// The ID of the pull request review to which the comment belongs.
    pub pull_request_review_id: i64,
    /// The ID of the pull request review comment.
    pub id: i64,
    /// The node ID of the pull request review comment.
    pub node_id: String,
    /// The diff of the line that the comment refers to.
    pub diff_hunk: String,
    /// The relative path of the file to which the comment applies.
    pub path: String,
    /// The line index in the diff to which the comment applies.
    pub position: i64,
    /// The index of the original line in the diff to which the comment applies.
    pub original_position: i64,
    /// The SHA of the commit to which the comment applies.
    pub commit_id: String,
    /// The SHA of the original commit to which the comment applies.
    pub original_commit_id: String,
    /// The comment ID to reply to.
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    pub user: SimpleUser,
    /// The text of the comment.
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// HTML URL for the pull request review comment.
    pub html_url: String,
    /// URL for the pull request that the review comment belongs to.
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    pub _links: PullrequestreviewcommentLinks,
    /// The first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,
    /// The first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_start_line: Option<i64>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_line: Option<i64>,
    /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
}

/// Pull Request Review Request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewRequest {
    pub users: Vec<SimpleUser>,
    pub teams: Vec<TeamSimple>,
}

/// Pull Request Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequestSimple {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: i64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: SimpleUser,
    pub body: String,
    pub labels: Vec<PullrequestsimpleLabels>,
    pub milestone: Milestone,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: chrono::DateTime<chrono::Utc>,
    pub merged_at: chrono::DateTime<chrono::Utc>,
    pub merge_commit_sha: String,
    pub assignee: SimpleUser,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requested_teams: Option<Vec<TeamSimple>>,
    pub head: PullrequestsimpleHead,
    pub base: PullrequestsimpleHead,
    pub _links: PullrequestsimpleLinks,
    pub author_association: AuthorAssociation,
    pub auto_merge: AutoMerge,
    /// Indicates whether or not the pull request is a draft.
    #[serde(skip_serializing_if="Option::is_none")]
    pub draft: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: PullrequestBaseRepo,
    pub sha: String,
    pub user: PullrequestHeadRepoOwner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestBaseRepo {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: String,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub notifications_url: String,
    pub owner: PullrequestHeadRepoOwner,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub forks: i64,
    pub forks_count: i64,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub homepage: String,
    pub language: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub mirror_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    pub license: LicenseSimple,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub size: i64,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub svn_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    pub watchers: i64,
    pub watchers_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: PullrequestHeadRepo,
    pub sha: String,
    pub user: PullrequestHeadRepoOwner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepo {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: String,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub notifications_url: String,
    pub owner: PullrequestHeadRepoOwner,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub forks: i64,
    pub forks_count: i64,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub homepage: String,
    pub language: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub mirror_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    pub license: PullrequestHeadRepoLicense,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub size: i64,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub svn_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    pub watchers: i64,
    pub watchers_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepoLicense {
    pub key: String,
    pub name: String,
    pub url: String,
    pub spdx_id: String,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestHeadRepoOwner {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestLabels {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestminimalHead {
    #[serde(rename = "ref")]
    pub _ref: String,
    pub sha: String,
    pub repo: PullrequestminimalHeadRepo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestminimalHeadRepo {
    pub id: i64,
    pub url: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewLinks {
    pub html: PullrequestreviewLinksHtml,
    pub pull_request: PullrequestreviewLinksHtml,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewLinksHtml {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinks {
    #[serde(rename = "self")]
    pub _self: PullrequestreviewcommentLinksSelf,
    pub html: PullrequestreviewcommentLinksHtml,
    pub pull_request: PullrequestreviewcommentLinksPullRequest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksHtml {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksPullRequest {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestreviewcommentLinksSelf {
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleLabels {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullrequestsimpleLinks {
    pub comments: Link,
    pub commits: Link,
    pub statuses: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comments: Link,
    pub review_comment: Link,
    #[serde(rename = "self")]
    pub _self: Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: i64,
    pub remaining: i64,
    pub reset: i64,
}

/// Rate Limit Overview
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitOverview {
    pub resources: RatelimitoverviewResources,
    pub rate: RateLimit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatelimitoverviewResources {
    pub core: RateLimit,
    #[serde(skip_serializing_if="Option::is_none")]
    pub graphql: Option<RateLimit>,
    pub search: RateLimit,
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_import: Option<RateLimit>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub integration_manifest: Option<RateLimit>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub code_scanning_upload: Option<RateLimit>,
}

/// Reactions to conversations provide a way to help people express their feelings more simply and effectively.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub id: i64,
    pub node_id: String,
    pub user: SimpleUser,
    /// The reaction to use
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionRollup {
    pub url: String,
    pub total_count: i64,
    #[serde(rename = "+1")]
    pub plus_1: i64,
    #[serde(rename = "-1")]
    pub minus_1: i64,
    pub laugh: i64,
    pub confused: i64,
    pub heart: i64,
    pub hooray: i64,
    pub eyes: i64,
    pub rocket: i64,
}

/// Referrer Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferrerTraffic {
    pub referrer: String,
    pub count: i64,
    pub uniques: i64,
}

/// A release.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: i64,
    pub node_id: String,
    /// The name of the tag.
    pub tag_name: String,
    /// Specifies the commitish value that determines where the Git tag is created from.
    pub target_commitish: String,
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body: Option<String>,
    /// true to create a draft (unpublished) release, false to create a published one.
    pub draft: bool,
    /// Whether to identify the release as a prerelease or a full release.
    pub prerelease: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub author: SimpleUser,
    pub assets: Vec<ReleaseAsset>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
}

/// Data related to a release.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub url: String,
    pub browser_download_url: String,
    pub id: i64,
    pub node_id: String,
    /// The file name of the asset.
    pub name: String,
    pub label: String,
    /// State of the release asset.
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uploader: SimpleUser,
}

/// Repo Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoSearchResultItem {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: SimpleUser,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub homepage: String,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: String,
    pub forks_count: i64,
    pub open_issues_count: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
    pub default_branch: String,
    pub score: i64,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    pub mirror_url: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub has_downloads: bool,
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    pub license: LicenseSimple,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<FullrepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
}

/// A git repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier of the repository
    pub id: i64,
    pub node_id: String,
    /// The name of the repository.
    pub name: String,
    pub full_name: String,
    pub license: LicenseSimple,
    pub forks: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    pub owner: SimpleUser,
    /// Whether the repository is private or public.
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: String,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub language: String,
    pub forks_count: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub size: i64,
    /// The default branch of the repository.
    pub default_branch: String,
    pub open_issues_count: i64,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Whether issues are enabled.
    pub has_issues: bool,
    /// Whether projects are enabled.
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    pub has_wiki: bool,
    pub has_pages: bool,
    /// Whether downloads are enabled.
    pub has_downloads: bool,
    /// Whether the repository is archived.
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<RepositoryTemplateRepository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    /// Whether to allow squash merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<String>,
}

/// Repository Collaborator Permission
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCollaboratorPermission {
    pub permission: String,
    pub user: SimpleUser,
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryInvitation {
    /// Unique identifier of the repository invitation.
    pub id: i64,
    pub repository: MinimalRepository,
    pub invitee: SimpleUser,
    pub inviter: SimpleUser,
    /// The permission associated with the invitation.
    pub permissions: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether or not the invitation has expired
    #[serde(skip_serializing_if="Option::is_none")]
    pub expired: Option<bool>,
    /// URL for the repository invitation
    pub url: String,
    pub html_url: String,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryPermissions {
    pub admin: bool,
    pub pull: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub triage: Option<bool>,
    pub push: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub maintain: Option<bool>,
}

/// Repository invitations let you manage who you collaborate with.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositorySubscription {
    /// Determines if notifications should be received from this repository.
    pub subscribed: bool,
    /// Determines if all notifications should be blocked from this repository.
    pub ignored: bool,
    pub reason: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub repository_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepository {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner: Option<RepositoryTemplateRepositoryOwner>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub fork: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub archive_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub assignees_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub blobs_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub collaborators_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub compare_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contents_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contributors_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deployments_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub downloads_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_commits_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_refs_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_tags_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub git_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_comment_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issue_events_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub keys_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub labels_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub languages_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub merges_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub milestones_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub notifications_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pulls_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub releases_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub statuses_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub trees_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub clone_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mirror_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub svn_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub forks_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub stargazers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub open_issues_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pushed_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryTemplateRepositoryPermissions>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepositoryOwner {
    #[serde(skip_serializing_if="Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub following_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub events_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub site_admin: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryTemplateRepositoryPermissions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub admin: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub push: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pull: Option<bool>,
}

/// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredPullRequestReviews {
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismissal_restrictions: Option<ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions>,
    /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
    #[serde(skip_serializing_if="Option::is_none")]
    pub dismiss_stale_reviews: Option<bool>,
    /// Blocks merging pull requests until [code owners](https://help.github.com/articles/about-code-owners/) review them.
    #[serde(skip_serializing_if="Option::is_none")]
    pub require_code_owner_reviews: Option<bool>,
    /// Specify the number of reviewers required to approve pull requests. Use a number between 1 and 6.
    #[serde(skip_serializing_if="Option::is_none")]
    pub required_approving_review_count: Option<i64>,
}

/// Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredPullRequestReviewsDismissalRestrictions {
    /// The list of user `login`s with dismissal access
    #[serde(skip_serializing_if="Option::is_none")]
    pub users: Option<Vec<String>>,
    /// The list of team `slug`s with dismissal access
    #[serde(skip_serializing_if="Option::is_none")]
    pub teams: Option<Vec<String>>,
}

/// Require status checks to pass before merging. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRequiredStatusChecks {
    /// Require branches to be up to date before merging.
    pub strict: bool,
    /// The list of status checks to require in order to merge into this branch
    pub contexts: Vec<String>,
}

/// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepobranchesbranchprotectionRestrictions {
    /// The list of user `login`s with push access
    pub users: Vec<String>,
    /// The list of team `slug`s with push access
    pub teams: Vec<String>,
    /// The list of app `slug`s with push access
    #[serde(skip_serializing_if="Option::is_none")]
    pub apps: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsActions {
    /// The text to be displayed on a button in the web UI. The maximum size is 20 characters.
    pub label: String,
    /// A short explanation of what this action would do. The maximum size is 40 characters.
    pub description: String,
    /// A reference for the action on the integrator's system. The maximum size is 20 characters.
    pub identifier: String,
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutput {
    /// The title of the check run.
    pub title: String,
    /// The summary of the check run. This parameter supports Markdown.
    pub summary: String,
    /// The details of the check run. This parameter supports Markdown.
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    /// Adds information from your analysis to specific lines of code. Annotations are visible on GitHub in the **Checks** and **Files changed** tab of the pull request. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about how you can view annotations on GitHub, see \"[About status checks](https://help.github.com/articles/about-status-checks#checks)\". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object) description for details about how to use this parameter.
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations: Option<Vec<ReposownerrepocheckrunsOutputAnnotations>>,
    /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#images-object) description for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<Vec<ReposownerrepocheckrunsOutputImages>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutputAnnotations {
    /// The path of the file to add an annotation to. For example, `assets/css/main.css`.
    pub path: String,
    /// The start line of the annotation.
    pub start_line: i64,
    /// The end line of the annotation.
    pub end_line: i64,
    /// The start column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_column: Option<i64>,
    /// The end column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_column: Option<i64>,
    /// The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
    pub annotation_level: String,
    /// A short description of the feedback for these lines of code. The maximum size is 64 KB.
    pub message: String,
    /// The title that represents the annotation. The maximum size is 255 characters.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// Details about this annotation. The maximum size is 64 KB.
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw_details: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunsOutputImages {
    /// The alternative text for the image.
    pub alt: String,
    /// The full URL of the image.
    pub image_url: String,
    /// A short image description.
    #[serde(skip_serializing_if="Option::is_none")]
    pub caption: Option<String>,
}

/// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocheckrunscheckRunIdOutput {
    /// **Required**.
    #[serde(skip_serializing_if="Option::is_none")]
    pub title: Option<String>,
    /// Can contain Markdown.
    pub summary: String,
    /// Can contain Markdown.
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,
    /// Adds information from your analysis to specific lines of code. Annotations are visible in GitHub's pull request UI. Annotations are visible in GitHub's pull request UI. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about annotations in the UI, see \"[About status checks](https://help.github.com/articles/about-status-checks#checks)\". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotations: Option<Vec<ReposownerrepocheckrunsOutputAnnotations>>,
    /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
    #[serde(skip_serializing_if="Option::is_none")]
    pub images: Option<Vec<ReposownerrepocheckrunsOutputImages>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepochecksuitespreferencesAutoTriggerChecks {
    /// The `id` of the GitHub App.
    pub app_id: i64,
    /// Set to `true` to enable automatic creation of CheckSuite events upon pushes to the repository, or `false` to disable them.
    pub setting: bool,
}

/// The author of the file. Default: The `committer` or the authenticated user if you omit `committer`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathAuthor {
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    pub name: String,
    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    pub email: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

/// object containing information about the author.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathAuthor1 {
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
}

/// The person that committed the file. Default: the authenticated user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathCommitter {
    /// The name of the author or committer of the commit. You'll receive a `422` status code if `name` is omitted.
    pub name: String,
    /// The email of the author or committer of the commit. You'll receive a `422` status code if `email` is omitted.
    pub email: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

/// object containing information about the committer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepocontentspathCommitter1 {
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepoenvironmentsenvironmentNameReviewers {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<DeploymentReviewerType>,
    /// The id of the user or team who can review the deployment
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
}

/// Information about the author of the commit. By default, the `author` will be the authenticated user and the current date. See the `author` and `committer` object below for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogitcommitsAuthor {
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

/// Information about the person who is making the commit. By default, `committer` will use the information set in `author`. See the `author` and `committer` object below for details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogitcommitsCommitter {
    /// The name of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The email of the author (or committer) of the commit
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// Indicates when this commit was authored (or committed). This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

/// An object with information about the individual creating the tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogittagsTagger {
    /// The name of the author of the tag
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The email of the author of the tag
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    /// When this object was tagged. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepogittreesTree {
    /// The file referenced in the tree.
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    /// The file mode; one of `100644` for file (blob), `100755` for executable (blob), `040000` for subdirectory (tree), `160000` for submodule (commit), or `120000` for a blob that specifies the path of a symlink.
    #[serde(skip_serializing_if="Option::is_none")]
    pub mode: Option<String>,
    /// Either `blob`, `tree`, or `commit`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    /// The SHA1 checksum ID of the object in the tree. Also called `tree.sha`. If the value is `null` then the file will be deleted.      **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(skip_serializing_if="Option::is_none")]
    pub sha: Option<String>,
    /// The content you want this file to have. GitHub will write this blob out and use that SHA for this entry. Use either this, or `tree.sha`.      **Note:** Use either `tree.sha` or `content` to specify the contents of the entry. Using both `tree.sha` and `content` will return an error.
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepohooksConfig {
    pub url: WebhookConfigUrl,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub digest: Option<String>,
}

/// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/repos#create-hook-config-params).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepohookshookIdConfig {
    pub url: WebhookConfigUrl,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub insecure_ssl: Option<WebhookConfigInsecureSsl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub room: Option<String>,
}

/// The source branch and directory used to publish your Pages site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepopagesSource {
    /// The repository branch used to publish your site's source files.
    pub branch: String,
    /// The repository directory that includes the source files for the Pages site. Allowed paths are `/` or `/docs`. Default: `/`
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReposownerrepopullspullNumberreviewsComments {
    /// The relative path to the file that necessitates a review comment.
    pub path: String,
    /// The position in the diff where you want to add a review comment. Note this value is not the same as the line number in the file. For help finding the position value, read the note below.
    #[serde(skip_serializing_if="Option::is_none")]
    pub position: Option<i64>,
    /// Text of the review comment.
    pub body: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,
}

/// Legacy Review Comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewComment {
    pub url: String,
    pub pull_request_review_id: i64,
    pub id: i64,
    pub node_id: String,
    pub diff_hunk: String,
    pub path: String,
    pub position: i64,
    pub original_position: i64,
    pub commit_id: String,
    pub original_commit_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    pub user: SimpleUser,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    pub _links: ReviewcommentLinks,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub body_html: Option<String>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub side: Option<String>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_side: Option<String>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub line: Option<i64>,
    /// The original line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_line: Option<i64>,
    /// The first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_line: Option<i64>,
    /// The original first line of the range for a multi-line comment.
    #[serde(skip_serializing_if="Option::is_none")]
    pub original_start_line: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewcommentLinks {
    #[serde(rename = "self")]
    pub _self: Link,
    pub html: Link,
    pub pull_request: Link,
}

/// A self hosted runner
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Runner {
    /// The id of the runner.
    pub id: i64,
    /// The name of the runner.
    pub name: String,
    /// The Operating System of the runner.
    pub os: String,
    /// The status of the runner.
    pub status: String,
    pub busy: bool,
    pub labels: Vec<RunnerLabels>,
}

/// Runner Application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerApplication {
    pub os: String,
    pub architecture: String,
    pub download_url: String,
    pub filename: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerGroupsEnterprise {
    pub id: f64,
    pub name: String,
    pub visibility: String,
    pub default: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_organizations_url: Option<String>,
    pub runners_url: String,
    pub allows_public_repositories: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerGroupsOrg {
    pub id: f64,
    pub name: String,
    pub visibility: String,
    pub default: bool,
    /// Link to the selected repositories resource for this runner group. Not present unless visibility was set to `selected`
    #[serde(skip_serializing_if="Option::is_none")]
    pub selected_repositories_url: Option<String>,
    pub runners_url: String,
    pub inherited: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub inherited_allows_public_repositories: Option<bool>,
    pub allows_public_repositories: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunnerLabels {
    /// Unique identifier of the label.
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    /// Name of the label.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    /// The type of label. Read-only labels are applied automatically when the runner is configured.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimEnterpriseGroup {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<ScimgrouplistenterpriseMembers>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimEnterpriseUser {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ScimuserlistenterpriseName>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<ScimenterpriseuserEmails>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,
}

/// Scim Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimError {
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i64>,
    #[serde(rename = "scimType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scim_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimGroupListEnterprise {
    pub schemas: Vec<String>,
    #[serde(rename = "totalResults")]
    pub total_results: f64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: f64,
    #[serde(rename = "startIndex")]
    pub start_index: f64,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimgrouplistenterpriseResources>,
}

/// SCIM /Users provisioning endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUser {
    /// SCIM schema used.
    pub schemas: Vec<String>,
    /// Unique identifier of an external identity
    pub id: String,
    /// The ID of the User.
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// Configured by the admin. Could be an email, login, or username
    #[serde(rename = "userName")]
    pub user_name: String,
    /// The name of the user, suitable for display to end-users
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    pub name: ScimuserName,
    /// user emails
    pub emails: Vec<ScimuserEmails>,
    /// The active status of the User.
    pub active: bool,
    pub meta: ScimuserMeta,
    /// The ID of the organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_id: Option<i64>,
    /// Set of operations to be performed
    #[serde(skip_serializing_if="Option::is_none")]
    pub operations: Option<Vec<ScimuserOperations>>,
    /// associated groups
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<ScimuserGroups>>,
}

/// SCIM User List
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUserList {
    /// SCIM schema used.
    pub schemas: Vec<String>,
    #[serde(rename = "totalResults")]
    pub total_results: i64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimUserListEnterprise {
    pub schemas: Vec<String>,
    #[serde(rename = "totalResults")]
    pub total_results: f64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: f64,
    #[serde(rename = "startIndex")]
    pub start_index: f64,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimuserlistenterpriseResources>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimenterpriseuserEmails {
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseMembers {
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _ref: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseMeta {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimgrouplistenterpriseResources {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub members: Option<Vec<ScimgrouplistenterpriseMembers>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserEmails {
    pub value: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserGroups {
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserMeta {
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserName {
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub formatted: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserOperations {
    pub op: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimuserlistenterpriseEmails {
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
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
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<ScimuserlistenterpriseName>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub emails: Option<Vec<ScimuserlistenterpriseEmails>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub groups: Option<Vec<Scimv2enterprisesenterpriseUsersGroups>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub meta: Option<ScimgrouplistenterpriseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseGroupsMembers {
    /// The SCIM user ID for a user.
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseGroupsscimGroupIdOperations {
    pub op: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersEmails {
    /// The email address.
    pub value: String,
    /// The type of email address.
    #[serde(rename = "type")]
    pub _type: String,
    /// Whether this email address is the primary address.
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersGroups {
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2enterprisesenterpriseUsersName {
    /// The first name of the user.
    #[serde(rename = "givenName")]
    pub given_name: String,
    /// The last name of the user.
    #[serde(rename = "familyName")]
    pub family_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersEmails {
    pub value: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersName {
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub formatted: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersscimUserIdEmails {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub _type: Option<String>,
    pub value: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scimv2organizationsorgUsersscimUserIdOperations {
    pub op: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScopedInstallation {
    pub permissions: AppPermissions,
    /// Describe whether all repositories have been selected or there's a selection involved
    pub repository_selection: String,
    pub single_file_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub single_file_paths: Option<Vec<String>>,
    pub repositories_url: String,
    pub account: SimpleUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultTextMatches {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchresulttextmatchesInner {
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub property: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub fragment: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub matches: Option<Vec<Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretScanningAlert {
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<AlertNumber>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<AlertCreatedAt>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<AlertUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub html_url: Option<AlertHtmlUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<SecretScanningAlertState>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolution: Option<SecretScanningAlertResolution>,
    /// The time that the alert was resolved in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`.
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub resolved_by: Option<SimpleUser>,
    /// The type of secret that secret scanning detected.
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret_type: Option<String>,
    /// The secret that was detected.
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
    pub github_owned_allowed: bool,
    /// Whether actions in GitHub Marketplace from verified creators are allowed. Set to `true` to allow all GitHub Marketplace actions by verified creators.
    pub verified_allowed: bool,
    /// Specifies a list of string-matching patterns to allow specific action(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/_*`.\"
    pub patterns_allowed: Vec<String>,
}

/// The API URL to use to get or set the actions that are allowed to run, when `allowed_actions` is set to `selected`.
pub type SelectedActionsUrl = String;

/// Short Blob
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortBlob {
    pub url: String,
    pub sha: String,
}

/// Short Branch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortBranch {
    pub name: String,
    pub commit: ShortbranchCommit,
    pub protected: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection: Option<BranchProtection>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub protection_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShortbranchCommit {
    pub sha: String,
    pub url: String,
}

/// Simple Commit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub author: SimplecommitAuthor,
    pub committer: SimplecommitAuthor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleCommitStatus {
    pub description: String,
    pub id: i64,
    pub node_id: String,
    pub state: String,
    pub context: String,
    pub target_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<bool>,
    pub avatar_url: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Simple User
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimpleUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub starred_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplecommitAuthor {
    pub name: String,
    pub email: String,
}

/// Stargazer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stargazer {
    pub starred_at: chrono::DateTime<chrono::Utc>,
    pub user: SimpleUser,
}

/// Starred Repository
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarredRepository {
    pub starred_at: chrono::DateTime<chrono::Utc>,
    pub repo: Repository,
}

/// The status of a commit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub url: String,
    pub avatar_url: String,
    pub id: i64,
    pub node_id: String,
    pub state: String,
    pub description: String,
    pub target_url: String,
    pub context: String,
    pub created_at: String,
    pub updated_at: String,
    pub creator: SimpleUser,
}

/// Status Check Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCheckPolicy {
    pub url: String,
    pub strict: bool,
    pub contexts: Vec<String>,
    pub contexts_url: String,
}

/// Tag
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub commit: ShortbranchCommit,
    pub zipball_url: String,
    pub tarball_url: String,
    pub node_id: String,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    pub permission: String,
    pub url: String,
    pub html_url: String,
    pub members_url: String,
    pub repositories_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<TeamSimple>,
}

/// A team discussion is a persistent record of a free-form conversation within a team.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamDiscussion {
    pub author: SimpleUser,
    /// The main text of the discussion.
    pub body: String,
    pub body_html: String,
    /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
    pub body_version: String,
    pub comments_count: i64,
    pub comments_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_edited_at: chrono::DateTime<chrono::Utc>,
    pub html_url: String,
    pub node_id: String,
    /// The unique sequence number of a team discussion.
    pub number: i64,
    /// Whether or not this discussion should be pinned for easy retrieval.
    pub pinned: bool,
    /// Whether or not this discussion should be restricted to team members and organization administrators.
    pub private: bool,
    pub team_url: String,
    /// The title of the discussion.
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}

/// A reply to a discussion within a team.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamDiscussionComment {
    pub author: SimpleUser,
    /// The main text of the comment.
    pub body: String,
    pub body_html: String,
    /// The current version of the body content. If provided, this update operation will be rejected if the given version does not match the latest version on the server.
    pub body_version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_edited_at: chrono::DateTime<chrono::Utc>,
    pub discussion_url: String,
    pub html_url: String,
    pub node_id: String,
    /// The unique sequence number of a team discussion comment.
    pub number: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamFull {
    /// Unique identifier of the team
    pub id: i64,
    pub node_id: String,
    /// URL for the team
    pub url: String,
    pub html_url: String,
    /// Name of the team
    pub name: String,
    pub slug: String,
    pub description: String,
    /// The level of privacy this team should have
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    /// Permission that the team will have for its repositories
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent: Option<TeamSimple>,
    pub members_count: i64,
    pub repos_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub organization: OrganizationFull,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,
}

/// Team Membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamMembership {
    pub url: String,
    /// The role of the user in the team.
    pub role: String,
    pub state: String,
}

/// A team's access to a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamProject {
    pub owner_url: String,
    pub url: String,
    pub html_url: String,
    pub columns_url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub body: String,
    pub number: i64,
    pub state: String,
    pub creator: SimpleUser,
    pub created_at: String,
    pub updated_at: String,
    /// The organization permission for this project. Only present when owner is an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization_permission: Option<String>,
    /// Whether the project is private or not. Only present when owner is an organization.
    #[serde(skip_serializing_if="Option::is_none")]
    pub private: Option<bool>,
    pub permissions: TeamprojectPermissions,
}

/// A team's access to a repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRepository {
    /// Unique identifier of the repository
    pub id: i64,
    pub node_id: String,
    /// The name of the repository.
    pub name: String,
    pub full_name: String,
    pub license: LicenseSimple,
    pub forks: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    pub owner: SimpleUser,
    /// Whether the repository is private or public.
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    pub mirror_url: String,
    pub hooks_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub language: String,
    pub forks_count: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub size: i64,
    /// The default branch of the repository.
    pub default_branch: String,
    pub open_issues_count: i64,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Whether issues are enabled.
    pub has_issues: bool,
    /// Whether projects are enabled.
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    pub has_wiki: bool,
    pub has_pages: bool,
    /// Whether downloads are enabled.
    pub has_downloads: bool,
    /// Whether the repository is archived.
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    #[serde(skip_serializing_if="Option::is_none")]
    pub visibility: Option<String>,
    pub pushed_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub template_repository: Option<Repository>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub temp_clone_token: Option<String>,
    /// Whether to allow squash merges for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub network_count: Option<i64>,
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub master_branch: Option<String>,
}

/// Groups of organization members that gives permissions on specified repositories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSimple {
    /// Unique identifier of the team
    pub id: i64,
    pub node_id: String,
    /// URL for the team
    pub url: String,
    pub members_url: String,
    /// Name of the team
    pub name: String,
    /// Description of the team
    pub description: String,
    /// Permission that the team will have for its repositories
    pub permission: String,
    /// The level of privacy this team should have
    #[serde(skip_serializing_if="Option::is_none")]
    pub privacy: Option<String>,
    pub html_url: String,
    pub repositories_url: String,
    pub slug: String,
    /// Distinguished Name (DN) that team maps to within LDAP environment
    #[serde(skip_serializing_if="Option::is_none")]
    pub ldap_dn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamprojectPermissions {
    pub read: bool,
    pub write: bool,
    pub admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamsteamIdteamsyncgroupmappingsGroups {
    /// ID of the IdP group.
    pub group_id: String,
    /// Name of the IdP group.
    pub group_name: String,
    /// Description of the IdP group.
    pub group_description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
}

/// Thread
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub repository: MinimalRepository,
    pub subject: ThreadSubject,
    pub reason: String,
    pub unread: bool,
    pub updated_at: String,
    pub last_read_at: String,
    pub url: String,
    pub subscription_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadSubject {
    pub title: String,
    pub url: String,
    pub latest_comment_url: String,
    #[serde(rename = "type")]
    pub _type: String,
}

/// Thread Subscription
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadSubscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub thread_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_url: Option<String>,
}

/// A topic aggregates entities that are related to a subject.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub names: Vec<String>,
}

/// Topic Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicSearchResultItem {
    pub name: String,
    pub display_name: String,
    pub short_description: String,
    pub description: String,
    pub created_by: String,
    pub released: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub featured: bool,
    pub curated: bool,
    pub score: i64,
    #[serde(skip_serializing_if="Option::is_none")]
    pub repository_count: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub related: Option<Vec<TopicsearchresultitemRelated>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub aliases: Option<Vec<TopicsearchresultitemRelated>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsearchresultitemRelated {
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_relation: Option<TopicsearchresultitemTopicRelation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicsearchresultitemTopicRelation {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub topic_id: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub relation_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Traffic {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub uniques: i64,
    pub count: i64,
}

/// User Marketplace Purchase
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMarketplacePurchase {
    pub billing_cycle: String,
    pub next_billing_date: chrono::DateTime<chrono::Utc>,
    pub unit_count: i64,
    pub on_free_trial: bool,
    pub free_trial_ends_on: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub account: MarketplaceAccount,
    pub plan: MarketplaceListingPlan,
}

/// User Search Result Item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSearchResultItem {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub score: i64,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub events_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_repos: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub public_gists: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub followers: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub following: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub location: Option<String>,
    pub site_admin: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub hireable: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub text_matches: Option<SearchResultTextMatches>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub blog: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Validation Error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub message: String,
    pub documentation_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<ValidationerrorErrors>>,
}

/// Validation Error Simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationErrorSimple {
    pub message: String,
    pub documentation_url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationerrorErrors {
    #[serde(skip_serializing_if="Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    pub code: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub payload: String,
    pub signature: String,
}

/// View Traffic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewTraffic {
    pub count: i64,
    pub uniques: i64,
    pub views: Vec<Traffic>,
}

/// The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days).
pub type WaitTimer = i32;

/// Configuration object of the webhook
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookConfig {
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<WebhookConfigUrl>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_type: Option<WebhookConfigContentType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub secret: Option<WebhookConfigSecret>,
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
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
    pub html_url: String,
    pub badge_url: String,
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
    pub id: i64,
    /// The name of the workflow run.
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub head_branch: String,
    /// The SHA of the head commit that points to the version of the worflow being run.
    pub head_sha: String,
    /// The auto incrementing run number for the workflow run.
    pub run_number: i64,
    pub event: String,
    pub status: String,
    pub conclusion: String,
    /// The ID of the parent workflow.
    pub workflow_id: i64,
    /// The URL to the workflow run.
    pub url: String,
    pub html_url: String,
    pub pull_requests: Vec<PullRequestMinimal>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The URL to the jobs for the workflow run.
    pub jobs_url: String,
    /// The URL to download the logs for the workflow run.
    pub logs_url: String,
    /// The URL to the associated check suite.
    pub check_suite_url: String,
    /// The URL to the artifacts for the workflow run.
    pub artifacts_url: String,
    /// The URL to cancel the workflow run.
    pub cancel_url: String,
    /// The URL to rerun the workflow run.
    pub rerun_url: String,
    /// The URL to the workflow.
    pub workflow_url: String,
    pub head_commit: SimpleCommit,
    pub repository: MinimalRepository,
    pub head_repository: MinimalRepository,
    #[serde(skip_serializing_if="Option::is_none")]
    pub head_repository_id: Option<i64>,
}

/// Workflow Run Usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunUsage {
    pub billable: WorkflowrunusageBillable,
    pub run_duration_ms: i64,
}

/// Workflow Usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowUsage {
    pub billable: WorkflowusageBillable,
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
    pub total_ms: i64,
    pub jobs: i64,
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
    #[serde(skip_serializing_if="Option::is_none")]
    pub total_ms: Option<i64>,
}
