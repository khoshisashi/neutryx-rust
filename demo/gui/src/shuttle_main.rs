//! Shuttle deployment entry point for FrictionalBank Web Dashboard.
//!
//! Deploy: cd demo/gui && shuttle deploy
//! Local:  cd demo/gui && shuttle run

mod shuttle_web;

use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    println!();
    println!("  FrictionalBank Web Dashboard (Shuttle)");
    println!("  =======================================");
    println!();
    println!("  API Endpoints:");
    println!("    GET  /api/health     - Health check");
    println!("    GET  /api/portfolio  - Portfolio data");
    println!("    POST /api/portfolio  - Price portfolio");
    println!("    GET  /api/exposure   - Exposure metrics");
    println!("    GET  /api/risk       - Risk metrics");
    println!();

    let router = shuttle_web::build_router();
    Ok(router.into())
}
