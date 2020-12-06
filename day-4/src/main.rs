use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Passport {
    hcl: String,
    hgt: String,
    ecl: String,
    pid: String,
    cid: Option<u32>,
    byr: u32,
    iyr: u32,
    eyr: u32,
}

#[derive(Default, Debug)]
struct PassportBuilder {
    hcl: Option<String>,
    hgt: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u32>,
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
}

impl PassportBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn put(&mut self, field: &str) {
        let mut split = field.split(':');
        let key = split.next().unwrap();
        let value = split.next().unwrap();

        match key {
            "byr" => self.byr = Some(value.parse::<u32>().unwrap()),
            "cid" => self.cid = Some(value.parse::<u32>().unwrap()),
            "ecl" => self.ecl = Some(value.parse::<String>().unwrap()),
            "eyr" => self.eyr = Some(value.parse::<u32>().unwrap()),
            "hcl" => self.hcl = Some(value.parse::<String>().unwrap()),
            "hgt" => self.hgt = Some(value.parse::<String>().unwrap()),
            "iyr" => self.iyr = Some(value.parse::<u32>().unwrap()),
            "pid" => self.pid = Some(value.parse::<String>().unwrap()),
            _ => unreachable!(),
        }
    }

    fn build_no_validate(self) -> Option<Passport> {
        match self {
            Self {
                hcl: Some(hcl),
                hgt: Some(hgt),
                ecl: Some(ecl),
                pid: Some(pid),
                byr: Some(byr),
                iyr: Some(iyr),
                eyr: Some(eyr),
                ..
            } => Some(Passport {
                hcl,
                hgt,
                ecl,
                pid,
                byr,
                iyr,
                eyr,
                cid: self.cid,
            }),
            _ => None,
        }
    }
    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    /// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    /// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    /// hgt (Height) - a number followed by either cm or in:
    ///     If cm, the number must be at least 150 and at most 193.
    ///     If in, the number must be at least 59 and at most 76.
    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    /// cid (Country ID) - ignored, missing or not.
    fn build(self) -> Option<Passport> {
        let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match self {
            Self {
                hcl: Some(hcl),
                hgt: Some(hgt),
                ecl: Some(ecl),
                pid: Some(pid),
                byr: Some(byr),
                iyr: Some(iyr),
                eyr: Some(eyr),
                ..
            } => {
                if byr < 1920 || byr > 2002 {
                    return None;
                }

                if iyr < 2010 || iyr > 2020 {
                    return None;
                }

                if eyr < 2020 || eyr > 2030 {
                    return None;
                }

                if let Some(inner) = hgt.strip_suffix("cm") {
                    if let Ok(value) = inner.parse::<u16>() {
                        if value < 150 || value > 193 {
                            println!("{}", hgt);
                            return None;
                        }
                    } else {
                        println!("{}", hgt);
                        return None;
                    }
                } else if let Some(inner) = hgt.strip_suffix("in") {
                    if let Ok(value) = inner.parse::<u16>() {
                        if value < 59 || value > 76 {
                            println!("{}", hgt);
                            return None;
                        }
                    } else {
                        println!("{}", hgt);
                        return None;
                    }
                } else {
                    println!("{}", hgt);
                    return None;
                }

                if let Some(inner) = hcl.strip_prefix("#") {
                    if inner.len() != 6 {
                        return None;
                    }
                    if let Err(_) = u32::from_str_radix(inner, 16) {
                        return None;
                    }
                } else {
                    return None;
                }

                if !eye_colors.contains(&ecl.as_str()) {
                    return None;
                }

                if pid.len() == 9 {
                    if let Err(_) = pid.parse::<u32>() {
                        return None;
                    }
                } else {
                    return None;
                }

                Some(Passport {
                    hcl,
                    hgt,
                    ecl,
                    pid,
                    byr,
                    iyr,
                    eyr,
                    cid: self.cid,
                })
            }
            _ => None,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut strings: Vec<String> = Vec::new();
    let mut curr = String::new();

    for line in contents.split("\n") {
        if line.len() == 0 {
            strings.push(curr);
            curr = String::new();
        } else {
            curr.push_str(&format!(" {}", line));
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for entry in &strings {
        let mut fields: Vec<&str> = entry.split_whitespace().into_iter().collect();
        fields.sort();

        let mut builder = PassportBuilder::new();
        for field in &fields {
            builder.put(field);
        }

        if let Some(_passport) = builder.build_no_validate() {
            p1 += 1;
        }

        let mut builder = PassportBuilder::new();
        for field in fields {
            builder.put(field);
        }

        if let Some(passport) = builder.build() {
            println!("{:#?}", passport);
            p2 += 1;
        }
    }

    println!("[part 1] valid count: {}", p1);

    println!("[part 2] valid count: {}", p2);

    Ok(())
}
