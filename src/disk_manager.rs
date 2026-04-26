use std::fs::{File, OpenOptions};
use std::io::{Read,Write,Seek,SeekFrom};
use crate::page::{Page,PageId,PAGE_SIZE};

pub struct DiskManager{
    file: File,
    next_page_id:PageId
}

impl DiskManager {
    pub fn new(path:&str) -> std::io::Result<Self>{
        let file = OpenOptions :: new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let file_size = file.metadata()?.len();
        let next_page_id = (file_size / PAGE_SIZE as u64) as PageId;

        Ok(DiskManager { file, next_page_id })
    }
    pub fn allocate_page(&mut self) -> PageId{
        let id = self.next_page_id;
        self.next_page_id += 1;
        id
    }

    pub fn write_page(&mut self, page_id: PageId, page: &Page)-> std::io::Result<()>{
        let offset = page_id as u64 * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all((&page.data))?;
        self.file.flush()

    }

    pub fn read_page(&mut self, page_id: PageId, page: &mut Page) -> std::io::Result<()>{
        let offset = page_id as u64 & PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(&mut page.data)
    }
}
