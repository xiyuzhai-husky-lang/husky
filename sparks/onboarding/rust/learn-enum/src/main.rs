// = inductive
// 枚举类型
#[derive(Debug, Copy, Clone)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn next(self) -> Self {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => todo!(),
            Weekday::Thursday => todo!(),
            Weekday::Friday => todo!(),
            Weekday::Saturday => todo!(),
            Weekday::Sunday => todo!(),
        }
    }
}

fn main() {
    let x = Weekday::Tuesday;
    println!("Next day of {x:?} is {:?}", x.next());
}
