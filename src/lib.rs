
use rand::prelude::*;

#[derive(Debug)]
pub struct CompanyName {
    company_name: String, 
}

impl CompanyName {
    //Randomly generated n-letter company name
    pub fn new(n: usize) -> Self {
        let mut rng = thread_rng();
        let mut random_chars: Vec<char> = vec!['a'; n];
        //ASCII range [97, 122] for low case English alphabet letters
        random_chars = random_chars.iter().map(|_e| rng.gen_range(97u8..=122) as char).collect();
        Self {
            company_name: random_chars.into_iter().collect::<String>(),
        }
    }

    //n - number of letters in the company name
    //m - how many names to generate
    pub fn generate_list(n: usize, m: usize) -> Vec<String> {
        let random_company_name_list: Vec<String> = vec!['a'.to_string(); m];
        random_company_name_list.iter().map(|_e| Self::new(n).company_name()).collect()
    }
    
    pub fn company_name(self: Self) -> String {
        self.company_name
    }
}

