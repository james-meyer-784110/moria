use regex::Regex;

use super::MetaData;

pub struct DynamicMap {
    map: Vec<Vec<Regex>>,
    meta: Vec<MetaData>,
}

impl DynamicMap {

    pub fn vec_contains(v: &Vec<Regex>, t: &str) -> bool {
        for reg in v {
            match reg.captures(t){
                Some(t) => return true,
                _ => ()
            }
        }
        false
    }

    pub fn get(&self, target: &str) -> Option<&MetaData> {
        self.meta.get(0)
    }

}

// struct UrlMap<'a> {
//     groups: Vec<String>,
//     origins: Vec<String>,
//     map: Vec<Vec<Either<String,Regex>>>,
//     search: Option<Vec<&'a str>>
// }
//
// impl <'a>UrlMap<'a> {
//
//     fn find_in_vec(vec: &Vec<Either<String,Regex>>, target: &str) -> Option<usize> {
//         let mut i: usize = 0;
//         for either in vec {
//             match either {
//                 // Either we can have a string of our url piece, like "/api" or "/users"
//                 Either::This(a) => {
//                     if a == target {
//                         i = i + 1;
//                         return Some(i)
//                     }
//                 },
//                 // Or we have a regex because it is an optional type
//                 Either::That(b) => {
//                     if b.captures(target).is_some() {
//                         i = i + 1;
//                         return Some(i)
//                     }
//                 },
//                 _ => ()
//             }
//         }
//         None
//     }
//
//     fn get<A,B>(&self, target: (usize,usize)) -> Either<A,B> {
//         let index_of = match self.map.get(target.0) {
//             Either::This(a) => Either::This(a),
//             _ => Either::None,
//         };
//         Either::None
//     }
//
//     fn find(&self, taget: Vec<&str>) -> Vec<(usize,usize)> {
//         let mut x =  0;
//         let mut at: Vec<(usize,usize)> = Vec::new();
//
//         for i in self.map.iter() {
//             match UrlMap::find_in_vec(i, taget.get(i).unwrap()) {
//                 Some(y) => at.push((x,y)),
//                 _ => (),
//             }
//         }
//         at
//     }
// }