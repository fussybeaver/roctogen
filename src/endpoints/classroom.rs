//! Method, error and parameter types for the Classroom endpoint.
#![allow(
    clippy::all
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Classroom<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Classroom {
    Classroom { auth }
}

/// Errors for the [Get a classroom](Classroom::get_a_classroom_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomGetAClassroomError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get an assignment](Classroom::get_an_assignment_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomGetAnAssignmentError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get assignment grades](Classroom::get_assignment_grades_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomGetAssignmentGradesError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [List accepted assignments for an assignment](Classroom::list_accepted_assignments_for_an_assignment_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomListAcceptedAssignmentsForAnAssignmentError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [List assignments for a classroom](Classroom::list_assignments_for_a_classroom_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomListAssignmentsForAClassroomError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [List classrooms](Classroom::list_classrooms_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum ClassroomListClassroomsError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}


/// Query parameters for the [List accepted assignments for an assignment](Classroom::list_accepted_assignments_for_an_assignment_async()) endpoint.
#[derive(Default, Serialize)]
pub struct ClassroomListAcceptedAssignmentsForAnAssignmentParams {
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    page: Option<u16>, 
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    per_page: Option<u16>
}

impl ClassroomListAcceptedAssignmentsForAnAssignmentParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn page(self, page: u16) -> Self {
        Self {
            page: Some(page),
            per_page: self.per_page, 
        }
    }

    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn per_page(self, per_page: u16) -> Self {
        Self {
            page: self.page, 
            per_page: Some(per_page),
        }
    }
}

impl<'enc> From<&'enc PerPage> for ClassroomListAcceptedAssignmentsForAnAssignmentParams {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}
/// Query parameters for the [List assignments for a classroom](Classroom::list_assignments_for_a_classroom_async()) endpoint.
#[derive(Default, Serialize)]
pub struct ClassroomListAssignmentsForAClassroomParams {
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    page: Option<u16>, 
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    per_page: Option<u16>
}

impl ClassroomListAssignmentsForAClassroomParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn page(self, page: u16) -> Self {
        Self {
            page: Some(page),
            per_page: self.per_page, 
        }
    }

    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn per_page(self, per_page: u16) -> Self {
        Self {
            page: self.page, 
            per_page: Some(per_page),
        }
    }
}

impl<'enc> From<&'enc PerPage> for ClassroomListAssignmentsForAClassroomParams {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}
/// Query parameters for the [List classrooms](Classroom::list_classrooms_async()) endpoint.
#[derive(Default, Serialize)]
pub struct ClassroomListClassroomsParams {
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    page: Option<u16>, 
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    per_page: Option<u16>
}

impl ClassroomListClassroomsParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn page(self, page: u16) -> Self {
        Self {
            page: Some(page),
            per_page: self.per_page, 
        }
    }

    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn per_page(self, per_page: u16) -> Self {
        Self {
            page: self.page, 
            per_page: Some(per_page),
        }
    }
}

impl<'enc> From<&'enc PerPage> for ClassroomListClassroomsParams {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}

