use std::collections::HashMap;


struct Solution{}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let x=i32::from_str_radix(&a, 2).unwrap();
        let y=i32::from_str_radix(&b, 2).unwrap();
        let result=format!("{:b}",x+y);
        result
    }

    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut ans=Vec::new();
        let mut word_dic=HashMap::new();
        let mut length=list1.len()+list2.len();
        for (i,word) in list1.iter().enumerate(){
            word_dic.insert(word, i);
        }
        for (i,word) in list2.iter().enumerate(){
            if word_dic.contains_key(word){
                if i+word_dic.get(word).unwrap()<length{
                    length=i+word_dic.get(word).unwrap();
                    ans.clear();
                    ans.push(word.clone());
                }else if i+word_dic.get(word).unwrap()==length {
                    ans.push(word.clone());
                }
            } 
        }
        ans
    }
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
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let result=match haystack.find(&needle) {
            Some(v)=>v as i32,
            None=>-1,
        };
        result
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
