use aws_sdk_s3::{Client, config::Region};
use aws_config::{self, profile::ProfileFileCredentialsProvider};
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
struct Config {
    profiles: Vec<String>,
    regions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_str = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read config.toml: {}", e);
            return Err(e.into());
        }
    };
    let config: Config = toml::from_str(&config_str)?;

    for region in &config.regions {
        for profile in &config.profiles {
            let credentials_provider = ProfileFileCredentialsProvider::builder().profile_name(profile).build();
            let aws_config_loader = aws_config::from_env()
                .credentials_provider(credentials_provider)
                .region(Region::new(region.clone()))
                .load()
                .await;

            let client = Client::new(&aws_config_loader);

            let bucket_list = client.list_buckets().send().await?;
            for bucket in bucket_list.buckets.unwrap_or_default() {
                let bucket_name = bucket.name.unwrap_or_else(|| "Unknown".to_string());

                match client.get_object_lock_configuration().bucket(&bucket_name).send().await {
                    Ok(resp) => {
                        if resp.object_lock_configuration.is_some() {
                            println!("Object Lock is enabled on bucket: {}", bucket_name);
                        }
                    }
                   Err(_) => {
                        println!("An error occurred while checking Object Lock for bucket: {}", bucket_name);
}
                }
            }
        }
    }

    Ok(())
}
