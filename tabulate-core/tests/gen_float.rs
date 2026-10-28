// Generated from CPython 3.12 output. DO NOT HAND-EDIT.
use tabulate_core::numfmt::{float_repr, format_float, parse_spec};

#[test]
fn generated_float_vectors() {
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.0".to_string()), "0.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.00".to_string()), "0.0|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.000".to_string()), "0.0|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.00".to_string()), "0.0|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.0e+00".to_string()),
            "0.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.00E+00".to_string()),
            "0.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.000000%".to_string()),
            "0.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.0%".to_string()), "0.0|.1%");
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.".to_string()), "0.0|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.00000".to_string()),
            "0.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("      0.00".to_string()),
            "0.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("00000.00".to_string()),
            "0.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("+0.00".to_string()),
            "0.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok(" 0.00".to_string()),
            "0.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("      0.00".to_string()),
            "0.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("   0.00   ".to_string()),
            "0.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.00      ".to_string()),
            "0.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0000000.00".to_string()),
            "0.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok(float_repr(0.0)), "0.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(0.0, &spec),
            Ok("0.0000000000".to_string()),
            "0.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0".to_string()), "0.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(0.0, &spec), Ok("0.00".to_string()), "0.0|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(0.0, &spec).is_err(), "0.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0.0".to_string()), "-0.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00".to_string()),
            "-0.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.000".to_string()),
            "-0.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00".to_string()),
            "-0.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.0e+00".to_string()),
            "-0.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00E+00".to_string()),
            "-0.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.000000%".to_string()),
            "-0.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.0%".to_string()),
            "-0.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.".to_string()),
            "-0.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00000".to_string()),
            "-0.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("     -0.00".to_string()),
            "-0.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0000.00".to_string()),
            "-0.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00".to_string()),
            "-0.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00".to_string()),
            "-0.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-     0.00".to_string()),
            "-0.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("  -0.00   ".to_string()),
            "-0.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00     ".to_string()),
            "-0.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("00000-0.00".to_string()),
            "-0.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok(float_repr(-0.0)), "-0.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.0000000000".to_string()),
            "-0.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(-0.0, &spec), Ok("-0".to_string()), "-0.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(-0.0, &spec),
            Ok("-0.00".to_string()),
            "-0.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(-0.0, &spec).is_err(), "-0.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.0".to_string()), "1.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.00".to_string()), "1.0|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.000".to_string()), "1.0|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.00".to_string()), "1.0|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("1.0e+00".to_string()),
            "1.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("1.00E+00".to_string()),
            "1.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("100.000000%".to_string()),
            "1.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("100.0%".to_string()),
            "1.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.".to_string()), "1.0|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("1.00000".to_string()),
            "1.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("      1.00".to_string()),
            "1.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("00001.00".to_string()),
            "1.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("+1.00".to_string()),
            "1.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok(" 1.00".to_string()),
            "1.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("      1.00".to_string()),
            "1.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("   1.00   ".to_string()),
            "1.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("1.00      ".to_string()),
            "1.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("0000001.00".to_string()),
            "1.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok(float_repr(1.0)), "1.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1.0, &spec),
            Ok("1.0000000000".to_string()),
            "1.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1".to_string()), "1.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(1.0, &spec), Ok("1.00".to_string()), "1.0|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1.0, &spec).is_err(), "1.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1.0".to_string()), "-1.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00".to_string()),
            "-1.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.000".to_string()),
            "-1.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00".to_string()),
            "-1.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.0e+00".to_string()),
            "-1.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00E+00".to_string()),
            "-1.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-100.000000%".to_string()),
            "-1.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-100.0%".to_string()),
            "-1.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.".to_string()),
            "-1.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00000".to_string()),
            "-1.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("     -1.00".to_string()),
            "-1.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-0001.00".to_string()),
            "-1.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00".to_string()),
            "-1.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00".to_string()),
            "-1.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-     1.00".to_string()),
            "-1.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("  -1.00   ".to_string()),
            "-1.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00     ".to_string()),
            "-1.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("00000-1.00".to_string()),
            "-1.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok(float_repr(-1.0)), "-1.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.0000000000".to_string()),
            "-1.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(-1.0, &spec), Ok("-1".to_string()), "-1.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(-1.0, &spec),
            Ok("-1.00".to_string()),
            "-1.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(-1.0, &spec).is_err(), "-1.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.50".to_string()), "1.5|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.500".to_string()), "1.5|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.50".to_string()), "1.5|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("2".to_string()), "1.5|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("1.5e+00".to_string()),
            "1.5|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("1.50E+00".to_string()),
            "1.5|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("150.000000%".to_string()),
            "1.5|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("150.0%".to_string()),
            "1.5|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("2.".to_string()), "1.5|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("1.50000".to_string()),
            "1.5|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("2".to_string()), "1.5|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("      1.50".to_string()),
            "1.5|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("00001.50".to_string()),
            "1.5|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("+1.50".to_string()),
            "1.5|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok(" 1.50".to_string()),
            "1.5| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("      1.50".to_string()),
            "1.5|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("   1.50   ".to_string()),
            "1.5|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("1.50      ".to_string()),
            "1.5|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("0000001.50".to_string()),
            "1.5|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok(float_repr(1.5)), "1.5|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("2".to_string()), "1.5|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1.5, &spec),
            Ok("1.5000000000".to_string()),
            "1.5|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.5".to_string()), "1.5|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(1.5, &spec), Ok("1.50".to_string()), "1.5|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1.5, &spec).is_err(), "1.5|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-1.5".to_string()), "-1.5|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50".to_string()),
            "-1.5|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.500".to_string()),
            "-1.5|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-1.5".to_string()), "-1.5|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.5".to_string()),
            "-1.5|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.5".to_string()),
            "-1.5|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50".to_string()),
            "-1.5|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-2".to_string()), "-1.5|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.5e+00".to_string()),
            "-1.5|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50E+00".to_string()),
            "-1.5|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-150.000000%".to_string()),
            "-1.5|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-150.0%".to_string()),
            "-1.5|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-2.".to_string()),
            "-1.5|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50000".to_string()),
            "-1.5|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-2".to_string()), "-1.5|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("     -1.50".to_string()),
            "-1.5|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-0001.50".to_string()),
            "-1.5|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50".to_string()),
            "-1.5|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50".to_string()),
            "-1.5| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-     1.50".to_string()),
            "-1.5|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("  -1.50   ".to_string()),
            "-1.5|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50     ".to_string()),
            "-1.5|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("00000-1.50".to_string()),
            "-1.5|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok(float_repr(-1.5)), "-1.5|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-1.5".to_string()), "-1.5|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(-1.5, &spec), Ok("-2".to_string()), "-1.5|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.5000000000".to_string()),
            "-1.5|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.5".to_string()),
            "-1.5|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(-1.5, &spec),
            Ok("-1.50".to_string()),
            "-1.5|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(-1.5, &spec).is_err(), "-1.5|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.0".to_string()),
            "100.0|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.00".to_string()),
            "100.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.000".to_string()),
            "100.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok("100".to_string()), "100.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100".to_string()),
            "100.0|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100".to_string()),
            "100.0|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.00".to_string()),
            "100.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100".to_string()),
            "100.0|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("1.0e+02".to_string()),
            "100.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("1.00E+02".to_string()),
            "100.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("10000.000000%".to_string()),
            "100.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("10000.0%".to_string()),
            "100.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.".to_string()),
            "100.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.000".to_string()),
            "100.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("1e+02".to_string()),
            "100.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("    100.00".to_string()),
            "100.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("00100.00".to_string()),
            "100.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("+100.00".to_string()),
            "100.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(" 100.00".to_string()),
            "100.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("    100.00".to_string()),
            "100.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("  100.00  ".to_string()),
            "100.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.00    ".to_string()),
            "100.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("0000100.00".to_string()),
            "100.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok(float_repr(100.0)), "100.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok(float_repr(100.0)), "100.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok(float_repr(100.0)), "100.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok(float_repr(100.0)), "100.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok(float_repr(100.0)),
            "100.0|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok(float_repr(100.0)), "100.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(100.0, &spec), Ok("100".to_string()), "100.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100".to_string()),
            "100.0|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.0000000000".to_string()),
            "100.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("1E+02".to_string()),
            "100.0|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(100.0, &spec),
            Ok("100.00".to_string()),
            "100.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(100.0, &spec).is_err(), "100.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.46".to_string()),
            "123.456|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.46".to_string()),
            "123.456|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.46".to_string()),
            "123.456|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123".to_string()),
            "123.456|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("1.2e+02".to_string()),
            "123.456|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("1.23E+02".to_string()),
            "123.456|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("12345.600000%".to_string()),
            "123.456|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("12345.6%".to_string()),
            "123.456|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.".to_string()),
            "123.456|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("1e+02".to_string()),
            "123.456|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("    123.46".to_string()),
            "123.456|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("00123.46".to_string()),
            "123.456|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("+123.46".to_string()),
            "123.456|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(" 123.46".to_string()),
            "123.456| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("    123.46".to_string()),
            "123.456|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("  123.46  ".to_string()),
            "123.456|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.46    ".to_string()),
            "123.456|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("0000123.46".to_string()),
            "123.456|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok(float_repr(123.456)),
            "123.456|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.456".to_string()),
            "123.456|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123".to_string()),
            "123.456|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.4560000000".to_string()),
            "123.456|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("1.2E+02".to_string()),
            "123.456|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(123.456, &spec),
            Ok("123.46".to_string()),
            "123.456|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(123.456, &spec).is_err(), "123.456|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.000".to_string()),
            "1e+16|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10,000,000,000,000,000.00".to_string()),
            "1e+16|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000".to_string()),
            "1e+16|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1.0e+16".to_string()),
            "1e+16|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1.00E+16".to_string()),
            "1e+16|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1000000000000000000.000000%".to_string()),
            "1e+16|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1000000000000000000.0%".to_string()),
            "1e+16|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.".to_string()),
            "1e+16|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1.00000e+16".to_string()),
            "1e+16|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("+10000000000000000.00".to_string()),
            "1e+16|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(" 10000000000000000.00".to_string()),
            "1e+16| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.00".to_string()),
            "1e+16|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(1e+16, &spec), Ok(float_repr(1e+16)), "1e+16|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(1e+16, &spec), Ok(float_repr(1e+16)), "1e+16|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(1e+16, &spec), Ok(float_repr(1e+16)), "1e+16|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(1e+16, &spec), Ok(float_repr(1e+16)), "1e+16|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok(float_repr(1e+16)),
            "1e+16|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(1e+16, &spec), Ok(float_repr(1e+16)), "1e+16|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1e+16".to_string()),
            "1e+16|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10,000,000,000,000,000".to_string()),
            "1e+16|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10000000000000000.0000000000".to_string()),
            "1e+16|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("1E+16".to_string()),
            "1e+16|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1e+16, &spec),
            Ok("10_000_000_000_000_000.00".to_string()),
            "1e+16|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1e+16, &spec).is_err(), "1e+16|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.0".to_string()),
            "1000000000000000.0|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.000".to_string()),
            "1000000000000000.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1e+15".to_string()),
            "1000000000000000.0|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1e+15".to_string()),
            "1000000000000000.0|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1e+15".to_string()),
            "1000000000000000.0|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1,000,000,000,000,000.00".to_string()),
            "1000000000000000.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000".to_string()),
            "1000000000000000.0|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1.0e+15".to_string()),
            "1000000000000000.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1.00E+15".to_string()),
            "1000000000000000.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("100000000000000000.000000%".to_string()),
            "1000000000000000.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("100000000000000000.0%".to_string()),
            "1000000000000000.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.".to_string()),
            "1000000000000000.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1.00000e+15".to_string()),
            "1000000000000000.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1e+15".to_string()),
            "1000000000000000.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("+1000000000000000.00".to_string()),
            "1000000000000000.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(" 1000000000000000.00".to_string()),
            "1000000000000000.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.00".to_string()),
            "1000000000000000.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok(float_repr(1000000000000000.0)),
            "1000000000000000.0|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1e+15".to_string()),
            "1000000000000000.0|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1,000,000,000,000,000".to_string()),
            "1000000000000000.0|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1000000000000000.0000000000".to_string()),
            "1000000000000000.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1E+15".to_string()),
            "1000000000000000.0|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1000000000000000.0, &spec),
            Ok("1_000_000_000_000_000.00".to_string()),
            "1000000000000000.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(
                format_float(1000000000000000.0, &spec).is_err(),
                "1000000000000000.0|s"
            ),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.891".to_string()),
            "1234567.891|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.891".to_string()),
            "1234567.891|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.23457e+06".to_string()),
            "1234567.891|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.23457e+06".to_string()),
            "1234567.891|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.2346e+06".to_string()),
            "1234567.891|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1,234,567.89".to_string()),
            "1234567.891|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234568".to_string()),
            "1234567.891|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.2e+06".to_string()),
            "1234567.891|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.23E+06".to_string()),
            "1234567.891|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("123456789.100000%".to_string()),
            "1234567.891|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("123456789.1%".to_string()),
            "1234567.891|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234568.".to_string()),
            "1234567.891|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.23457e+06".to_string()),
            "1234567.891|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1e+06".to_string()),
            "1234567.891|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("+1234567.89".to_string()),
            "1234567.891|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(" 1234567.89".to_string()),
            "1234567.891| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.89".to_string()),
            "1234567.891|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok(float_repr(1234567.891)),
            "1234567.891|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.23457e+06".to_string()),
            "1234567.891|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1,234,568".to_string()),
            "1234567.891|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1234567.8910000001".to_string()),
            "1234567.891|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1.2E+06".to_string()),
            "1234567.891|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1234567.891, &spec),
            Ok("1_234_567.89".to_string()),
            "1234567.891|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1234567.891, &spec).is_err(), "1234567.891|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.00".to_string()),
            "0.0001|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.000".to_string()),
            "0.0001|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.00".to_string()),
            "0.0001|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0".to_string()),
            "0.0001|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("1.0e-04".to_string()),
            "0.0001|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("1.00E-04".to_string()),
            "0.0001|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.010000%".to_string()),
            "0.0001|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0%".to_string()),
            "0.0001|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.".to_string()),
            "0.0001|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.000100000".to_string()),
            "0.0001|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("      0.00".to_string()),
            "0.0001|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("00000.00".to_string()),
            "0.0001|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("+0.00".to_string()),
            "0.0001|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(" 0.00".to_string()),
            "0.0001| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("      0.00".to_string()),
            "0.0001|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("   0.00   ".to_string()),
            "0.0001|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.00      ".to_string()),
            "0.0001|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0000000.00".to_string()),
            "0.0001|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok(float_repr(0.0001)),
            "0.0001|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0".to_string()),
            "0.0001|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001000000".to_string()),
            "0.0001|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.0001".to_string()),
            "0.0001|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(0.0001, &spec),
            Ok("0.00".to_string()),
            "0.0001|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(0.0001, &spec).is_err(), "0.0001|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.00".to_string()),
            "1e-05|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.000".to_string()),
            "1e-05|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.00".to_string()),
            "1e-05|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok("0".to_string()), "1e-05|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1.0e-05".to_string()),
            "1e-05|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1.00E-05".to_string()),
            "1e-05|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.001000%".to_string()),
            "1e-05|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.0%".to_string()),
            "1e-05|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.".to_string()),
            "1e-05|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1.00000e-05".to_string()),
            "1e-05|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("      0.00".to_string()),
            "1e-05|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("00000.00".to_string()),
            "1e-05|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("+0.00".to_string()),
            "1e-05|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(" 0.00".to_string()),
            "1e-05| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("      0.00".to_string()),
            "1e-05|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("   0.00   ".to_string()),
            "1e-05|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.00      ".to_string()),
            "1e-05|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0000000.00".to_string()),
            "1e-05|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok(float_repr(1e-05)), "1e-05|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok(float_repr(1e-05)), "1e-05|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok(float_repr(1e-05)), "1e-05|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok(float_repr(1e-05)), "1e-05|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok(float_repr(1e-05)),
            "1e-05|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(1e-05, &spec), Ok(float_repr(1e-05)), "1e-05|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1e-05".to_string()),
            "1e-05|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0".to_string()),
            "1e-05|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.0000100000".to_string()),
            "1e-05|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("1E-05".to_string()),
            "1e-05|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1e-05, &spec),
            Ok("0.00".to_string()),
            "1e-05|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1e-05, &spec).is_err(), "1e-05|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23456789".to_string()),
            "1.23456789|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23".to_string()),
            "1.23456789|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.235".to_string()),
            "1.23456789|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23457".to_string()),
            "1.23456789|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23457".to_string()),
            "1.23456789|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.2346".to_string()),
            "1.23456789|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23".to_string()),
            "1.23456789|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1".to_string()),
            "1.23456789|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.2e+00".to_string()),
            "1.23456789|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23E+00".to_string()),
            "1.23456789|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("123.456789%".to_string()),
            "1.23456789|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("123.5%".to_string()),
            "1.23456789|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.".to_string()),
            "1.23456789|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23457".to_string()),
            "1.23456789|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1".to_string()),
            "1.23456789|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("      1.23".to_string()),
            "1.23456789|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("00001.23".to_string()),
            "1.23456789|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("+1.23".to_string()),
            "1.23456789|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(" 1.23".to_string()),
            "1.23456789| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("      1.23".to_string()),
            "1.23456789|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("   1.23   ".to_string()),
            "1.23456789|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23      ".to_string()),
            "1.23456789|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("0000001.23".to_string()),
            "1.23456789|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok(float_repr(1.23456789)),
            "1.23456789|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23457".to_string()),
            "1.23456789|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1".to_string()),
            "1.23456789|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.2345678900".to_string()),
            "1.23456789|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.2".to_string()),
            "1.23456789|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1.23456789, &spec),
            Ok("1.23".to_string()),
            "1.23456789|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1.23456789, &spec).is_err(), "1.23456789|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.50".to_string()), "2.5|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.500".to_string()), "2.5|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.50".to_string()), "2.5|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2".to_string()), "2.5|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("2.5e+00".to_string()),
            "2.5|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("2.50E+00".to_string()),
            "2.5|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("250.000000%".to_string()),
            "2.5|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("250.0%".to_string()),
            "2.5|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.".to_string()), "2.5|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("2.50000".to_string()),
            "2.5|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2".to_string()), "2.5|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("      2.50".to_string()),
            "2.5|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("00002.50".to_string()),
            "2.5|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("+2.50".to_string()),
            "2.5|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok(" 2.50".to_string()),
            "2.5| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("      2.50".to_string()),
            "2.5|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("   2.50   ".to_string()),
            "2.5|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("2.50      ".to_string()),
            "2.5|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("0000002.50".to_string()),
            "2.5|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok(float_repr(2.5)), "2.5|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2".to_string()), "2.5|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(2.5, &spec),
            Ok("2.5000000000".to_string()),
            "2.5|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.5".to_string()), "2.5|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(2.5, &spec), Ok("2.50".to_string()), "2.5|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(2.5, &spec).is_err(), "2.5|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.50".to_string()), "3.5|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.500".to_string()), "3.5|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.50".to_string()), "3.5|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("4".to_string()), "3.5|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("3.5e+00".to_string()),
            "3.5|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("3.50E+00".to_string()),
            "3.5|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("350.000000%".to_string()),
            "3.5|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("350.0%".to_string()),
            "3.5|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("4.".to_string()), "3.5|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("3.50000".to_string()),
            "3.5|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("4".to_string()), "3.5|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("      3.50".to_string()),
            "3.5|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("00003.50".to_string()),
            "3.5|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("+3.50".to_string()),
            "3.5|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok(" 3.50".to_string()),
            "3.5| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("      3.50".to_string()),
            "3.5|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("   3.50   ".to_string()),
            "3.5|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("3.50      ".to_string()),
            "3.5|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("0000003.50".to_string()),
            "3.5|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok(float_repr(3.5)), "3.5|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("4".to_string()), "3.5|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(3.5, &spec),
            Ok("3.5000000000".to_string()),
            "3.5|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.5".to_string()), "3.5|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(3.5, &spec), Ok("3.50".to_string()), "3.5|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(3.5, &spec).is_err(), "3.5|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2.5".to_string()), "-2.5|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50".to_string()),
            "-2.5|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.500".to_string()),
            "-2.5|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2.5".to_string()), "-2.5|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.5".to_string()),
            "-2.5|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.5".to_string()),
            "-2.5|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50".to_string()),
            "-2.5|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2".to_string()), "-2.5|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.5e+00".to_string()),
            "-2.5|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50E+00".to_string()),
            "-2.5|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-250.000000%".to_string()),
            "-2.5|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-250.0%".to_string()),
            "-2.5|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.".to_string()),
            "-2.5|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50000".to_string()),
            "-2.5|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2".to_string()), "-2.5|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("     -2.50".to_string()),
            "-2.5|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-0002.50".to_string()),
            "-2.5|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50".to_string()),
            "-2.5|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50".to_string()),
            "-2.5| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-     2.50".to_string()),
            "-2.5|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("  -2.50   ".to_string()),
            "-2.5|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50     ".to_string()),
            "-2.5|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("00000-2.50".to_string()),
            "-2.5|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok(float_repr(-2.5)), "-2.5|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2.5".to_string()), "-2.5|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(-2.5, &spec), Ok("-2".to_string()), "-2.5|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.5000000000".to_string()),
            "-2.5|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.5".to_string()),
            "-2.5|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(-2.5, &spec),
            Ok("-2.50".to_string()),
            "-2.5|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(-2.5, &spec).is_err(), "-2.5|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.000".to_string()), "1e+300|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1,000,000,000,000,000,052,504,760,255,204,420,248,704,468,581,108,159,154,915,854,115,511,802,457,988,908,195,786,371,375,080,447,864,043,704,443,832,883,878,176,942,523,235,360,430,575,644,792,184,786,706,982,848,387,200,926,575,803,737,830,233,794,788,090,059,368,953,234,970,799,945,081,119,038,967,640,880,074,652,742,780,142,494,579,258,788,820,056,842,838,115,669,472,196,386,865,459,400,540,160.00".to_string()), "1e+300|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160".to_string()), "1e+300|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1.0e+300".to_string()),
            "1e+300|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1.00E+300".to_string()),
            "1e+300|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("100000000000000007629703079084894925347346855150656811701601734206211380288125794484142188964691784076639747577138548761372210387844799938291815611350519830750167649856488981626536368095414607314235151058373458986890825155659063617715863205282622390509284183439858617103083735673849899204570498157510656.000000%".to_string()), "1e+300|%");
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("100000000000000007629703079084894925347346855150656811701601734206211380288125794484142188964691784076639747577138548761372210387844799938291815611350519830750167649856488981626536368095414607314235151058373458986890825155659063617715863205282622390509284183439858617103083735673849899204570498157510656.0%".to_string()), "1e+300|.1%");
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.".to_string()), "1e+300|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1.00000e+300".to_string()),
            "1e+300|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|10.2f");
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|08.2f");
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("+1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|+.2f");
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok(" 1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300| .2f");
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|=10.2f");
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|^10.2f");
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|<10.2f");
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.00".to_string()), "1e+300|0>10.2f");
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok(float_repr(1e+300)),
            "1e+300|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1e+300".to_string()),
            "1e+300|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1,000,000,000,000,000,052,504,760,255,204,420,248,704,468,581,108,159,154,915,854,115,511,802,457,988,908,195,786,371,375,080,447,864,043,704,443,832,883,878,176,942,523,235,360,430,575,644,792,184,786,706,982,848,387,200,926,575,803,737,830,233,794,788,090,059,368,953,234,970,799,945,081,119,038,967,640,880,074,652,742,780,142,494,579,258,788,820,056,842,838,115,669,472,196,386,865,459,400,540,160".to_string()), "1e+300|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1000000000000000052504760255204420248704468581108159154915854115511802457988908195786371375080447864043704443832883878176942523235360430575644792184786706982848387200926575803737830233794788090059368953234970799945081119038967640880074652742780142494579258788820056842838115669472196386865459400540160.0000000000".to_string()), "1e+300|.10f");
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1e+300, &spec),
            Ok("1E+300".to_string()),
            "1e+300|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(1e+300, &spec), Ok("1_000_000_000_000_000_052_504_760_255_204_420_248_704_468_581_108_159_154_915_854_115_511_802_457_988_908_195_786_371_375_080_447_864_043_704_443_832_883_878_176_942_523_235_360_430_575_644_792_184_786_706_982_848_387_200_926_575_803_737_830_233_794_788_090_059_368_953_234_970_799_945_081_119_038_967_640_880_074_652_742_780_142_494_579_258_788_820_056_842_838_115_669_472_196_386_865_459_400_540_160.00".to_string()), "1e+300|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1e+300, &spec).is_err(), "1e+300|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("5e-324".to_string()),
            "5e-324|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.00".to_string()),
            "5e-324|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.000".to_string()),
            "5e-324|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.94066e-324".to_string()),
            "5e-324|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.94066e-324".to_string()),
            "5e-324|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.9407e-324".to_string()),
            "5e-324|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.00".to_string()),
            "5e-324|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0".to_string()),
            "5e-324|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.9e-324".to_string()),
            "5e-324|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.94E-324".to_string()),
            "5e-324|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.000000%".to_string()),
            "5e-324|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.0%".to_string()),
            "5e-324|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.".to_string()),
            "5e-324|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.94066e-324".to_string()),
            "5e-324|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("5e-324".to_string()),
            "5e-324|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("      0.00".to_string()),
            "5e-324|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("00000.00".to_string()),
            "5e-324|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("+0.00".to_string()),
            "5e-324|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(" 0.00".to_string()),
            "5e-324| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("      0.00".to_string()),
            "5e-324|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("   0.00   ".to_string()),
            "5e-324|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.00      ".to_string()),
            "5e-324|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0000000.00".to_string()),
            "5e-324|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok(float_repr(5e-324)),
            "5e-324|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.94066e-324".to_string()),
            "5e-324|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0".to_string()),
            "5e-324|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.0000000000".to_string()),
            "5e-324|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("4.9E-324".to_string()),
            "5e-324|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(5e-324, &spec),
            Ok("0.00".to_string()),
            "5e-324|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(5e-324, &spec).is_err(), "5e-324|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.30000000000000004".to_string()),
            "0.30000000000000004|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.30".to_string()),
            "0.30000000000000004|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.300".to_string()),
            "0.30000000000000004|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.30".to_string()),
            "0.30000000000000004|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0".to_string()),
            "0.30000000000000004|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("3.0e-01".to_string()),
            "0.30000000000000004|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("3.00E-01".to_string()),
            "0.30000000000000004|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("30.000000%".to_string()),
            "0.30000000000000004|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("30.0%".to_string()),
            "0.30000000000000004|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.".to_string()),
            "0.30000000000000004|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.300000".to_string()),
            "0.30000000000000004|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("      0.30".to_string()),
            "0.30000000000000004|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("00000.30".to_string()),
            "0.30000000000000004|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("+0.30".to_string()),
            "0.30000000000000004|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(" 0.30".to_string()),
            "0.30000000000000004| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("      0.30".to_string()),
            "0.30000000000000004|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("   0.30   ".to_string()),
            "0.30000000000000004|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.30      ".to_string()),
            "0.30000000000000004|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0000000.30".to_string()),
            "0.30000000000000004|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok(float_repr(0.30000000000000004)),
            "0.30000000000000004|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0".to_string()),
            "0.30000000000000004|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3000000000".to_string()),
            "0.30000000000000004|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.3".to_string()),
            "0.30000000000000004|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(0.30000000000000004, &spec),
            Ok("0.30".to_string()),
            "0.30000000000000004|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(
                format_float(0.30000000000000004, &spec).is_err(),
                "0.30000000000000004|s"
            ),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999".to_string()),
            "41.9999|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.00".to_string()),
            "41.9999|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.000".to_string()),
            "41.9999|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999".to_string()),
            "41.9999|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999".to_string()),
            "41.9999|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42".to_string()),
            "41.9999|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.00".to_string()),
            "41.9999|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42".to_string()),
            "41.9999|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("4.2e+01".to_string()),
            "41.9999|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("4.20E+01".to_string()),
            "41.9999|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("4199.990000%".to_string()),
            "41.9999|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("4200.0%".to_string()),
            "41.9999|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.".to_string()),
            "41.9999|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999".to_string()),
            "41.9999|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("4e+01".to_string()),
            "41.9999|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("     42.00".to_string()),
            "41.9999|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("00042.00".to_string()),
            "41.9999|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("+42.00".to_string()),
            "41.9999|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(" 42.00".to_string()),
            "41.9999| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("     42.00".to_string()),
            "41.9999|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("  42.00   ".to_string()),
            "41.9999|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.00     ".to_string()),
            "41.9999|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("0000042.00".to_string()),
            "41.9999|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok(float_repr(41.9999)),
            "41.9999|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999".to_string()),
            "41.9999|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42".to_string()),
            "41.9999|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("41.9999000000".to_string()),
            "41.9999|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42".to_string()),
            "41.9999|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(41.9999, &spec),
            Ok("42.00".to_string()),
            "41.9999|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(41.9999, &spec).is_err(), "41.9999|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.6".to_string()),
            "5973.6|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.60".to_string()),
            "5973.6|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.600".to_string()),
            "5973.6|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.6".to_string()),
            "5973.6|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.6".to_string()),
            "5973.6|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5,973.6".to_string()),
            "5973.6|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5,973.60".to_string()),
            "5973.6|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5974".to_string()),
            "5973.6|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("6.0e+03".to_string()),
            "5973.6|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5.97E+03".to_string()),
            "5973.6|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("597360.000000%".to_string()),
            "5973.6|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("597360.0%".to_string()),
            "5973.6|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5974.".to_string()),
            "5973.6|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.60".to_string()),
            "5973.6|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("6e+03".to_string()),
            "5973.6|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("   5973.60".to_string()),
            "5973.6|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("05973.60".to_string()),
            "5973.6|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("+5973.60".to_string()),
            "5973.6|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(" 5973.60".to_string()),
            "5973.6| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("   5973.60".to_string()),
            "5973.6|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(" 5973.60  ".to_string()),
            "5973.6|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.60   ".to_string()),
            "5973.6|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("0005973.60".to_string()),
            "5973.6|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok(float_repr(5973.6)),
            "5973.6|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.6".to_string()),
            "5973.6|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5,974".to_string()),
            "5973.6|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5973.6000000000".to_string()),
            "5973.6|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("6E+03".to_string()),
            "5973.6|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(5973.6, &spec),
            Ok("5_973.60".to_string()),
            "5973.6|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(5973.6, &spec).is_err(), "5973.6|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.0".to_string()),
            "1989100000.0|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.000".to_string()),
            "1989100000.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.9891e+09".to_string()),
            "1989100000.0|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.9891e+09".to_string()),
            "1989100000.0|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.9891e+09".to_string()),
            "1989100000.0|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1,989,100,000.00".to_string()),
            "1989100000.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000".to_string()),
            "1989100000.0|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("2.0e+09".to_string()),
            "1989100000.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.99E+09".to_string()),
            "1989100000.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("198910000000.000000%".to_string()),
            "1989100000.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("198910000000.0%".to_string()),
            "1989100000.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.".to_string()),
            "1989100000.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.98910e+09".to_string()),
            "1989100000.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("2e+09".to_string()),
            "1989100000.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("+1989100000.00".to_string()),
            "1989100000.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(" 1989100000.00".to_string()),
            "1989100000.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.00".to_string()),
            "1989100000.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok(float_repr(1989100000.0)),
            "1989100000.0|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1.9891e+09".to_string()),
            "1989100000.0|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1,989,100,000".to_string()),
            "1989100000.0|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1989100000.0000000000".to_string()),
            "1989100000.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("2E+09".to_string()),
            "1989100000.0|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(1989100000.0, &spec),
            Ok("1_989_100_000.00".to_string()),
            "1989100000.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(1989100000.0, &spec).is_err(), "1989100000.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.0".to_string()),
            "451.0|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.00".to_string()),
            "451.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.000".to_string()),
            "451.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok("451".to_string()), "451.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451".to_string()),
            "451.0|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451".to_string()),
            "451.0|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.00".to_string()),
            "451.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451".to_string()),
            "451.0|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("4.5e+02".to_string()),
            "451.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("4.51E+02".to_string()),
            "451.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("45100.000000%".to_string()),
            "451.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("45100.0%".to_string()),
            "451.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.".to_string()),
            "451.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.000".to_string()),
            "451.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("5e+02".to_string()),
            "451.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("    451.00".to_string()),
            "451.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("00451.00".to_string()),
            "451.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("+451.00".to_string()),
            "451.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(" 451.00".to_string()),
            "451.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("    451.00".to_string()),
            "451.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("  451.00  ".to_string()),
            "451.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.00    ".to_string()),
            "451.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("0000451.00".to_string()),
            "451.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok(float_repr(451.0)), "451.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok(float_repr(451.0)), "451.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok(float_repr(451.0)), "451.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok(float_repr(451.0)), "451.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok(float_repr(451.0)),
            "451.0|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok(float_repr(451.0)), "451.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(451.0, &spec), Ok("451".to_string()), "451.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451".to_string()),
            "451.0|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.0000000000".to_string()),
            "451.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("4.5E+02".to_string()),
            "451.0|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(451.0, &spec),
            Ok("451.00".to_string()),
            "451.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(451.0, &spec).is_err(), "451.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.0".to_string()), "2.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.00".to_string()), "2.0|.2f");
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.000".to_string()), "2.0|.3f");
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.00".to_string()), "2.0|,.2f");
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("2.0e+00".to_string()),
            "2.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("2.00E+00".to_string()),
            "2.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("200.000000%".to_string()),
            "2.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("200.0%".to_string()),
            "2.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.".to_string()), "2.0|#.0f");
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("2.00000".to_string()),
            "2.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|.0g");
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("      2.00".to_string()),
            "2.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("00002.00".to_string()),
            "2.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("+2.00".to_string()),
            "2.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok(" 2.00".to_string()),
            "2.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("      2.00".to_string()),
            "2.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("   2.00   ".to_string()),
            "2.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("2.00      ".to_string()),
            "2.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("0000002.00".to_string()),
            "2.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok(float_repr(2.0)), "2.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(2.0, &spec),
            Ok("2.0000000000".to_string()),
            "2.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2".to_string()), "2.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(format_float(2.0, &spec), Ok("2.00".to_string()), "2.0|_.2f");
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(2.0, &spec).is_err(), "2.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.0".to_string()),
            "6000.0|"
        );
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.00".to_string()),
            "6000.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.000".to_string()),
            "6000.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000".to_string()),
            "6000.0|g"
        );
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000".to_string()),
            "6000.0|.6g"
        );
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6,000".to_string()),
            "6000.0|,.5g"
        );
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6,000.00".to_string()),
            "6000.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000".to_string()),
            "6000.0|.0f"
        );
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6.0e+03".to_string()),
            "6000.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6.00E+03".to_string()),
            "6000.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("600000.000000%".to_string()),
            "6000.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("600000.0%".to_string()),
            "6000.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.".to_string()),
            "6000.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.00".to_string()),
            "6000.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6e+03".to_string()),
            "6000.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("   6000.00".to_string()),
            "6000.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("06000.00".to_string()),
            "6000.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("+6000.00".to_string()),
            "6000.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(" 6000.00".to_string()),
            "6000.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("   6000.00".to_string()),
            "6000.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(" 6000.00  ".to_string()),
            "6000.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.00   ".to_string()),
            "6000.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("0006000.00".to_string()),
            "6000.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|,d"
        );
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|+d"
        );
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|05d"
        );
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|x"
        );
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|#x"
        );
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|X"
        );
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|o"
        );
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|#o"
        );
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|b"
        );
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|#b"
        );
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok(float_repr(6000.0)),
            "6000.0|c"
        );
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000".to_string()),
            "6000.0|n"
        );
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6,000".to_string()),
            "6000.0|,.0f"
        );
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6000.0000000000".to_string()),
            "6000.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6E+03".to_string()),
            "6000.0|.2G"
        );
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(6000.0, &spec),
            Ok("6_000.00".to_string()),
            "6000.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(6000.0, &spec).is_err(), "6000.0|s"),
            Err(_) => {}
        }
    }
    {
        let spec = parse_spec("").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74.0".to_string()), "74.0|");
    }
    {
        let spec = parse_spec(".2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.00".to_string()),
            "74.0|.2f"
        );
    }
    {
        let spec = parse_spec(".3f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.000".to_string()),
            "74.0|.3f"
        );
    }
    {
        let spec = parse_spec("g").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|g");
    }
    {
        let spec = parse_spec(".6g").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|.6g");
    }
    {
        let spec = parse_spec(",.5g").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|,.5g");
    }
    {
        let spec = parse_spec(",.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.00".to_string()),
            "74.0|,.2f"
        );
    }
    {
        let spec = parse_spec(".0f").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|.0f");
    }
    {
        let spec = parse_spec(".1e").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("7.4e+01".to_string()),
            "74.0|.1e"
        );
    }
    {
        let spec = parse_spec(".2E").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("7.40E+01".to_string()),
            "74.0|.2E"
        );
    }
    {
        let spec = parse_spec("%").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("7400.000000%".to_string()),
            "74.0|%"
        );
    }
    {
        let spec = parse_spec(".1%").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("7400.0%".to_string()),
            "74.0|.1%"
        );
    }
    {
        let spec = parse_spec("#.0f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.".to_string()),
            "74.0|#.0f"
        );
    }
    {
        let spec = parse_spec("#.6g").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.0000".to_string()),
            "74.0|#.6g"
        );
    }
    {
        let spec = parse_spec(".0g").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("7e+01".to_string()),
            "74.0|.0g"
        );
    }
    {
        let spec = parse_spec("10.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("     74.00".to_string()),
            "74.0|10.2f"
        );
    }
    {
        let spec = parse_spec("08.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("00074.00".to_string()),
            "74.0|08.2f"
        );
    }
    {
        let spec = parse_spec("+.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("+74.00".to_string()),
            "74.0|+.2f"
        );
    }
    {
        let spec = parse_spec(" .2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok(" 74.00".to_string()),
            "74.0| .2f"
        );
    }
    {
        let spec = parse_spec("=10.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("     74.00".to_string()),
            "74.0|=10.2f"
        );
    }
    {
        let spec = parse_spec("^10.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("  74.00   ".to_string()),
            "74.0|^10.2f"
        );
    }
    {
        let spec = parse_spec("<10.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.00     ".to_string()),
            "74.0|<10.2f"
        );
    }
    {
        let spec = parse_spec("0>10.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("0000074.00".to_string()),
            "74.0|0>10.2f"
        );
    }
    {
        let spec = parse_spec(",d").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|,d");
    }
    {
        let spec = parse_spec("+d").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|+d");
    }
    {
        let spec = parse_spec("05d").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|05d");
    }
    {
        let spec = parse_spec("x").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|x");
    }
    {
        let spec = parse_spec("#x").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|#x");
    }
    {
        let spec = parse_spec("X").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|X");
    }
    {
        let spec = parse_spec("o").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|o");
    }
    {
        let spec = parse_spec("#o").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|#o");
    }
    {
        let spec = parse_spec("b").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|b");
    }
    {
        let spec = parse_spec("#b").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|#b");
    }
    {
        let spec = parse_spec("c").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok(float_repr(74.0)), "74.0|c");
    }
    {
        let spec = parse_spec("n").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|n");
    }
    {
        let spec = parse_spec(",.0f").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|,.0f");
    }
    {
        let spec = parse_spec(".10f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.0000000000".to_string()),
            "74.0|.10f"
        );
    }
    {
        let spec = parse_spec(".2G").unwrap();
        assert_eq!(format_float(74.0, &spec), Ok("74".to_string()), "74.0|.2G");
    }
    {
        let spec = parse_spec("_.2f").unwrap();
        assert_eq!(
            format_float(74.0, &spec),
            Ok("74.00".to_string()),
            "74.0|_.2f"
        );
    }
    {
        match parse_spec("s") {
            Ok(spec) => assert!(format_float(74.0, &spec).is_err(), "74.0|s"),
            Err(_) => {}
        }
    }
}
