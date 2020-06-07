pub trait MongoDocModel {
    fn collection_name() -> &'static str;
}
