use minio_rsc::client::Minio;
use minio_rsc::provider::StaticProvider;
use minio_rsc::errors::{Result};
use dotenv::dotenv;


pub fn get_test_minio() -> Minio  {
    dotenv().ok();
    let _minio_url: String = std::env::var("MINIO_URL").expect("DB_DSN must be set.");
    let minio_key: String = std::env::var("MINIO_KEY").expect("DB_DSN must be set.");
    let minio_secret: String = std::env::var("MINIO_SECRET").expect("DB_DSN must be set.");
    let provider = StaticProvider::new(minio_key, minio_secret, None);

    let minio = Minio::builder()
        .host("minio.sad.ms.gov.br")
        .provider(provider)
        .secure(false)
        //.build()
        .unwrap();

        minio

        //build(self) -> std::result::Result<Minio, ValueError> 
}


pub async fn list_buckets() -> Result<()> {
    let minio = get_test_minio();
    let (buckets, owner) = minio.list_buckets().await.unwrap();
    Ok(())
}