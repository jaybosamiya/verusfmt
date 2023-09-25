use verusfmt::parse_and_format;
use insta::assert_snapshot;

/// Tests of Verus-specific formatting

// We use insta tests (http://insta.rs) to manage the correct answers.
// See README.md for details on how to run and update these tests.

#[test]
fn verus_functions() {
    let file = r#"
pub fn test_function(x: bool, y: bool) -> u32
    by (nonlinear_arith)
    requires
        x,
        y,
    recommends
        x,
    decreases x, y,
    ensures
        x,
{
    assume(x);
    assert(x);
    5
}
spec fn dec0(a: int) -> int
    decreases a,
    when a
    via dec0_decreases
{
    5
}
"#;

    assert_snapshot!(parse_and_format(file).unwrap(), @r###"
    pub fn test_function(x: bool, y: bool) -> u32
        by (nonlinear_arith)
        requires
            x,
            y,
        recommends
            x,
        decreases x, y,
        ensures
            x,
    {
        assume(x);
        assert(x);
        5
    }
    spec fn dec0(a: int) -> int
        decreases a,
        when a
        via dec0_decreases
    {
        5
    }
    "###);
}

#[test]
fn verus_assert_by() {
    let file = r#"
pub fn test_function(x: bool, y: bool) -> u32
    by (nonlinear_arith)
{
    assert(x) by (bit_vector);
    assert(f1(3)) by {
        reveal(f1);
    };
    assert(x) by (nonlinear_arith)
        requires
            x,
            z,
    {
        assert(y);
    };
    assert(forall|x: int, y: int| x) by {
        reveal(f1);
    };
    5
}
"#;

    assert_snapshot!(parse_and_format(file).unwrap(), @r###"
    pub fn test_function(x: bool, y: bool) -> u32
        by (nonlinear_arith)
    {
        assert(x) by (bit_vector);
        assert(f1(3)) by {
            reveal(f1);
        };
        assert(x) by (nonlinear_arith)
            requires
                x,
                z,
        {
            assert(y);
        };
        assert(forall|x: int, y: int| x) by {
            reveal(f1);
        };
        5
    }
    "###);
}

// We deviate from rustfmt here, so use our own snapshot to check for self-consistency
#[test]
fn verus_expr() {
    let file = r#"
pub fn test_function(x: int, y: int) -> u32 {
    let very_very_very_very_very_very_long = very_very_very_very_very_very_x 
        + very_very_very_very_y + very_very_very_very_z;
    5
}    
"#;

    assert_snapshot!(parse_and_format(file).unwrap(), @r###"
    pub fn test_function(x: int, y: int) -> u32 {
        let very_very_very_very_very_very_long = very_very_very_very_very_very_x 
            + very_very_very_very_y + very_very_very_very_z;
        5
    }    
    "###);
}

