use csv::Trim;
use serde::Deserialize;

fn parse_state_code(state_code: &str) -> Option<String> {
    let s = match state_code {
        "AZ" => "Arizona",
        "CA" => "California",
        "ID" => "Idaho",
        "IN" => "Indiana",
        "MA" => "Massachusetts",
        "OK" => "Oklahoma",
        "PA" => "Pennsylvania",
        "VA" => "Virginia",
        _ => {
            return None;
        }
    };
    Some(s.to_string())
}

#[derive(Deserialize, Debug, PartialEq)]
struct Record {
    name: String,
    street: String,
    address_state: String,
}

impl Record {
    fn new(name:&str, street:&str, address_state:&str) -> Self {
        Self{name:name.to_string(), street:street.to_string(), address_state:address_state.to_string()}
    }

    fn to_address(&self) -> Address {
        let elems: Vec<&str> = self.address_state.split(" ").collect();
        let sep = elems.len() - 1;
        let address = elems[0..sep].join(" ");
        let state = parse_state_code(elems[sep]).expect("parse state code");
        Address::new(&self.name, &self.street, &address, &state)
    }
}

#[derive(Debug, PartialEq)]
struct Address {
    name: String,
    street: String,
    address: String,
    state: String,
}

impl Address {
    fn new(name:&str, street:&str, address:&str, state:&str) -> Self {
        Self{name:name.to_string(), street:street.to_string(), address:address.to_string(), state:state.to_string()}
    }
}

fn solve(address_csv:&str) -> String {
    "".to_string()
}

fn parse_csv(address_csv:&str) -> Vec<Record> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).trim(csv::Trim::All).from_reader(address_csv.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize(){
        let record: Record = result.expect("deserialize");
        records.push(record);
    }
    records
}

fn convert_records_addresses(records:&[Record]) -> Vec<Address> {
    records.iter().map(|r| r.to_address()).collect()
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::{Address, Record};

    #[test]
    fn test_solve(){
        let input = file_as_string("test/address_book.csv");
        let want = file_as_string("test/address_book.txt");
        let got = super::solve(&input);
        assert_eq!(want,got);
    }

    #[test]
    fn parse_csv(){
        let input = file_as_string("test/address_book.csv");
        let want = vec![
            Record::new("John Daggett", "341 King Road", "Plymouth MA"),
            Record::new("Alice Ford", "22 East Broadway", "Richmond VA"),
            Record::new("Orville Thomas", "11345 Oak Bridge Road", "Tulsa OK"),
            Record::new("Terry Kalkas", "402 Lans Road", "Beaver Falls PA"),
            Record::new("Eric Adams", "20 Post Road", "Sudbury MA"),
            Record::new("Hubert Sims", "328A Brook Road", "Roanoke VA"),
            Record::new("Amy Wilde", "334 Bayshore Pkwy", "Mountain View CA"),
            Record::new("Sal Carpenter", "73 6th Street", "Boston MA"),
        ];
        let got = super::parse_csv(&input);
        assert!(equal_records(&want,&got));
    }

    #[test]
    fn convert_record_address(){
        let input = file_as_string("test/address_book.csv");
        let records = super::parse_csv(&input);
        let want = vec![
            Address::new("John Daggett", "341 King Road", "Plymouth", "Massachusetts"),
            Address::new("Alice Ford", "22 East Broadway", "Richmond", "Virginia"),
            Address::new("Orville Thomas", "11345 Oak Bridge Road", "Tulsa", "Oklahoma"),
            Address::new("Terry Kalkas", "402 Lans Road", "Beaver Falls", "Pennsylvania"),
            Address::new("Eric Adams", "20 Post Road", "Sudbury", "Massachusetts"),
            Address::new("Hubert Sims", "328A Brook Road", "Roanoke", "Virginia"),
            Address::new("Amy Wilde", "334 Bayshore Pkwy", "Mountain View", "California"),
            Address::new("Sal Carpenter", "73 6th Street", "Boston", "Massachusetts"),
        ];
        let got = super::convert_records_addresses(&records);
        assert!(equal_addresses(&want,&got));
    }

    fn file_as_string(file:&str) -> String {
        let mut f = std::fs::File::open(file).expect("open file");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("read to string");
        content
    }

    fn equal_records(xs:&[Record], ys:&[Record]) -> bool {
        if xs.len() != ys.len(){
            println!("length mismatch");
            return false;
        }
        for i in 0..xs.len(){
            if xs[i] != ys[i] {
                println!("left {:?} right {:?}", xs[i], ys[i]);
                return false;
            }
        }
        true
    }

    fn equal_addresses(xs:&[Address], ys:&[Address]) -> bool {
        if xs.len() != ys.len(){
            println!("length mismatch");
            return false;
        }
        for i in 0..xs.len(){
            if xs[i] != ys[i] {
                println!("left {:?} right {:?}", xs[i], ys[i]);
                return false;
            }
        }
        true
    }
}
