use aws_config::load_from_env;
use aws_sdk_s3 as s3;

use crate::utils::constants::AWS_REGION as AWS_S3_REGION;

pub struct AWSS3 {
    pub s3_client: s3::Client,
}

impl AWSS3 {
    pub async fn new() -> Self {
        let myconfig = load_from_env().await;
        let s3_client = s3::Client::new(&myconfig);
        Self { s3_client }
    }

    pub fn create_byte_data(&self, data: &str) -> Vec<u8> {
        data.as_bytes().to_vec()
    }

    pub async fn create_bucket(&self, bucket_name: &str) -> Result<(), s3::Error> {
        let constraint = s3::types::BucketLocationConstraint::from(AWS_S3_REGION.as_str());
        let bucket_cfg = s3::types::CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        let create_result = self
            .s3_client
            .create_bucket()
            .create_bucket_configuration(bucket_cfg)
            .bucket(bucket_name)
            .send()
            .await;

        if create_result.is_ok() {
            println!("Bucket created successfully");
            println!("Create bucket response: {:?}", create_result);
        } else {
            println!("Error creating bucket");
            println!("Create bucket response: {:?}", create_result.err());
        }
        Ok(())
    }

    pub async fn list_buckets(&self) -> Result<(), s3::Error> {
        let resp = self.s3_client.list_buckets().send().await?;
        println!("Buckets: {:?}", resp.buckets);
        Ok(())
    }

    pub async fn put_object(
        &self,
        bucket_name: &str,
        object_key: &str,
        object_data: Vec<u8>,
    ) -> Result<(), s3::Error> {
        let primitive_byte_stream = s3::primitives::ByteStream::from(object_data);
        let put_result = self
            .s3_client
            .put_object()
            .bucket(bucket_name)
            .key(object_key)
            .body(primitive_byte_stream)
            .send()
            .await;

        if put_result.is_ok() {
            println!("Object uploaded successfully");
        } else {
            println!("Error uploading object");
            println!("Put object response: {:?}", put_result.err());
        }
        Ok(())
    }

    pub async fn get_object(&self, bucket_name: &str, object_key: &str) -> Result<(), s3::Error> {
        let get_result = self
            .s3_client
            .get_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await;

        if get_result.is_ok() {
            println!("Object retrieved successfully");
        } else {
            println!("Error retrieving object");
            println!("Get object response: {:?}", get_result.err());
        }
        Ok(())
    }

    pub async fn delete_object(
        &self,
        bucket_name: &str,
        object_key: &str,
    ) -> Result<(), s3::Error> {
        let delete_result = self
            .s3_client
            .delete_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await;

        if delete_result.is_ok() {
            println!("Object deleted successfully");
        } else {
            println!("Error deleting object");
            println!("Delete object response: {:?}", delete_result.err());
        }
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_bucket_operations() {
        let s3 = AWSS3::new().await;
        let bucket_name = "rust-aws-s3-integration-test-bucket";

        // Test Create Bucket
        s3.create_bucket(bucket_name)
            .await
            .expect("Failed to create bucket");
        println!("Bucket created: {}", bucket_name);

        // Test List Buckets
        s3.list_buckets().await.expect("Failed to list buckets");

        // Test Put Object
        let object_key = "test_object.txt";
        let data = b"Hello AWS S3 from Rust!".to_vec();
        s3.put_object(bucket_name, object_key, data)
            .await
            .expect("Failed to put object");
        println!(
            "Object '{}' uploaded to bucket '{}'",
            object_key, bucket_name
        );

        // Test Get Object
        s3.get_object(bucket_name, object_key)
            .await
            .expect("Failed to get object");

        // Cleanup: Delete object and bucket after tests (not shown here, implement as needed)
    }

    #[tokio::test]
    #[ignore]
    async fn test_object_operations() {
        let s3 = AWSS3::new().await;
        let bucket_name = "rust-aws-s3-integration-test-bucket";
        let object_key = "another_test_object.txt";
        let content = b"Another test file content";

        // Ensure bucket exists
        s3.create_bucket(bucket_name).await.ok();

        // Test uploading an object
        s3.put_object(bucket_name, object_key, content.to_vec())
            .await
            .expect("Failed to upload object");

        // Test retrieving the object
        s3.get_object(bucket_name, object_key)
            .await
            .expect("Failed to retrieve object");

        // Cleanup: Remove object and bucket
    }
}