impl<'api> Classroom<'api> {
    /// ---
    ///
    /// # Get a classroom
    ///
    /// Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// [GitHub API docs for get_a_classroom](https://docs.github.com/rest/classroom/classroom#get-a-classroom)
    ///
    /// ---
    pub async fn get_a_classroom_async(&self, classroom_id: i32) -> Result<crate::models::Classroom, ClassroomGetAClassroomError> {

        let request_uri = format!("{}/classrooms/{}", super::GITHUB_BASE_API_URL, classroom_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAClassroomError::Status404(crate::adapters::to_json_async(github_response).await?)),
                code => Err(ClassroomGetAClassroomError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get a classroom
    ///
    /// Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// [GitHub API docs for get_a_classroom](https://docs.github.com/rest/classroom/classroom#get-a-classroom)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_a_classroom(&self, classroom_id: i32) -> Result<crate::models::Classroom, ClassroomGetAClassroomError> {

        let request_uri = format!("{}/classrooms/{}", super::GITHUB_BASE_API_URL, classroom_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAClassroomError::Status404(crate::adapters::to_json(github_response)?)),
                code => Err(ClassroomGetAClassroomError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get an assignment
    ///
    /// Gets a GitHub Classroom assignment. Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for get_an_assignment](https://docs.github.com/rest/classroom/classroom#get-an-assignment)
    ///
    /// ---
    pub async fn get_an_assignment_async(&self, assignment_id: i32) -> Result<ClassroomAssignment, ClassroomGetAnAssignmentError> {

        let request_uri = format!("{}/assignments/{}", super::GITHUB_BASE_API_URL, assignment_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAnAssignmentError::Status404(crate::adapters::to_json_async(github_response).await?)),
                code => Err(ClassroomGetAnAssignmentError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get an assignment
    ///
    /// Gets a GitHub Classroom assignment. Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for get_an_assignment](https://docs.github.com/rest/classroom/classroom#get-an-assignment)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_an_assignment(&self, assignment_id: i32) -> Result<ClassroomAssignment, ClassroomGetAnAssignmentError> {

        let request_uri = format!("{}/assignments/{}", super::GITHUB_BASE_API_URL, assignment_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAnAssignmentError::Status404(crate::adapters::to_json(github_response)?)),
                code => Err(ClassroomGetAnAssignmentError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get assignment grades
    ///
    /// Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for get_assignment_grades](https://docs.github.com/rest/classroom/classroom#get-assignment-grades)
    ///
    /// ---
    pub async fn get_assignment_grades_async(&self, assignment_id: i32) -> Result<Vec<ClassroomAssignmentGrade>, ClassroomGetAssignmentGradesError> {

        let request_uri = format!("{}/assignments/{}/grades", super::GITHUB_BASE_API_URL, assignment_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAssignmentGradesError::Status404(crate::adapters::to_json_async(github_response).await?)),
                code => Err(ClassroomGetAssignmentGradesError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get assignment grades
    ///
    /// Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for get_assignment_grades](https://docs.github.com/rest/classroom/classroom#get-assignment-grades)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_assignment_grades(&self, assignment_id: i32) -> Result<Vec<ClassroomAssignmentGrade>, ClassroomGetAssignmentGradesError> {

        let request_uri = format!("{}/assignments/{}/grades", super::GITHUB_BASE_API_URL, assignment_id);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                404 => Err(ClassroomGetAssignmentGradesError::Status404(crate::adapters::to_json(github_response)?)),
                code => Err(ClassroomGetAssignmentGradesError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List accepted assignments for an assignment
    ///
    /// Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for list_accepted_assignments_for_an_assignment](https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment)
    ///
    /// ---
    pub async fn list_accepted_assignments_for_an_assignment_async(&self, assignment_id: i32, query_params: Option<impl Into<ClassroomListAcceptedAssignmentsForAnAssignmentParams>>) -> Result<Vec<ClassroomAcceptedAssignment>, ClassroomListAcceptedAssignmentsForAnAssignmentError> {

        let mut request_uri = format!("{}/assignments/{}/accepted_assignments", super::GITHUB_BASE_API_URL, assignment_id);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListAcceptedAssignmentsForAnAssignmentError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List accepted assignments for an assignment
    ///
    /// Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// [GitHub API docs for list_accepted_assignments_for_an_assignment](https://docs.github.com/rest/classroom/classroom#list-accepted-assignments-for-an-assignment)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_accepted_assignments_for_an_assignment(&self, assignment_id: i32, query_params: Option<impl Into<ClassroomListAcceptedAssignmentsForAnAssignmentParams>>) -> Result<Vec<ClassroomAcceptedAssignment>, ClassroomListAcceptedAssignmentsForAnAssignmentError> {

        let mut request_uri = format!("{}/assignments/{}/accepted_assignments", super::GITHUB_BASE_API_URL, assignment_id);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: ClassroomListAcceptedAssignmentsForAnAssignmentParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListAcceptedAssignmentsForAnAssignmentError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List assignments for a classroom
    ///
    /// Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// [GitHub API docs for list_assignments_for_a_classroom](https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom)
    ///
    /// ---
    pub async fn list_assignments_for_a_classroom_async(&self, classroom_id: i32, query_params: Option<impl Into<ClassroomListAssignmentsForAClassroomParams>>) -> Result<Vec<SimpleClassroomAssignment>, ClassroomListAssignmentsForAClassroomError> {

        let mut request_uri = format!("{}/classrooms/{}/assignments", super::GITHUB_BASE_API_URL, classroom_id);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListAssignmentsForAClassroomError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List assignments for a classroom
    ///
    /// Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// [GitHub API docs for list_assignments_for_a_classroom](https://docs.github.com/rest/classroom/classroom#list-assignments-for-a-classroom)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_assignments_for_a_classroom(&self, classroom_id: i32, query_params: Option<impl Into<ClassroomListAssignmentsForAClassroomParams>>) -> Result<Vec<SimpleClassroomAssignment>, ClassroomListAssignmentsForAClassroomError> {

        let mut request_uri = format!("{}/classrooms/{}/assignments", super::GITHUB_BASE_API_URL, classroom_id);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: ClassroomListAssignmentsForAClassroomParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListAssignmentsForAClassroomError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List classrooms
    ///
    /// Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
    ///
    /// [GitHub API docs for list_classrooms](https://docs.github.com/rest/classroom/classroom#list-classrooms)
    ///
    /// ---
    pub async fn list_classrooms_async(&self, query_params: Option<impl Into<ClassroomListClassroomsParams>>) -> Result<Vec<SimpleClassroom>, ClassroomListClassroomsError> {

        let mut request_uri = format!("{}/classrooms", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListClassroomsError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # List classrooms
    ///
    /// Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
    ///
    /// [GitHub API docs for list_classrooms](https://docs.github.com/rest/classroom/classroom#list-classrooms)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_classrooms(&self, query_params: Option<impl Into<ClassroomListClassroomsParams>>) -> Result<Vec<SimpleClassroom>, ClassroomListClassroomsError> {

        let mut request_uri = format!("{}/classrooms", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: ClassroomListClassroomsParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(ClassroomListClassroomsError::Generic { code }),
            }
        }
    }

}
