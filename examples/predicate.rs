use predicates::prelude::*;

fn main() {
    let always_true = predicate::always();
    assert_eq!(true, always_true.eval(&10));

    let less_than_ten = predicate::lt(10);
    assert_eq!(true, less_than_ten.eval(&9));
    assert_eq!(false, less_than_ten.eval(&11));

    let bound = 5;
    let predicate_fn = predicate::function(|&x| x >= bound);
    let between_5_and_10 = predicate_fn.and(predicate::le(10));
    assert_eq!(true, between_5_and_10.eval(&7));
}
