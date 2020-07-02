#[cfg(test)]
mod test {
    extern crate cvss_calc;

    use cvss_calc::calc::*;
    #[test]
    fn test_default() {
        let mut c = Calculator::new();
        c.calc();
        assert_eq!(0.0, c.base_score.score);
        assert_eq!(Grade::None, c.base_score.grade);
    }

    #[test]
    fn test_cve_2020_9292() {
        let mut c = Calculator::new();
        c.av = AttackVector::Network;
        c.ac = AttackComplexity::Low;
        c.pr = PrivilegesRequired::None;
        c.ui = UserInteraction::None;
        c.s  = Scope::Unchanged;
        c.c  = Confidentiality::High;
        c.i  = Integrity::High;
        c.a  = Availability::High;

        c.calc();
        assert_eq!(9.8, c.base_score.score);
        assert_eq!(Grade::Critical, c.base_score.grade);
    }

    #[test]
    fn test_cve_2020_13904() {
        let mut c = Calculator::new();
        c.read_attack_vector("l".as_bytes())
            .read_attack_complexity("Low".as_bytes())
            .read_privileges_required("none".as_bytes())
            .read_user_interaction("r".as_bytes())
            .read_scope("u".as_bytes())
            .read_confidentiality("n".as_bytes())
            .read_integrity("none".as_bytes())
            .read_availability("high".as_bytes())
            .calc();

        assert_eq!(Grade::Medium, c.base_score.grade);
    }

    #[test]
    fn test_display_initial() {
        #[derive(display_initial::DisplayInitial, Debug)]
        enum Hoge {
            Fuga,
        }

        let hoge = Hoge::Fuga;
        assert_eq!("F", format!("{}", hoge));
    }

    #[test]
    fn test_parse() {
        match "local".to_string().parse::<AttackVector>() {
            Ok(t) => {
                assert_eq!(AttackVector::Local, t);
                ()
            }
            Err(_) => panic!(),
        }
    }
}
