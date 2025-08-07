use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PipelineConfig {
    repo_url: String,
    build_tool: String,
    deploy_script: String,
}

#[derive(Serialize, Deserialize)]
struct PipelineStatus {
    pipeline_id: String,
    status: String,
}

async fn create_pipeline(config: web::Json<PipelineConfig>) -> impl Responder {
    // Implement pipeline creation logic
    PipelineStatus {
        pipeline_id: " pipeline-123".to_string(),
        status: "created".to_string(),
    }
}

async fn get_pipeline(pipeline_id: web::Path<String>) -> impl Responder {
    // Implement pipeline retrieval logic
    PipelineStatus {
        pipeline_id: pipeline_id.to_string(),
        status: "running".to_string(),
    }
}

async fn update_pipeline(pipeline_id: web::Path<String>, config: web::Json<PipelineConfig>) -> impl Responder {
    // Implement pipeline update logic
    PipelineStatus {
        pipeline_id: pipeline_id.to_string(),
        status: "updated".to_string(),
    }
}

async fn delete_pipeline(pipeline_id: web::Path<String>) -> impl Responder {
    // Implement pipeline deletion logic
    "Pipeline deleted".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/pipelines").route(web::post().to(create_pipeline)))
            .service(web::resource("/pipelines/{pipeline_id}").route(web::get().to(get_pipeline)))
            .service(web::resource("/pipelines/{pipeline_id}").route(web::put().to(update_pipeline)))
            .service(web::resource("/pipelines/{pipeline_id}").route(web::delete().to(delete_pipeline)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}