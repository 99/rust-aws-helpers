use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_ec2::Client;
use aws_types::region::Region;
use prettytable::{Table, row};
use serde::Deserialize;
use std::error::Error;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct Config {
    profiles: Vec<String>,
    regions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut include_filter: Option<Vec<String>> = None;
    let mut exclude_filter: Option<Vec<String>> = None;

    for arg in args.iter().skip(1) {
        if let Some(stripped) = arg.strip_prefix("--include=") {
            include_filter = Some(stripped.split(',').map(String::from).collect());
        } else if let Some(stripped) = arg.strip_prefix("--exclude=") {
            exclude_filter = Some(stripped.split(',').map(String::from).collect());
        }
    }

    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    let mut instances_info = Vec::new(); 

    for region in &config.regions {
        for profile in &config.profiles {
        
            let loader = aws_config::from_env()
                .region(Region::new(region.clone()))
                .credentials_provider(ProfileFileCredentialsProvider::builder().profile_name(profile).build());
            let aws_config = loader.load().await;

            let client = Client::new(&aws_config);
            
            let resp = match client.describe_instances().send().await {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Failed to describe instances for profile {}: {:?}", profile, e);
                    continue;
                }
            };

            for reservation in resp.reservations.unwrap_or_default() {
                for instance in reservation.instances.unwrap_or_default() {
                    let instance_id_string = instance.instance_id.unwrap_or_default();
                    let instance_type_string = instance.instance_type.as_ref().map_or(String::from("Unknown"), |it| it.as_str().to_string());

                    if let Some(ref types) = include_filter {
                        if !types.iter().any(|type_str| instance_type_string == *type_str) {
                            continue;
                        }
                    }

                    if let Some(ref types) = exclude_filter {
                        if types.iter().any(|type_str| instance_type_string == *type_str) {
                            continue;
                        }
                    }

                    let tags_string = instance.tags.as_ref().map_or(String::from("None"), |tags| {
                        tags.iter()
                            .filter_map(|tag| {
                                if let (Some(key), Some(value)) = (&tag.key, &tag.value) {
                                    Some(format!("{}: {}", key, value))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    });

                    let created_string = instance.launch_time.as_ref().map_or(String::from("Unknown"), |dt| format!("{:?}", dt));

                    instances_info.push(InstanceInfo {
                        account: profile.to_string(),
                        instance_id: instance_id_string,
                        region: region.to_string(),
                        instance_type: instance_type_string,
                        tags: tags_string,
                        created: created_string,
                    });
                }
            }
        }
    }

    let mut table = Table::new();
    table.add_row(row!["Account", "Instance", "Type", "Region", "Created", "Tags"]);

    for info in instances_info {
        table.add_row(row![info.account, info.instance_id, info.instance_type, info.region, info.created, info.tags]);
    }

    table.printstd();

    Ok(())
}

struct InstanceInfo {
    account: String,
    instance_id: String,
    instance_type: String,
    region: String,
    tags: String,
    created: String,
}
