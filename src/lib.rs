use std::collections::HashMap;

mod util;

const EMPTY: char = '∅';
const UNION: char = '⋃';
const INFINITY: char = '∞';

/// `Value` is referencing points. Its type should
/// also implement comparision methods
#[derive(Clone)]
pub enum Value<T> {
    Included(T),
    Secluded(T),
    Infinite,
}

/// `Domain` is an embeddable tool referencing the variants:
/// * `Union` - which means that domains inside are united
/// * `Domain` - single domain with start end end `Value`
/// * `None` - which means empty set (function does not take any value)
/// ```
/// use dynamic_domain::Domain;
/// let domain = Domain::<i32>::new();
/// println!("Domain is {}", domain.repr());
/// ```
#[derive(Clone)]
pub enum Domain<T> {
    /// Domains inside of the vector are united into single domain
    /// Union inside Union is prohibited
    Union(Vec<Domain<T>>),

    /// First value is the starting border (in most implementations default to -infinity)
    /// Second value is the ending border (in most implementations default to infinity)
    Domain(Value<T>, Value<T>),

    /// Empty set
    None,
}

impl Domain<i32> {

    /// Creates new `Domain` with starting border = -infinity;
    /// ending border = infinity
    /// ```
    /// use dynamic_domain::Domain;
    /// let domain = Domain::<i32>::new();
    /// ```
    pub fn new() -> Self {
        Domain::Domain(Value::Infinite, Value::Infinite)
    }

    /// Value starting border is `value`
    /// ```
    /// use dynamic_domain::{Domain, Value};
    /// let domain = Domain::<i32>::new()
    ///     .gt(Value::Secluded(5));
    /// ```
    pub fn gt(&self, value: Value<i32>) -> Self {

        let secluded_value = match util::convert_to_secluded(value) {
            Some(i) => i,
            None => return Domain::None,
        };

        match self.clone() {
            Domain::Domain(
                left,
                right
            ) => {

                match left {

                    Value::Included(i) => {
                        if secluded_value > i.clone() {
                            return Domain::Domain(
                                Value::Included(secluded_value),
                                right
                            );
                        }
                    },
                    Value::Secluded(i) => {
                        if secluded_value > i.clone() {
                            return Domain::Domain(
                                Value::Secluded(secluded_value),
                                right
                            );
                        }
                    },

                    Value::Infinite => {
                        return Domain::Domain(
                            Value::Secluded(secluded_value),
                            right,
                        );
                    }
                }
            },
            _ => (),
        }

        return self.clone();
    }

    /// Value ending border is `value`
    /// ```
    /// use dynamic_domain::{Domain, Value};
    /// let domain = Domain::<i32>::new()
    ///     .lt(Value::Secluded(5));
    /// ```
    pub fn lt(&self, value: Value<i32>) -> Self {

        let secluded_value = match util::convert_to_secluded(value) {
            Some(i) => i,
            None => return Domain::None,
        };

        match self.clone() {
            Domain::Domain(
                left,
                right
            ) => {

                match right {

                    Value::Included(i) => {
                        if secluded_value < i.clone() {
                            return Domain::Domain(
                                left,
                                Value::Included(secluded_value)
                            );
                        }
                    },
                    Value::Secluded(i) => {
                        if secluded_value < i.clone() {
                            return Domain::Domain(
                                left,
                                Value::Secluded(secluded_value)
                            );
                        }
                    },

                    Value::Infinite => {
                        return Domain::Domain(
                            left,
                            Value::Secluded(secluded_value)
                        );
                    }
                }
            },
            _ => (),
        }

        return self.clone();
    }

