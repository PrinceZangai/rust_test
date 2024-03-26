use std::collections::HashMap;


struct Solution{}

impl Solution {
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
}