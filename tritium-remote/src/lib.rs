pub struct Connection {
    pub address: String,
    pub open: bool,
}

pub fn connect(address: &str) -> Connection {

    Connection {
        address: String::from(address),
        open: true
    }
}





// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
