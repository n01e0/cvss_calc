mod calc;

use std::io::stdin;

fn main() {
    let mut c = calc::Calculator::new();

    c.read_attack_vector(stdin().lock())
        .read_attack_complexity(stdin().lock())
        .read_privileges_required(stdin().lock())
        .read_user_interaction(stdin().lock())
        .read_scope(stdin().lock())
        .read_confidentiality(stdin().lock())
        .read_integrity(stdin().lock())
        .read_availability(stdin().lock());

    println!("{}", c.calc());
    println!("Base score:   {:.1}", c.base_score.score);
    println!("Rating:       {:?}", c.base_score.grade);
}
