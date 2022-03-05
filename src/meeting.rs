use std::cmp::Ordering;
use std::str::FromStr;

fn meeting(s: &str) -> String {
    let ss: Vec<&str> = s.split(";").collect();
    let mut guests: Vec<Guest> = Vec::new();
    for s in ss {
        let guest = Guest::from_str(s).expect("from str");
        guests.push(guest);
    }
    guests.sort_by(cmp_guest);
    let ss: Vec<String> = guests.iter().map(|g| g.to_string()).collect();
    ss.join("")
}

fn cmp_guest(gx:&Guest, gy:&Guest) -> Ordering {
    let c = gx.last_name.cmp(&gy.last_name);
    if c == Ordering::Equal {
        gx.first_name.cmp(&gy.first_name)
    } else {
        c
    }
}

struct Guest {
    first_name: String,
    last_name: String,
}

impl FromStr for Guest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<&str> = s.split(":").collect();
        let first_name = elems[0].to_uppercase();
        let last_name = elems[1].to_uppercase();
        Ok(Guest{first_name, last_name})
    }
}

impl std::string::ToString for Guest {
    fn to_string(&self) -> String {
        format!("({}, {})", self.last_name, self.first_name)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::str::FromStr;

    fn dotest(s: &str, exp: &str) -> () {
        let ans = meeting(s);
        assert_eq!(ans, exp);
    }

    #[test]
    fn basic_tests() {
        dotest("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
               "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
        dotest("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
               "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");
        dotest("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
               "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");

    }

    #[test]
    fn test_from_str(){
        let got = Guest::from_str("Alex:Arno").expect("from str").to_string();
        let want = "(ARNO, ALEX)";
        assert_eq!(want,got);
    }
}