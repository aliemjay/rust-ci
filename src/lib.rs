trait Tr<'a> {}
impl<'a, T: 'a> Tr<'a> for T {}

fn check() where for<'a, 'b> &'a &'b (): Tr<'a>, {}
fn main() { check() }
