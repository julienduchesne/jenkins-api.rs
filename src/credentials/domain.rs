/// Helper type to act on a job
#[derive(Debug)]
pub struct DomainName<'a>(pub &'a str);
impl<'a> From<&'a str> for DomainName<'a> {
    fn from(v: &'a str) -> DomainName<'a> {
        DomainName(v)
    }
}
impl<'a> From<&'a String> for DomainName<'a> {
    fn from(v: &'a String) -> DomainName<'a> {
        DomainName(v)
    }
}