use serde::{Deserialize};
use serde_xml_rs::{from_str};

fn catalog(s: &str, article: &str) -> String {
    let products: Vec<Product> = from_str(s).expect("from XML");
    for p in products {
        if article == p.name {
            return format!("{} > prx: ${} qty: {}", article, p.price, p.quantity);
        }
    }
    panic!("failed to find product");
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename(deserialize = "prod"))]
struct Product {
    name: String,
    #[serde(rename(deserialize = "prx"))]
    price: f32,
    #[serde(rename(deserialize = "qty"))]
    quantity: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s() -> String {
        let a: Vec<&str> = vec!["<prod><name>drill</name><prx>99</prx><qty>5</qty></prod>",
                                "<prod><name>hammer</name><prx>10</prx><qty>50</qty></prod>",
                                "<prod><name>screwdriver</name><prx>5</prx><qty>51</qty></prod>",
                                "<prod><name>table saw</name><prx>1099.99</prx><qty>5</qty></prod>",
                                "<prod><name>saw</name><prx>9</prx><qty>10</qty></prod>",
                                "<prod><name>chair</name><prx>100</prx><qty>20</qty></prod>",
                                "<prod><name>fan</name><prx>50</prx><qty>8</qty></prod>",
                                "<prod><name>wire</name><prx>10.8</prx><qty>15</qty></prod>",
                                "<prod><name>battery</name><prx>150</prx><qty>12</qty></prod>",
                                "<prod><name>pallet</name><prx>10</prx><qty>50</qty></prod>",
                                "<prod><name>wheel</name><prx>8.80</prx><qty>32</qty></prod>",
                                "<prod><name>extractor</name><prx>105</prx><qty>17</qty></prod>",
                                "<prod><name>bumper</name><prx>150</prx><qty>3</qty></prod>",
                                "<prod><name>ladder</name><prx>112</prx><qty>12</qty></prod>",
                                "<prod><name>hoist</name><prx>13.80</prx><qty>32</qty></prod>",
                                "<prod><name>platform</name><prx>65</prx><qty>21</qty></prod>",
                                "<prod><name>car wheel</name><prx>505</prx><qty>7</qty></prod>",
                                "<prod><name>bicycle wheel</name><prx>150</prx><qty>11</qty></prod>",
                                "<prod><name>big hammer</name><prx>18</prx><qty>12</qty></prod>",
                                "<prod><name>saw for metal</name><prx>13.80</prx><qty>32</qty></prod>",
                                "<prod><name>wood pallet</name><prx>65</prx><qty>21</qty></prod>",
                                "<prod><name>circular fan</name><prx>80</prx><qty>8</qty></prod>",
                                "<prod><name>exhaust fan</name><prx>62</prx><qty>8</qty></prod>",
                                "<prod><name>window fan</name><prx>62</prx><qty>8</qty></prod>"];
        return a.join("\n\n");
    }

    fn dotest(s: &str, article: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        println!("article:{:?}", article);
        let ans = catalog(s, article);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let s = &s();
        dotest(s, "ladder" , "ladder > prx: $112 qty: 12");
        dotest(s, "ladder" , "ladder > prx: $112 qty: 12");

    }
}
