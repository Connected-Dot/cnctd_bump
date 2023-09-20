use std::path::Path;
use anyhow::anyhow;
use cnctd_cargo::Cargo;
use cnctd_npm::NPM;

fn check_for_config_files() -> anyhow::Result<String> {
    if Path::new("Cargo.toml").exists() {
        return Ok(String::from("rust"));
    } else if Path::new("package.json").exists() {
        return Ok(String::from("npm"));
    } else {
        return Err(anyhow::anyhow!("No recognized config file found."));
    }
}

pub async fn bump_project(version_part: &str) -> anyhow::Result<()> {
    match &*check_for_config_files()? {
        "rust" => {
            Cargo::bump_version(version_part).await?;
            Ok(())
        }
        "npm" => {
            NPM::bump_version(version_part).await?;
            Ok(())
        }
        &_ => Err(anyhow!("project type not detected"))
    }
}