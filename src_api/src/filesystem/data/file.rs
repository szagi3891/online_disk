#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystemFile {
    data: Vec<u8>,
}

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
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(content);

        Ok(FileSystemFile {
            data: data
        })
    }

    pub fn ref_data<'a>(&'a self) -> &'a Vec<u8> {
        &self.data
    }
}
