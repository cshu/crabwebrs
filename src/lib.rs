use crabrs::*;
use log::*;

pub fn log_resp_body_debug(resp: reqwest::blocking::Response) {
    if let Ok(bstr) = resp.text() {
        debug!("{}", bstr);
    }
}

pub fn easy_http_resp(
    rb: reqwest::blocking::RequestBuilder,
) -> CustRes<reqwest::blocking::Response> {
    let resp = rb.send()?;
    let st = resp.status();
    if !st.is_success() {
        warn!("{} {}", "Request failure", st);
        log_resp_body_debug(resp);
        return Err(CustomErr {});
    }
    Ok(resp)
}

pub fn easy_http(rb: reqwest::blocking::RequestBuilder) -> CustRes<String> {
    Ok(easy_http_resp(rb)?.text()?)
}

pub fn easy_http_copy_to<W: ?Sized>(
    rb: reqwest::blocking::RequestBuilder,
    w: &mut W,
) -> CustRes<u64>
where
    W: std::io::Write,
{
    let len = easy_http_resp(rb)?.copy_to(w)?;
    Ok(len)
}

pub fn easy_http_bytes(rb: reqwest::blocking::RequestBuilder) -> CustRes<Vec<u8>> {
    let bytes = easy_http_resp(rb)?.bytes()?;
    Ok(bytes.into())
}

pub fn easy_http_get(url: &str) -> CustRes<String> {
    let resp = reqwest::blocking::get(url)?;
    let st = resp.status();
    if !st.is_success() {
        warn!("{} {}", "Request failure", st);
        log_resp_body_debug(resp);
        return Err(CustomErr {});
    }
    Ok(resp.text()?)
}
