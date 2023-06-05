use std::env;

use actix_multipart::Multipart;
use chrono::Utc;
use futures_util::StreamExt;
use tokio::{fs, io::AsyncWriteExt};

use crate::models::File;

pub struct FileManager;

impl FileManager {
    // Makes sure we have the filestore directory specified
    // by the env var "LOCAL_FILESTORE_DIR"
    async fn ensure_filestore_dir_exists() -> () {
        let filestore_dir = env::var("LOCAL_FILESTORE_DIR").unwrap();

        match fs::create_dir_all(&filestore_dir).await {
            Ok(_) => println!(
                "Filestore directory created successfully or already exists at {}",
                filestore_dir
            ),
            Err(err) => panic!("Failed to create filestore directory: {}", err),
        }
    }

    fn generate_file_url(
        organization_uuid: &uuid::Uuid,
        file_uuid: &uuid::Uuid,
        file_extension: &str,
    ) -> String {
        let domain = env::var("SERVER_DOMAIN").unwrap();
        format!(
            "http://{}/{}/{}.{}",
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
        Self::ensure_filestore_dir_exists().await;
        let filestore_dir = env::var("LOCAL_FILESTORE_DIR").unwrap();
        let max_file_count = env::var("MAX_FILE_UPLOAD_COUNT")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut file_count = 0;
        let mut files: Vec<File> = Vec::new();

        while let Some(item) = payload.next().await {
            if file_count >= max_file_count {
                break;
            }

            let mut form_field = item.unwrap();
            let filename = form_field.content_disposition().get_filename();

            if let None = filename {
                continue;
            }

            let filename = filename.unwrap();
            let file_extension = filename.split(".").last();

            if let None = file_extension {
                continue;
            }

            let file_extension = file_extension.unwrap();
            let file_uuid = uuid::Uuid::new_v4();
            let file_url = Self::generate_file_url(&organization_uuid, &file_uuid, &file_extension);

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
                file_extension
            ))
            .await
            .expect("Error creating file");

            while let Some(chunk) = form_field.next().await {
                let _ = file.write_all(&chunk.unwrap()).await;
            }

            files.push(new_file);
            file_count += 1;
        }

        return files;
    }
}
