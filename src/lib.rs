use std::fmt;

#[derive(Debug)]
pub enum BitSetError {
    /// The element you are trying to access is not within the domain of the bitset
    OutOfRange,
    /// The element you are trying to access is not found in the bitset
    ElementNotFound
}

/// Bitset representation
pub struct BitSet {
    data: u128,
    capacity: u8
}

impl BitSet {
    /// Construct and return new `BitSet` struct
    ///
    /// # Arguments
    ///
    /// * `size` - The virutal size of the bitset
    pub fn new(size: u8) -> Self {
        let data: u128 = 0;
        BitSet {data, capacity: size}
    }

    /// Returns the max number of elements that can fit into the bitset
    pub fn capacity(&self) -> u8 {
        self.capacity
    }
    
    /// Returns the number of elements in bitset
    pub fn size(&self) -> u8 {
        // count how many 1s are in the binary representation of `self.data`
        let str = format!("{:0128b}", self.data);
        let chars = str.chars();

        let ones: u8 = chars.filter(|c| c == &'1').count().try_into().unwrap();

        ones
        
    }
    
    // is num inside the domain of the set
    fn is_valid(&self, num: &u8) -> Result<(), BitSetError> {
        let _num = num.to_owned();
        if _num >= 128u8 || _num >= self.capacity  {
            return Err(BitSetError::OutOfRange)
        }

        Ok(())
    }
    
    pub fn get_data(&self) -> &u128 {
        &self.data
    }
    
    /// Returns if an element is in the bitset
    ///
    /// # Arguments
    /// 
    /// * `num` - The element to check for inclusion
    pub fn has(&self, num: &u8) -> Result<bool, BitSetError> {
        self.is_valid(num)?;
        
        let mut _data = self.data.to_owned();
        let mask = 1u128;
        _data >>= num;
    
        Ok(_data & mask == 1u128)
    }

    /// Add an element to the bitset
    ///
    /// # Arguments
    ///
    /// * `num` - The element to add to the bitset
    pub fn add(&mut self, num: &u8) -> Result<(), BitSetError> {
        self.is_valid(num)?; 
        
        let mut mask = 1u128;
        mask <<= num;

        self.data |= mask;

        Ok(())
    }

    /// Remove an element from the bitset
    ///
    /// # Arguments
    ///
    /// * `num` - The element to remove from the bitset
    pub fn remove(&mut self, num: &u8) -> Result<(), BitSetError> {
        self.is_valid(num)?;
        if !self.has(num)? {
            return Err(BitSetError::ElementNotFound)
        }

        let mut mask = 1u128;
        mask <<= num;
        mask = !mask;

        self.data &= mask;

        Ok(())
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("{:0128b}", self.data);
        let start = 128 - self.capacity; 
        
        let out_str: String = str.chars().skip(usize::from(start)).collect();

        write!(f, "[{}]", out_str)
    }
}
