use std::env::{self, VarError};

use actix_multipart::Multipart;
use chrono::Utc;
use dotenvy::dotenv;
use futures_util::StreamExt;
use tokio::{fs, io::AsyncWriteExt};

use crate::models::File;

pub struct FileManager;

impl FileManager {
    fn get_filestore_dir() -> Result<String, VarError> {
        dotenv().ok();
        env::var("LOCAL_FILESTORE_DIR")
    }

    // Makes sure we have the filestore directory specified
    // by the env var "LOCAL_FILESTORE_DIR"
    async fn ensure_filestore_dir() -> () {
        if let Ok(filestore_dir) = Self::get_filestore_dir() {
            match fs::create_dir_all(&filestore_dir).await {
                Ok(_) => println!(
                    "Filestore directory created successfully or already exists at {}",
                    &filestore_dir
                ),
                Err(err) => panic!("Failed to create filestore directory: {}", err),
            }
        }
    }

    fn generate_file_url(
        organization_uuid: &uuid::Uuid,
        file_uuid: &uuid::Uuid,
        file_extension: &str,
    ) -> String {
        dotenv().ok();
        let domain =
            env::var("SERVER_DOMAIN").expect("Environment variable 'SERVER_DOMAIN' missing");
        format!(
            "http://{}/fileserver/{}/{}.{}",
            domain,
            organization_uuid.to_string(),
            file_uuid.to_string(),
            file_extension
        )
    }

    pub async fn create_files_from_multipart(
        organization_uuid: uuid::Uuid,
        mut payload: Multipart,
    ) -> Vec<File> {
        Self::ensure_filestore_dir().await;
        let filestore_dir = Self::get_filestore_dir().unwrap();

        let mut file_count = 0;
        let mut files: Vec<File> = Vec::new();

        while let Some(item) = payload.next().await {
            if file_count >= 3 {
                break;
            }

            let mut field = item.unwrap();
            let filename = field.content_disposition().get_filename().unwrap();
            let file_ext = format!("{}", filename.split(".").last().unwrap());

            let file_uuid = uuid::Uuid::new_v4();
            let file_url = Self::generate_file_url(&organization_uuid, &file_uuid, &file_ext);

            let new_file = File {
                id: file_uuid,
                url: file_url,
                name: filename.to_string(),
                created_at: Utc::now().naive_utc(),
                organization_id: organization_uuid,
            };

            fs::create_dir_all(format!(
                "{}/{}/",
                filestore_dir,
                organization_uuid.to_string()
            ))
            .await
            .expect("Error creating organization directory");

            let mut file = fs::File::create(format!(
                "{}/{}/{}.{}",
                filestore_dir,
                organization_uuid.to_string(),
                new_file.id,
                file_ext
            ))
            .await
            .expect("Error creating file");

            while let Some(chunk) = field.next().await {
                let _ = file.write_all(&chunk.unwrap()).await;
            }

            files.push(new_file);
            file_count += 1;
        }

        return files;

        // while let Some(item) = payload.next().await {
        //     let mut field = item.unwrap();
        //     let file = files.get(file_count).unwrap();

        //     fs::create_dir_all(format!("{}/{}/", filestore_dir, file.organization_id))
        //         .await
        //         .expect("Error creating organization directory");

        //     let mut file = fs::File::create(format!(
        //         "{}/{}/{}",
        //         filestore_dir, file.organization_id, file.id
        //     ))
        //     .await
        //     .expect("Error creating file");

        //     while let Some(chunk) = field.next().await {
        //         let _ = file.write_all(&chunk.unwrap()).await;
        //     }

        //     file_count += 1;
        // }
    }
}
