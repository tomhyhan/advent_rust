use std::{collections::{HashSet, HashMap}, hash::{Hash, Hasher}};

use advent_2017::{Runner, get_file};

pub struct Q4 {

}

#[derive(Debug, PartialEq, Eq)]
struct Anagram {
    chars : HashMap<char, i32>
}

impl Hash for Anagram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut list : Vec<_> = self.chars.iter().map(|pair| (pair)).collect();
        list.sort();
        list.iter().for_each(|pair| pair.hash(state));
    }
}

impl Anagram {
    fn new(word:String) -> Self {
        let mut chars = HashMap::new();
        word.chars().for_each(|c|{ chars.entry(c).and_modify(|cnt| *cnt +=1).or_insert(1); });
        Anagram {chars}
    }
}


struct PassPhrases {
    words: Vec<Vec<String>>
}


impl PassPhrases {
    fn new() -> PassPhrases {
        let contents = get_file("q4.txt").unwrap();
        let words : Vec<Vec<_>>= contents.lines().map(|words|words.split_whitespace().map(String::from).collect()).collect();
        PassPhrases { words }
    }

    fn cnt_valid_phrases(&self) -> i32{
        let mut total = 0;
        self.words.iter().for_each(|phrase|{
            let mut valids_phrases = HashSet::new();
            phrase.iter().for_each(|p| { valids_phrases.insert(p); });
            if valids_phrases.len() == phrase.len() {
                total += 1;
            }
        } );
        total
    }

    fn cnt_valid_anagram_phrases(&self) -> i32 {
        let mut total = 0;
        self.words.iter().for_each(|phrase|{
            let mut valids_phrases = HashSet::new();
            phrase.iter().for_each(|p| { valids_phrases.insert(Anagram::new(p.clone())); });
            if valids_phrases.len() == phrase.len() {
                total += 1;
            }
        } );
        total
    }
}

impl Q4 {
    pub fn new() -> Self {
        Q4 {}
    }

    fn part1(&mut self){
        let passphrases = PassPhrases::new();
        println!("{:?}", passphrases.cnt_valid_phrases());
    }

    fn part2(&mut self) {
        let passphrases = PassPhrases::new();
        println!("{:?}", passphrases.cnt_valid_anagram_phrases())
    }
}

impl Runner for Q4 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn get_correct_anagram()  {
        let anagram = Anagram::new("abcd".to_string());
        println!("{anagram:?}");
        assert_eq!(1,1)
    }

    #[test]
    fn two_anagrams_matches() {
        let a = Anagram::new("abcde".to_string());
        let b = Anagram::new("abcde".to_string());
        let mut set = HashSet::new();
        set.insert(a);
        assert_eq!(set.contains(&b),true);
    }    
}