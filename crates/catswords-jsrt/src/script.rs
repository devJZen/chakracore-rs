use crate::error::{ok, Result};
use crate::guard::Guard;
use crate::value::Value;
use catswords_jsrt_sys as sys;

fn to_wide_null(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub fn eval(_guard: &Guard<'_>, code: &str) -> Result<Value> {
    let script = to_wide_null(code);
    let url = to_wide_null("eval.js");

    let mut out: sys::JsValueRef = std::ptr::null_mut();
    unsafe {
        ok(sys::JsRunScript(
            script.as_ptr(),
            0 as sys::JsSourceContext,
            url.as_ptr(),
            &mut out,
        ))?;
    }
    Ok(Value { raw: out })
}
