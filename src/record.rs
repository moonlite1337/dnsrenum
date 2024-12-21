#[derive(Debug)]
pub struct Record {}

trait Resolver {
    fn new(h: String) -> Self;
    todo!("implement enum to target record type");
    fn resolve() -> Record;
}
