mod math_expr_tests {
    // Imports
    use meval::{eval_str, Expr, Context};

    #[test]
    fn test_simple() {
        assert_eq!(eval_str("1 + 2").unwrap(), 3.0);
    }

    #[test]
    fn test_sample() {
        // Compile expression
        let expr = "phi(-2 * zeta + x)".parse::<Expr>().unwrap();
        // Build execution context
        let mut ctx = Context::new();
        ctx.func("phi", |x| x + 1.)
            .var("zeta", -1.);
        // Combine expression and context, bind variable and output function
        let func = expr.bind_with_context(&ctx, "x").unwrap();
        // Call function and test
        assert_eq!(func(2.), 5.);
    }
}