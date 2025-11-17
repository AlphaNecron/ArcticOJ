use poem::Request;
use std::collections::HashSet;

pub(crate) async fn extract(_req: &mut Request) -> poem::Result<HashSet<String>> {
    Ok(HashSet::new())
}
