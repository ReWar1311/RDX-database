#[tokio::test]
async fn test_set_get() {
    use crate::{storage::Db, commands};
    let db = Db::new();

    commands::execute(vec!["SET".into(), "foo".into(), "bar".into()], &db).await;
    let result = commands::execute(vec!["GET".into(), "foo".into()], &db).await;

    assert!(result.contains("bar"));
}
