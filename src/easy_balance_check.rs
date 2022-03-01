use anyhow::Result;

fn parse_book(content: &str) -> Result<Book> {
    let lines: Vec<&str> = content.split("\n").collect();
    let original_balance: f32 = lines[0].parse()?;
    let mut records = Vec::new();
    for line in &lines[1..] {
        println!("parsing {}", line);
        let elems: Vec<&str> = line.split(",").collect();
        let check_number = elems[0].parse()?;
        let category = elems[1];
        let amount = elems[2].parse()?;
        let record = Record::new(check_number, category, amount);
        records.push(record);
    }
    Ok(Book::new(original_balance, records))
}

struct Record {
    check_number: u32,
    category: String,
    amount: f32,
}

impl Record {
    fn new(check_number:u32, category:&str, amount:f32) -> Self {
        Record{check_number, category:category.to_string(), amount}
    }
}

struct Book {
    original_balance: f32,
    records: Vec<Record>,
}

impl Book {
    fn new(original_balance:f32, records:Vec<Record>) -> Self {
        Book {original_balance, records}
    }

    fn make_report(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    #[test]
    fn test0() {
        let mut f = std::fs::File::open("./test/book0.csv").expect("read file");
        let mut book_content = String::new();
        f.read_to_string(&mut book_content);
        //println!("book_content {}", book_content);
        let book = super::parse_book(&book_content).expect("parse book");
        let mut f = std::fs::File::open("./test/report0.txt").expect("read file");
        let mut want = String::new();
        f.read_to_string(&mut want);
        let got = book.make_report();
        assert_eq!(want, got);
    }
}
