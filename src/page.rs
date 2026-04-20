pub const PAGE_SIZE: usize = 4096;
pub type PageId = u32;

pub struct Page{
    pub data: [u8; PAGE_SIZE]
}

impl Page{
pub fn new() -> Page{
    Page{
        data:[0; PAGE_SIZE]
    }
}
}

