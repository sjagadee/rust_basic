trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Archer => "kung fu".to_string(),
            Character::Warrior => "wing chun".to_string(),
            Character::Wizard => "thai chi".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Archer;
        let choosen_fighing_style: String = my_character.choose_style();
        dbg!(choosen_fighing_style);
    }
}