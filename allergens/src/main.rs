use std::fmt;
use std::io::Write;

mod allergens;
use allergens::*;

const ALLERGENS: [u32; 8] = [EGGS, PNTS, SHFS, STWB, TMTO, CHLT, PLLN, CATS];

#[derive(Debug, PartialEq)]
// Generic is not necessary, just for implementation.
struct Allergen<T>(T);

impl<T> Allergen<T> {
    fn new(value: T) -> Self {
        Self { 0: value }
    }
}

impl FromIterator<u32> for Allergen<u32> {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut a = Allergen::new(0);

        for i in iter {
            a.0 += i;
        }

        a
    }
}

#[derive(Debug)]
struct Person {
    age: u8,
    name: String,
    height: f32,
    weight: f32,
    allergies: Allergen<u32>,
}

impl Person {
    fn new(m: (u8, &str, f32, f32, Allergen<u32>)) -> Self {
        let (age, name, height, weight, allergies) = m;

        Self {
            age,
            name: String::from(name),
            height,
            weight,
            allergies,
        }
    }

    fn is_allergic_to(&self, allergen: Allergen<u32>) -> bool {
        //! Compares if allergy bit is set in variable.
        //!
        //! Example:
        //! let a = Person::new(18, "John", 175.0, 85, EGGS|TMTO);
        //! assert_eq!(true, a.is_allergic_to(EGGS));

        (allergen.0 & self.allergies.0) == allergen.0
    }

    fn allergies(&self) -> Allergen<u32> {
        //! Loops through all known allergies and checks which allergy bit
        //! is set for current user.
        //!
        //! Example:
        //! let a = Person::new(32, "Britney", 156.3, 60.8, EGGS);
        //! assert_eq!(Allergen(EGGS), a.allergies());
        ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(Allergen::new(**a)))
            .cloned()
            .collect()
    }
}

impl fmt::Display for Allergen<u32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut score: u32 = self.0;
        let mut alrgn_str: String = String::new();

        if score == 0 {
            alrgn_str.clear();
            alrgn_str.push_str("NONE");
        } else {
            while score != 0 {
                if score & EGGS == EGGS {
                    score &= !EGGS;
                    alrgn_str.push_str("EGGS");
                } else if score & PNTS == PNTS {
                    score &= !PNTS;
                    alrgn_str.push_str("PNTS");
                } else if score & SHFS == SHFS {
                    score &= !SHFS;
                    alrgn_str.push_str("SHFS");
                } else if score & STWB == STWB {
                    score &= !STWB;
                    alrgn_str.push_str("STWB");
                } else if score & TMTO == TMTO {
                    score &= !STWB;
                    alrgn_str.push_str("TMTO");
                } else if score & CHLT == CHLT {
                    score &= !CHLT;
                    alrgn_str.push_str("CHLT");
                } else if score & PLLN == PLLN {
                    score &= !PLLN;
                    alrgn_str.push_str("PLLN");
                } else if score & CATS == CATS {
                    score &= !CATS;
                    alrgn_str.push_str("CATS");
                }

                alrgn_str.push('|');
            }
        }

        write!(f, "{}", alrgn_str.trim_matches('|'))
    }
}

fn score_menu(user: &mut Person) {
    let mut score_buf: String = String::new();

    loop {
        println!("{:?}", ALLERGENS);
        print!("Enter a valid allergy number: ");

        std::io::stdout()
            .flush()
            .expect("Error: Could not flush stdout.");

        std::io::stdin()
            .read_line(&mut score_buf)
            .expect("Error: Could not read input.");

        score_buf = score_buf.trim().to_string();

        if let Ok(u) = score_buf.parse::<u32>() {
            if u & ALLM == 0 {
                eprintln!("Invalid option");
                continue;
            }

            user.allergies.0 |= u;
        } else if let Ok(c) = score_buf.parse::<char>() {
            if c == 'q' {
                break;
            } else if c == 'a' {
                println!("{} has {} allergies.", user.name, user.allergies());
            } else if c == 'i' {
                println!(
                    "{}: Age = {}; Height = {:.2}; Weight = {:.2}",
                    user.name, user.age, user.height, user.weight
                );
            } else if c == 'c' {
                print!("Enter allergy identifier you want to remove: ");
                std::io::stdout()
                    .flush()
                    .expect("Error: Could not flush output.");
                score_buf.clear();
                std::io::stdin()
                    .read_line(&mut score_buf)
                    .expect("Error: Could not read input");

                score_buf = score_buf.trim().to_string();

                if let Ok(u) = score_buf.parse::<u32>() {
                    if u & ALLM != 0 {
                        user.allergies.0 &= !u;
                    }
                }
            }
        }

        score_buf.clear();
    }
}

fn main() {
    let t: Allergen<u32> = Allergen::new(0);
    let mut p = Person::new((18, "John Doe", 1.80, 70.0, Allergen::new(t.0)));

    score_menu(&mut p);
}
