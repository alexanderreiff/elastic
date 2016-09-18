use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-reindex.html
pub fn post_task_id<'a,
                I: Into<Body<'a>>>(client: &'a mut Client,
                                   req: &'a RequestParams, task_id: &'a str,
                                   body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 18 + 12 + task_id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_delete_by_query/");
    url_fmtd.push_str(task_id);
    url_fmtd.push_str("/_rethrottle");
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

