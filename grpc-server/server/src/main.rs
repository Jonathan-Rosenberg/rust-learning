use tonic::{transport::Server, Request, Response, Status};
use differ::{DiffRequest, DiffResponse, S3GatewayConfig, differ_server::{Differ, DifferServer}};
use std::io::{self, Write};
use std::collections::HashMap;
use deltalake::builder::DeltaTableBuilder;
use deltalake::delta::{DeltaTable, DeltaTableError};

const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID";
const AWS_ENDPOINT_URL: &str = "AWS_ENDPOINT_URL";
const AWS_S3_ADDRESSING_STYLE: &str = "AWS_S3_ADDRESSING_STYLE";
const AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY";

pub mod differ {
    tonic::include_proto!("proto");
}

#[derive(Debug, Default)]
pub struct DifferService {}

#[tonic::async_trait]
impl Differ for DifferService {
  async fn diff(&self, request: Request<DiffRequest>) -> Result<Response<DiffResponse>, Status> {
    eprintln!("differ was called!!! [stderr]");
    let r = request.into_inner();
    let s3_gateway_config_req = r.gateway_config.unwrap();
    let s3_config_map = construct_storage_config(s3_gateway_config_req).await;
    eprintln!("s3_config:");
    for (key, value) in &s3_config_map {
        eprintln!("{key}: {value}");
    }
    let ps = match r.paths {
        Some(ps) => ps,
        None => return Ok(Response::new(differ::DiffResponse { resp: format!("error with paths").into_bytes() }))
    };
    let first_table_path: String = ps.first_table_path;
    let _second_table_path: String = ps.second_table_path;
    let delta1: DeltaTable = match create_table_with_config(s3_config_map, first_table_path).await {
        Ok(table) => table,
        Err(err) => return Ok(Response::new(differ::DiffResponse { resp: format!("Returned err {}", err).into_bytes() }))
    };
    // let ans: Vec<u8> = format!("Diff of table: {}\nand table: {}", r.first_table_path, r.second_table_path).into_bytes();
    let ans: Vec<u8> = format!("Delta Version: {}", delta1.version()).into_bytes();
    Ok(Response::new(differ::DiffResponse { resp: ans }))
  }
}

async fn construct_storage_config(config: S3GatewayConfig) -> HashMap<String, String> {
    let mut s3_config: HashMap<String, String> = HashMap::new();
    s3_config.insert(AWS_ACCESS_KEY_ID.to_string(), config.key);
    s3_config.insert(AWS_ENDPOINT_URL.to_string(), config.endpoint);
    s3_config.insert(AWS_S3_ADDRESSING_STYLE.to_string(), "path".to_string());
    s3_config.insert(AWS_SECRET_ACCESS_KEY.to_string(), config.secret);
    s3_config
}

async fn create_table_with_config(config: HashMap<String, String>, path: String) -> Result<DeltaTable, DeltaTableError> {
    eprintln!("Path: {path}");
    let builder = DeltaTableBuilder::from_uri(path)
        .with_storage_options(config);
    
    eprintln!("builder: {:?}", builder);
    builder.load().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "127.0.0.1:1234".parse().unwrap();
  let differ_service = DifferService::default();

  println!("1|1|tcp|127.0.0.1:1234|grpc");
  io::stdout().flush().unwrap();

  Server::builder().add_service(DifferServer::new(differ_service))
    .serve(address)
    .await?;

  Ok(())
}