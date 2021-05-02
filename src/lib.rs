//! # Dynamic domain
//! ...


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

    /// Left value is the starting border (in most implementations default to -infinity)
    /// Right value is the ending border (in most implementations default to infinity)
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

                match left.clone() {

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

                match right.clone() {

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

                    "{},{}",

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

}

#[cfg(test)]
mod tests {
    use crate::{Domain, Value};

    #[test]
    fn test_manual_domain() {
        let domain = Domain::Domain(Value::Included(5), Value::Secluded(10));
        assert_eq!(domain.repr(), "[5,10)".to_string())
    }

    #[test]
    fn test_new() {
        let domain = Domain::new();
        println!("{}", domain.repr());
    }

    #[test]
    fn test_gt() {
        let mut domain = Domain::new()
            .gt(Value::Secluded(5));
        println!("{}", domain.repr());
    }

    #[test]
    fn test_lt() {
        let mut domain = Domain::new()
            .lt(Value::Secluded(5))
            .gt(Value::Included(3))
            .gt(Value::Secluded(2));
        println!("{}", domain.repr());
    }
}