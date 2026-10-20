#[derive(Copy, Clone, Debug, PartialEq)]
enum Var {
    X,
    Y,
    Z
}

impl Var {
    fn to_string(&self) -> String {
        return match self {
            Var::X => "X".to_string(),
            Var::Y => "Y".to_string(),
            Var::Z => "Z".to_string()
        }
    }
}

#[derive(Clone, Debug)]
enum Const {
    Numeric(i64),
    Named(String)
}

impl Const {
    fn to_string(&self) -> String {
        match self {
            Const::Numeric(n) => n.to_string(),
            Const::Named(name) => name.clone()
        }
    }
}

#[derive(Clone, Debug)]
enum E {
    Add(Box<E>, Box<E>),
    Neg(Box<E>),
    Mul(Box<E>, Box<E>),
    Inv(Box<E>),
    Const(Const),
    Func { name: String, args: Box<E>},
    Var(Var)
}

impl E {
    fn add(left: Box<E>, right: Box<E>) -> Box<Self> { Box::new(E::Add(left, right))}
    fn neg(expression: Box<E>) -> Box<Self> { Box::new(E::Neg(expression)) }
    fn mul(left: Box<E>, right: Box<E>)-> Box<Self> { Box::new(E::Mul(left, right)) }
    fn inv(expression: Box<E>) -> Box<Self> {Box::new(E::Inv(expression))}
    fn constant(constant : Const) -> Box<Self> { Box::new(constant) }
    fn func(name: String, args: Box<E>) -> Box<Self> { Box::new(E::Func {name, args})}
    fn var(var: Var) -> Box<Self> { Box::new(var) }

    fn to_string(&self) -> String {
        match self {
            E::Add(left, right) => format!("({} + {})", left.to_string(), right.to_string()),
            E::Neg(expression) => format!("-{}", expression.to_string()),
            E::Mul(left, right) => format!("({} * {})", left.to_string(), right.to_string()),
            E::Inv(expression) => format!("(inv {})", expression.to_string()),
            E::Const(constant) => constant.to_string(),
            E::Func { name, args } => format!("{}({})", name, args.to_string()),
            E::Var(var) => var.to_string()
        }
    }

    fn arg_count(&self) -> u32 {
        match self {
            E::Const(_) | E::Var(_) => 0,
            E::Neg(_) | E::Inv(_) | E::Func(_, _) => 1,
            E::Add(_, _) | E::Mul(_, _) => 2
        }
    }

    fn diff(&self, by: Var) -> Box<Self> {
        match self {
            E::Const(_) => constant(Const::Numeric(0)),
            E::Var(var) => if var == by {constant(Const::Numeric(1))} else {constant(Const::Numeric(0))},
            E::Func { name, args } => {}
            E::Add(left, right) => E::add(left.diff(by), right.diff(by)),
            E::Neg(expression) => E::neg(expression.diff(by)),
            E::Mul(left, right) => E::add(E::mul(left.diff(by), right), E::mul(left, right.diff(by))),
            E::Inv(expression) => E::mul(E::neg(E::inv((E::mul(expression, expression)))), expression.diff(by)),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_to_string() {
        let c_num = Const::Numeric(42);
        let c_name = Const::Named("a".into());
        assert_eq!(c_num.to_string(), "42");
        assert_eq!(c_name.to_string(), "a");
    }

    #[test]
    fn test_var_to_string() {
        assert_eq!(Var::X.to_string(), "X");
        assert_eq!(Var::Y.to_string(), "Y");
        assert_eq!(Var::Z.to_string(), "Z");
    }

    #[test]
    fn test_builder_constant_var() {
        let e_const = E::constant(Const::Numeric(5));
        let e_var = E::var(Var::X);
        assert_eq!(e_const.to_string(), "5");
        assert_eq!(e_var.to_string(), "X");
    }

    #[test]
    fn test_builder_add() {
        let expr = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        assert_eq!(expr.to_string(), "(2 + X)");
    }

    #[test]
    fn test_builder_neg() {
        let expr = E::neg(E::var(Var::X));
        assert_eq!(expr.to_string(), "-(X)");
    }

    #[test]
    fn test_builder_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        assert_eq!(expr.to_string(), "(X * Y)");
    }

    #[test]
    fn test_builder_inv() {
        let expr = E::inv(E::var(Var::X));
        assert_eq!(expr.to_string(), "1/(X)");
    }

    #[test]
    fn test_builder_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        assert_eq!(expr.to_string(), "f(X)");
    }

    #[test]
    fn test_expr_to_string_complex() {
        let expr1 = E::add(E::constant(Const::Numeric(2)), E::var(Var::X));
        let expr2 = E::mul(E::neg(E::var(Var::Y)), E::inv(E::var(Var::Z)));
        let complex = E::add(
            E::func("f".into(), expr1.clone()),
            E::func("g".into(), expr2.clone()),
        );
        assert_eq!(complex.to_string(), "(f((2 + X)) + g((-(Y) * 1/(Z))))");
    }

