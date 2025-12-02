use std::fmt::Display;
use std::fmt::Debug;
use std::usize;
#[derive(Debug)]
pub struct Matrix<T> {
    data   : Vec<T>,
    rows   : usize,
    cols   : usize,
} 

 pub trait BasicOps {
    type Item;
    pub fn add  (&self , other : &Matrix<Self::Item>);
    pub fn sub  (&self , other : &Matrix<Self::Item>);
    pub fn kmul    (&self , k: i32);
}


impl<T> BasicOps for Matrix<T>
where 

    {

    fn add  (&self , other : &Matrix<T>) {
        
    }
    fn sub  (&self , other : &Matrix<T>) {
        
    }
    fn kmul (&self , k: i32) {
        
    }

}


impl<T> Matrix<T>
where
    T: Copy + Debug + Display,
{
    /* Constructor Member Function */
    pub fn new(r : usize , c : usize, init : T) -> Self {
        Self {
                data: vec![init; r * c],
                rows: r,
                cols: c,
         }
    }
    fn get_idx(&self, r : usize , c : usize ) -> usize {
            let idx = r + self.rows * c;
            idx
    } 
    fn print_matrix (&self) {
        for i in 0..self.data.len(){
            if (i+1) % 4 == 0{
                println!(" ");
            }
            println!("{:?}",self.data[i]);
        }
    }
}

#[cfg(test)]

impl<T> Matrix<T>
where
    T: Copy + Debug + Display,
{
    pub fn test_new(r :usize, c : usize, init : T) -> Self {
        Self {
                data: vec![init; r * c],
                rows: r,
                cols: c,
         }
    }
    pub fn test_get_idx(&self, r : usize , c : usize ) -> usize {
            let idx = r * self.cols + c;
            idx
    } 



    pub fn test_print_matrix (&self) {
        for r in 0..self.rows{
            for c in 0..self.cols{
                let idx = self.test_get_idx(r, c);
                print!("{:?} ",self.data[idx]);
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let m = Matrix::test_new(4,34,0);  // <-- this only exists in tests
        m.test_print_matrix();
    }
}