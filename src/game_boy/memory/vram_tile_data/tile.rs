pub struct Tile {
    data : [[u8; 2]; 8]
}

impl Tile {
    pub(crate) fn new() -> Tile {
        Tile {data: [[0; 2]; 8] }
    }

    pub fn from_vec(external_data: Vec<u8>) -> Tile {
        let mut tile = Tile::new();
        let mut idx = 0;
        for i in 0..tile.data.len() {
            for j in 0..tile.data[i].len() {
                tile.data[i][j] = external_data[idx];
                idx += 1;
            }
        }

        tile
    }

    pub fn to_array(&self) -> [[u8; 8]; 8] {
        let mut array = [[0; 8]; 8];

        for row_idx in 0..8 {
            let row = &self.data[row_idx];
            let first_byte = row[0];
            let second_byte = row[1];

            for bit_pos in 0..8 {
                let flag = 1 << (7 - bit_pos);
                if first_byte & flag != 0 && second_byte & flag != 0 {
                    array[row_idx][bit_pos] = 0b11;
                } else if first_byte & flag == 0 && second_byte & flag == 0 {
                    array[row_idx][bit_pos] = 0b00;
                } else if first_byte & flag == 0 && second_byte & flag != 0 {
                    array[row_idx][bit_pos] = 0b10;
                } else {
                    array[row_idx][bit_pos] = 0b01;
                }
            }
        }

        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tile() {
        let data = vec![
            0x3C, 0x7E,
            0x42, 0x42,
            0x42, 0x42,
            0x42, 0x42,
            0x7E, 0x5E,
            0x7E, 0x0A,
            0x7C, 0x56,
            0x38, 0x7C,
        ];

        let tile = Tile::from_vec(data);

        let expected = [
        [0b00, 0b10, 0b11, 0b11, 0b11, 0b11, 0b10, 0b00],
        [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
        [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
        [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
        [0b00, 0b11, 0b01, 0b11, 0b11, 0b11, 0b11, 0b00],
        [0b00, 0b01, 0b01, 0b01, 0b11, 0b01, 0b11, 0b00],
        [0b00, 0b11, 0b01, 0b11, 0b01, 0b11, 0b10, 0b00],
        [0b00, 0b10, 0b11, 0b11, 0b11, 0b10, 0b00, 0b00],
        ];

        assert_eq!(tile.to_array(), expected);
    }
}