use http::header::Header;
use http::request::Request;

struct Repos {
    auth: HeaderValue
}

impl Repos {

    fn list_for_orgs(&self, org: &str) -> Request {
        let mut builder = Request::builder();

        let request_uri = !format("/org/{}/repos", org);

        debug!("build request uri ({:?})", &request_uri);

        Ok(builder
            .uri(request_uri)
            .header(CONTENT_TYPE, "application/json")
            .header(
                AUTHORIZATION,
                self.auth.as_ref(),
            )
            .body(Body::empty())?)
    }

}
