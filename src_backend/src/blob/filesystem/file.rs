#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemFile {
    data: Vec<u8>,
}

const CODE_FORMAT: u8 = 100;       //'d'

impl FileSystemFile {

    pub fn from_blob(content: &[u8]) -> Result<FileSystemFile, ()> {

        if let Some((head, body)) = content.split_first() {
            if *head != CODE_FORMAT {
                return Err(());
            }

            let mut data: Vec<u8> = Vec::new();
            data.extend_from_slice(body);

            return Ok(FileSystemFile {
                data: data
            });
        }

        Err(())
    }

    pub fn to_data(self) -> Vec<u8> {
        self.data
    }
}