    #[test]
    fn test_diff_add_vars() {
        let expr = E::add(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(1 + 0)");
    }

    #[test]
    fn test_unpack_inv_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let inner = double_inv.clone().unpack_inv_inv().unwrap();
        assert_eq!(inner.to_string(), "X");
    }

    #[test]
    fn test_unpack_neg_neg() {
        let double_neg = E::neg(E::neg(E::neg(E::neg(E::neg(E::var(Var::Y))))));
        let inner = double_neg.clone().unneg();
        assert_eq!(inner.to_string(), "-(Y)");
    }

    #[test]
    fn test_simplify_double_inv() {
        let double_inv = E::inv(E::inv(E::var(Var::X)));
        let simplified = double_inv.uninv();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_simplify_double_neg() {
        let double_neg = E::neg(E::neg(E::var(Var::X)));
        let simplified = double_neg.unneg();
        assert_eq!(simplified.to_string(), "X");
    }

    #[test]
    fn test_substitute_named_constant() {
        let expr = E::add(E::constant(Const::Named("a".into())), E::var(Var::X));
        let substituted = expr.substitute("a", E::constant(Const::Numeric(10)));
        assert_eq!(substituted.to_string(), "(10 + X)");
    }

    #[test]
    fn test_substitute_deep() {
        let expr = E::mul(
            E::constant(Const::Named("a".into())),
            E::func("f".into(), E::constant(Const::Named("a".into()))),
        );
        let substituted = expr.substitute("a", E::constant(Const::Numeric(3)));
        assert_eq!(substituted.to_string(), "(3 * f(3))");
    }

    #[test]
    fn test_diff_neg() {
        let expr = E::neg(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "-(1)");
    }

    #[test]
    fn test_diff_mul() {
        let expr = E::mul(E::var(Var::X), E::var(Var::Y));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "((1 * Y) + (X * 0))");
    }

    #[test]
    fn test_diff_inv() {
        let expr = E::inv(E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(-(1/((X * X))) * 1)");
    }

    #[test]
    fn test_diff_const_numeric() {
        let expr = E::constant(Const::Numeric(7));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_const_named() {
        let expr = E::constant(Const::Named("a".into()));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_func() {
        let expr = E::func("f".into(), E::var(Var::X));
        let d = expr.diff(Var::X);
        assert_eq!(d.to_string(), "(f_X(X) * 1)");
    }

    #[test]
    fn test_diff_var_same() {
        let d = E::var(Var::X).diff(Var::X);
        assert_eq!(d.to_string(), "1");
    }

    #[test]
    fn test_diff_var_other() {
        let d = E::var(Var::Y).diff(Var::X);
        assert_eq!(d.to_string(), "0");
    }

    #[test]
    fn test_diff_big_expression() {
        // (((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))
        let part1 = E::add(E::var(Var::X), E::neg(E::var(Var::Y)));
        let part2 = E::inv(E::var(Var::Z));
        let a = E::mul(part1.clone(), part2.clone());
        let xy = E::mul(E::var(Var::X), E::var(Var::Y));
        let b = E::func("f".into(), xy);
        let inv_x = E::inv(E::var(Var::X));
        let c = E::func("g".into(), inv_x);
        let big = E::add(a.clone(), E::add(b.clone(), c.clone()));

        assert_eq!(
            big.to_string(),
            "(((X + -(Y)) * 1/(Z)) + (f((X * Y)) + g(1/(X))))"
        );

        let d = big.diff(Var::X);
        assert_eq!(
            d.to_string(),
            "((((1 + -(0)) * 1/(Z)) + ((X + -(Y)) * (-(1/((Z * Z))) * 0))) + ((f_X((X * Y)) * ((1 * Y) + (X * 0))) + (g_X(1/(X)) * (-(1/((X * X))) * 1))))"
        );
    }

    #[test]
    fn test_arg_count_zeroary() {
        assert_eq!(E::constant(Const::Numeric(1)).arg_count(), 0);
        assert_eq!(E::var(Var::X).arg_count(), 0);
    }

    #[test]
    fn test_arg_count_unary() {
        assert_eq!(E::neg(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::inv(E::var(Var::X)).arg_count(), 1);
        assert_eq!(E::func("f".into(), E::var(Var::X)).arg_count(), 1);
    }

    #[test]
    fn test_arg_count_binary() {
        assert_eq!(E::add(E::var(Var::X), E::var(Var::Y)).arg_count(), 2);
        assert_eq!(E::mul(E::var(Var::X), E::var(Var::Z)).arg_count(), 2);
    }
}