use std::collections::{HashMap, HashSet};


fn main() {
    
}

struct Solution{
    
}

// impl Solution {
//     pub fn find_words(words: Vec<String>) -> Vec<String> {
//         // fn foo(){};
//         let line1="qwertyuiop".chars().collect::<HashSet<_>>();
//         let line2="asdfghjkl".chars().collect::<HashSet<_>>();
//         let line3="zxcvbnm".chars().collect::<HashSet<_>>();
//         let mut results=Vec::new();
//         for word in words{
//             let t=word.to_lowercase().chars().collect::<HashSet<_>>();
//             println!("{:?}",t);
//             println!("{:?}",line1);
//             if t.is_subset(&line1) || t.is_subset(&line2) ||t.is_subset(&line3){
//                 results.push(word);
//             }
//         }
//         results
//     }
// }

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut result=&String::new();
        // println!("{}",result);
        fn count_word(word:&String)->HashMap<char,i32>{
            let mut counter: HashMap<char, i32>=HashMap::new();
            for c in word.chars().filter(|&x| x.is_alphabetic()).map(|x|x.to_lowercase().next().unwrap()){
                let count=counter.entry(c).or_insert(0);
                *count+=1;
            };
            counter
        };
        let licens_counter=count_word(&license_plate);
        for word in &words{
            let word_counter=count_word(word);
            let mut flag=0;
            for (k,v) in licens_counter.iter(){
                if !word_counter.contains_key(k) || word_counter.get(k).unwrap()<v{
                    flag=1;
                    break
                }
            }
            if flag==0{
                if result.len()==0 || word.len()< result.len(){
                    result=word;
                }
            }
        }
        result.clone()
    }
}

#[cfg(test)]
mod tests{
    use crate::Solution;
    // #[test]
    // fn test1(){
    //     let words=vec!["qwee".to_string()];
    //     let result=Solution::find_words(words);
    //     println!("{:?}",result);
    // }

    #[test]
    fn test2(){
        // let x='a';
        // let y=x.to_lowercase().to_string().get(0);
        let words=["step".to_string(),"steps".to_string(),"stripe".to_string(),"stepple".to_string()].to_vec();
        // println!("{:?}",words);
        // println!("{}",words.get(0).unwrap());
        let result=Solution::shortest_completing_word(String::from("1s3 PSt"), words);
        println!("{}",result);
    }
}

