use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    #[cfg(test)]
    pub async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

}

#[cfg(test)]
mod test {

    use assert_json_diff::assert_json_eq;
    use async_graphql::*;
    use serde_json::json;

    #[async_std::test]
    async fn test_schema() -> std::io::Result<()> {
        let schema = Schema::new(super::Query, EmptyMutation, EmptySubscription);
        let res = schema.execute("{ add(a: 10, b: 20) }").await;
        assert_json_eq!(
            res,
            json!({
                "data": {
                    "add": 30
                }
            })
        );

        Ok(())
    }
}
