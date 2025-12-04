use ctrunner::{application::RaceService, infrastructure::db::Database};
use dotenv::dotenv;
use log::info;
use std::{fs, path::Path, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() {
    dotenv().ok();

    simple_logger::init_with_level(log::Level::Info).unwrap();

    if is_dev() {
        info!("🍕 Running in development mode...");
        copy_assets();
    }

    run_nightly_jobs().await;

    ctrunner::start().await;
}

async fn run_nightly_jobs() {
    let sched = JobScheduler::new().await.unwrap();
    let upcoming_races_job = Job::new_async("0 0 6 * * *", |_uuid, _l| {
        Box::pin(async move {
            info!("🦉 Gathering upcoming races...");
            let db = Database::initialize().await;
            let _ = RaceService::new(&Arc::new(db))
                .upcoming_races_nightly()
                .await;
            info!("Gathered some new races.");
        })
    })
    .unwrap();

    sched.add(upcoming_races_job).await.unwrap();
    sched.start().await.unwrap();
}

fn is_dev() -> bool {
    cfg!(debug_assertions)
}

/// Copy styles and scripts to .local versions if they are newer or don't exist
/// This is useful for development, so we can have unminified versions
/// that are not tracked by git and not used in production
fn copy_assets() {
    for dir in &["public/styles", "public/scripts"] {
        let path = Path::new(dir);
        if !path.is_dir() {
            continue;
        }
        info!("🤖 Processing files in {}...", dir);
        for entry in fs::read_dir(path).unwrap().flatten() {
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_string_lossy();
                if filename.contains(".local.") {
                    continue; // skip already copied files
                }
                let name = path.file_stem().unwrap().to_string_lossy();
                let ext = path.extension().unwrap_or_default().to_string_lossy();
                let target = path.with_file_name(format!("{}.local.{}", name, ext));

                // Copy only if target doesn't exist or source is newer
                let do_copy = !target.exists()
                    || path.metadata().unwrap().modified().unwrap()
                        > target
                            .metadata()
                            .unwrap_or_else(|_| path.metadata().unwrap())
                            .modified()
                            .unwrap();
                if do_copy {
                    fs::copy(&path, &target).unwrap();
                    info!("✅ Copied {} → {}", filename, target.display());
                }
            }
        }
    }
}
