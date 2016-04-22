//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html

//Autogenerated

use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

pub fn put_lang_id<'a,
               I: Into<Body<'a>>>(client: &'a mut Client, req: RequestParams,
                                  lang: &'a str, id: &'a str, body: I)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 1 + lang.len() + id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_scripts/");
    url_fmtd.push_str(lang);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res = client.put(&url_fmtd).headers(req.headers).body(body.into());
    res.send()
}
pub fn post_lang_id<'a,
                I: Into<Body<'a>>>(client: &'a mut Client, req: RequestParams,
                                   lang: &'a str, id: &'a str, body: I)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 1 + lang.len() + id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_scripts/");
    url_fmtd.push_str(lang);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res = client.post(&url_fmtd).headers(req.headers).body(body.into());
    res.send()
}
