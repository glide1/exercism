use std::mem::replace;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    elements: Vec<Option<T>>,
    write_location: usize,
    read_location: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}


impl<T> CircularBuffer<T>
    where T: Sized + Default + Clone + std::fmt::Debug
{
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            elements: vec![None; size],
            write_location: 0,
            read_location: 0,
            size: size,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        println!("read: {:?}", self);
        let location = self.read_location % self.size;
        let item = replace(&mut self.elements[location], None);
        match item {
            Some(i) => {
                self.increase_read_location();
                Ok(i)
            }
            None => Err(Error::EmptyBuffer),
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), Error> {
        let location = self.write_location % self.size;
        match self.elements[location] {
            Some(_) => Err(Error::FullBuffer),
            None => {
                self.elements[location] = Some(item);
                self.increase_write_location();
                Ok(())
            }
        }
    }

    fn increase_read_location(&mut self) {
        self.read_location += 1
    }

    fn increase_write_location(&mut self) {
        self.write_location += 1
    }

    pub fn overwrite(&mut self, item: T) -> Result<(), Error> {
        let location = self.write_location % self.size;
        if let Some(_) = self.elements[location] {
            self.increase_read_location()
        }
        self.increase_write_location();
        self.elements[location] = Some(item);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.write_location = 0;
        self.read_location = 0;
        for i in 0..self.size {
            self.elements[i] = None
        }
    }
}

