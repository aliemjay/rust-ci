fn closure_lifetimes() {
    let closure = |s: &str| &s[..];
    let output = Some("hello").map(closure);
}
