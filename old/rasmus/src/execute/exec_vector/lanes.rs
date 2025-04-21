pub fn to_lanes_8x16(vector: u128) -> [u8; 16] {
    vector.to_be_bytes()
}

pub fn to_lanes_16x8(vector: u128) -> [u16; 8] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u16; 8];

    for i in 0..arr.len() {
        arr[i] = u16::from_be_bytes([lanes[i * 2], lanes[i * 2 + 1]]);
    }

    arr
}

pub fn to_lanes_32x4(vector: u128) -> [u32; 4] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u32; 4];

    for i in 0..arr.len() {
        arr[i] = u32::from_be_bytes([
            lanes[i * 4],
            lanes[i * 4 + 1],
            lanes[i * 4 + 2],
            lanes[i * 4 + 3],
        ]);
    }

    arr
}

pub fn to_lanes_64x2(vector: u128) -> [u64; 2] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u64; 2];

    for i in 0..arr.len() {
        arr[i] = u64::from_be_bytes([
            lanes[i * 8],
            lanes[i * 8 + 1],
            lanes[i * 8 + 2],
            lanes[i * 8 + 3],
            lanes[i * 8 + 4],
            lanes[i * 8 + 5],
            lanes[i * 8 + 6],
            lanes[i * 8 + 7],
        ]);
    }

    arr
}

pub fn vec_from_lanes<T>(lanes: Vec<T>) -> u128
where
    T: From<u8>
        + ::std::fmt::Binary
        + Into<u128>
        + ::std::marker::Copy
        + ::std::ops::BitAnd<Output = T>
        + ::std::ops::Shl<Output = T>,
{
    let mut result = 0u128;
    let bits_num = u128::BITS as usize / lanes.len();

    for lane in lanes.iter().rev() {
        let mut mask: T = 1u8.into();
        for _ in 0..bits_num {
            let bit = *lane & mask;
            result |= bit.into();
            mask = mask << 1u8.into();
        }
        result = result.rotate_right(bits_num as u32);
    }

    result
}