    /// Performing value representation based on
    /// popular math notations
    /// ```
    /// use dynamic_domain::Domain;
    /// let domain = Domain::new();
    /// println!("{}", domain.repr());
    /// ```
    pub fn repr(self) -> String {
        match self {
            Domain::Union(
                domains
            ) => {

                let domain_reprs = domains
                    .iter()
                    .map(|domain| domain.clone().repr())
                    .collect::<Vec<String>>();

                domain_reprs.join(
                    UNION.to_string().as_str()
                )
            },

            Domain::Domain(left, right) => {
                format!(

                    "{};{}",

                    match left {
                        Value::Included(i) => format!("[{}", i),
                        Value::Secluded(i) => format!("({}", i),
                        Value::Infinite => format!("(-{}", INFINITY),
                    },

                    match right {
                        Value::Included(i) => format!("{}]", i),
                        Value::Secluded(i) => format!("{})", i),
                        Value::Infinite => format!("{})", INFINITY),
                    }

                )
            },
            Domain::None => EMPTY.to_string()
        }
    }

    /// Generates integers by the domain,
    /// sends integers to receiver
    /// ```
    /// use dynamic_domain::{Domain, Value};
    /// let domain = Domain::new()
    ///     .gt(Value::Secluded(5))
    ///     .lt(Value::Included(10));
    /// fn rec(n: i32, c: ()) { println!("Some stuff with {}", n); }
    /// domain.generate(rec, ());
    /// ```
    pub fn generate<Context: Clone>(&self, receiver: fn(i32, Context), context: Context) {

        match self {
            Domain::Union(
                domains
            ) => {
                for domain in domains {
                    domain.generate(receiver, context.clone())
                }
            },

            Domain::Domain(l, r) => {
                let mut from_l = true;
                let mut from_r = false;

                match l.clone() {
                    Value::Infinite => {
                        from_l = false;
                    },
                    _ => (),
                }

                match r.clone() {
                    Value::Infinite => {
                        from_r = false;
                    },
                    _ => from_r = true
                }

                if !from_l && !from_r {
                    return;
                }

                let (mut v, b, f) = if from_l {

                    let v = match l {
                        Value::Included(n) => *n,
                        Value::Secluded(n) => (*n) + 1,
                        _ => 0,
                    };

                    let b = match r {
                        Value::Included(n) => Some(*n),
                        Value::Secluded(n) => Some((*n) - 1),
                        _ => None,
                    };

                    (v, b, (|n: i32| -> i32 { n + 1 }) as fn(i32) -> i32)

                } else {

                    let b = match l {
                        Value::Included(n) => Some(*n),
                        Value::Secluded(n) => Some((*n) + 1),
                        _ => None,
                    };

                    let v = match r {
                        Value::Included(n) => *n,
                        Value::Secluded(n) => (*n) - 1,
                        _ => 0,
                    };

                    (v, b, (|n: i32| -> i32 { n - 1 }) as fn(i32) -> i32)
                };

                loop {
                    receiver(v, context.clone());

                    if let Some(bound) = b {
                        if v == bound { break; }
                    }

                    v = f(v)
                }

            }
            Domain::None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, Value};

    #[test]
    fn test_manual_domain() {
        let domain = Domain::Domain(
            Value::Included(5),
            Value::Secluded(10)
        );
        assert_eq!(domain.repr(), "[5;10)".to_string())
    }

    #[test]
    fn test_new() {
        let domain = Domain::new();
        assert_eq!(domain.repr(), "(-∞;∞)".to_string())
    }

    #[test]
    fn test_gt() {
        let mut domain = Domain::new()
            .gt(Value::Secluded(5));
        assert_eq!(domain.repr(), "(5;∞)".to_string())
    }

    #[test]
    fn test_lt() {
        let mut domain = Domain::new()
            .lt(Value::Included(5))
            .gt(Value::Secluded(3))
            .gt(Value::Secluded(1));
        assert_eq!(domain.repr(), "(3;4)".to_string())
    }

    #[test]
    fn test_generate() {

        fn rec(n: i32, c: ()) {
            assert!(n > 5);
            assert!(n < 10);
        }

        let mut domain = Domain::new()
            .gt(Value::Secluded(5))
            .lt(Value::Secluded(10));

        assert_eq!(domain.clone().repr(), "(5;10)".to_string());

        domain.generate(rec, ());
    }
}
