#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemFile {
    data: Vec<u8>,
}

const CODE_FORMAT: u8 = 100;       //'d'

impl FileSystemFile {

    pub fn new_from_slice(data: &[u8]) -> FileSystemFile {
        let mut data_vec: Vec<u8> = Vec::new();
        data_vec.extend_from_slice(data);

        FileSystemFile {
            data: data_vec
        }
    }

    pub fn to_data(self) -> Vec<u8> {
        self.data
    }

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

    pub fn to_blob(&self) -> Vec<u8> {
        let mut out = vec!(CODE_FORMAT);
        out.extend_from_slice(&self.data);
        out
    }
}
