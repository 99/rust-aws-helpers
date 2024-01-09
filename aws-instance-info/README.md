# AWS EC2 Instance Info Viewer

## Overview
The AWS EC2 Instance Info Viewer command-line tool designed to retrieve and display information about AWS EC2 instances across multiple profiles and regions.  It uses the AWS SDK for Rust to interact with AWS services and outputs the instance information in a tabular format.

## Features
Multiple AWS Profiles:  querying across a list of specified AWS profiles ( for example accross organization)
Region Specific: Fetches instance data from a specified AWS region.
Instance Filtering: Excludes/includes specific EC2 instance types (e.g., 'm5.xlarge' and 't2.micro ') from the output.


## Prerequisites
Rust Programming Environment: Ensure you have Rust installed on your system.
AWS Credentials: Set up your AWS credentials and configure profiles as needed.


## Configuration
- **config.toml** This file contains AWS profiles and regions for querying EC2 instances.
Set up profiles and regions in `config.toml` in the project root.

### Example
  ```toml
  profiles = ["profile1", "profile2"]
  regions = ["us-east-1", "us-west-2"]
  ```
  
- Replace profile1, profile2, etc., with your AWS profile names.
- Update regions as per your requirements.

## How to build and run

```bash
git clone git@github.com:99/rust-aws-helpers.git
cd aws-instance-info
cargo build
cargo run
```
Include additional instructions for using the `--include` or `--exclude` 

### Run without filters:
```bash 
cargo run
```
### Run with include filter (only include specified instance types):
```bash
cargo run -- --include=m5.large,t2.micro
```
### Run with exclude filter (exclude specified instance types):
```bash
cargo run -- --exclude=t3.medium
```

### Example
Here's an example of how the output will look:

+------------+-----------------+------------+-----------+-------------------+-------------+
| Account    | Instance        | Type       | Region    | Created           | Tags        |
+------------+-----------------+------------+-----------+-------------------+-------------+
| profile1   | i-abcdef12345   | m5.large   | us-east-1 | DateTime { seconds: 1667473025, subsecond_nanos: 0 }  | Tag1: Value |
| profile2   | i-12345abcdef   | t2.micro   | us-west-2 | DateTime { seconds: 1667473025, subsecond_nanos: 0 }  | Tag2: Value |
+------------+-----------------+------------+-----------+-------------------+-------------+
