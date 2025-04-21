pub trait BytesGetter<B> {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> B;
}

pub struct MemoryBytesGetter;

impl BytesGetter<[u8; 1]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> [u8; 1] {
        [data[offset]; 1]
    }
}

impl BytesGetter<[u8; 2]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> [u8; 2] {
        let mut arr = [0u8; 2];
        for b in 0..2 {
            arr[b] = data[b + offset];
        }
        arr
    }
}

impl BytesGetter<[u8; 4]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> [u8; 4] {
        let mut arr = [0u8; 4];
        for b in 0..4 {
            arr[b] = data[b + offset];
        }
        arr
    }
}

impl BytesGetter<[u8; 8]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> [u8; 8] {
        let mut arr = [0u8; 8];
        for b in 0..8 {
            arr[b] = data[b + offset];
        }
        arr
    }
}

impl BytesGetter<[u8; 16]> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> [u8; 16] {
        let mut arr = [0u8; 16];
        for b in 0..16 {
            arr[b] = data[b + offset];
        }
        arr
    }
}

impl BytesGetter<Vec<u8>> for MemoryBytesGetter {
    fn get_bytes(data: &Vec<u8>, offset: usize) -> Vec<u8> {
        let mut arr = [0u8; 16];
        for b in 0..16 {
            arr[b] = data[b + offset];
        }
        arr.to_vec()
    }
}

pub fn get_u8_bytes<T: AsRef<[u8]>>(data: T, offset: usize) -> [u8; 1] {
    [data.as_ref()[offset]; 1]
}

pub fn get_u16_bytes<T: AsRef<[u8]>>(data: T, offset: usize) -> [u8; 2] {
    let mut arr = [0u8; 2];
    for b in 0..2 {
        arr[b] = data.as_ref()[b + offset];
    }
    arr
}

pub fn get_u32_bytes<T: AsRef<[u8]>>(data: T, offset: usize) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for b in 0..4 {
        arr[b] = data.as_ref()[b + offset];
    }
    arr
}

pub fn get_u64_bytes<T: AsRef<[u8]>>(data: T, offset: usize) -> [u8; 8] {
    let mut arr = [0u8; 8];
    for b in 0..8 {
        arr[b] = data.as_ref()[b + offset];
    }
    arr
}

pub fn set_bytes<T: IntoIterator<Item = u8>>(data: &mut Vec<u8>, ea: usize, bytes: T) {
    for (i, byte) in bytes.into_iter().enumerate() {
        data[ea + i] = byte;
    }
}
