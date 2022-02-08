use anyhow::Result;
use std::{io::Read, path::Path};
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};
use tracing::{error, info};

pub async fn pack_dir(dir: &str, name: &str) -> Result<String> {
    let path = Path::new(dir);
    path.join(name)
        .to_str()
        .ok_or(anyhow::format_err!("pack path failed:{},{}", dir, name))
        .map(|s| s.to_string())
}

pub async fn write_file(filepath: &str, data: &[u8]) -> Result<()> {
    let path = Path::new(filepath);
    create_dir_all(path.parent().unwrap()).await?;
    let f = match File::create(&path).await {
        Ok(f) => f,
        Err(e) => return Err(anyhow::anyhow!("create {:?} failed:{:?}", path, e)),
    };
    let mut writer = BufWriter::new(f);
    writer.write_all(data).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn read_file(filepath: &str) -> Result<Vec<u8>> {
    let path = Path::new(filepath);
    match File::open(&path).await {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut buf = vec![];
            match reader.read_to_end(&mut buf).await {
                Ok(len) => {
                    info!("read len {}", len);
                    Ok(buf)
                }
                Err(e) => {
                    error!("read error:{:?}", e);
                    Err(anyhow::format_err!(e))
                }
            }
        }
        Err(e) => {
            error!("failed to open file {:?}:{:?}", path, e);
            Err(anyhow::format_err!(e))
        }
    }
}
