## A library to generate randomly a name for a company  

### Features: 
1. Generates company names using low case English letters;  
1. Allows to set number of letters in the company name, e.g. 5; 
1. Allows to generate a list of random company names
   
### struct CompanyName methods: 
1. pub fn new(n: usize) -> Self //constructs a struct CompanyName, which contains a private member company_name: String of n randomly generated low case English letters;  
1. pub fn company_name(self: Self) -> String //provides an access to the struct CompanyName private member company_name: String 
1. pub fn generate_list(n: usize, m: usize) -> Vec<String> //generates a list of m random n-letter company names

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    companyname = {git = "https://github.com/azavgo/companyname", branch = "main"}
```
2. Generate a 4-letter company name:  
```Rust
    use companyname::CompanyName;

    fn main() {
        let company = CompanyName::new(4);
        println!("Random company name: {}", company.company_name());
    }
```
3. Generate a list of 10 random 5-letter company names:  
```Rust
    use companyname::CompanyName;

    fn main() {
        let list = CompanyName::generate_list(5, 10);
        println!("Random company name list: {:#?}", &list);
    }
```
