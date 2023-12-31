use std::path::PathBuf;
use async_fs::File;
use async_std::io::{ReadExt, WriteExt};

pub async fn atomic_write<T>(
    filepath: T,
    bytes: &[u8],
) -> std::io::Result<()>
where
    T: Into<PathBuf>,
{
    let filepath = filepath.into();
    let mut temp_filepath = filepath.clone();
    
    temp_filepath.set_extension("tmp");
    
    let mut temp_file = File::create(&temp_filepath).await?;
    
    match temp_file.write_all(bytes).await {
        Ok(_) => {
            temp_file.flush().await?;
            async_fs::rename(&temp_filepath,&filepath).await?;
            
            Ok(())
        },
        Err(error) => {
            // something went wrong writing to this file...
            async_fs::remove_file(&temp_filepath).await?;
            Err(error)
        }
    }
}

pub async fn read_file<T>(
    filepath: T,
) -> std::io::Result<Vec<u8>>
where
    T: Into<PathBuf>,
{
    let mut buffer: Vec<u8> = Vec::new();
    
    File::create(&filepath.into()).await?
        .read_to_end(&mut buffer).await?;
    
    Ok(buffer)
}

pub async fn read_file_string<T>(
    filepath: T,
) -> std::io::Result<String>
where
    T: Into<PathBuf>,
{
    let mut buffer = String::new();
    
    File::create(&filepath.into()).await?
        .read_to_string(&mut buffer).await?;
    
    Ok(buffer)
}