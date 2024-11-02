mod evaluation{
    use string_mult::evaluate;

    #[test]
    fn simple_mult() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l\"\t\t\t*4")?;
        assert_eq!(data, "12packs, 10mg/l");
        Ok(())
    }

    #[test]
    fn simple_param_zero_is_same_as_simple() -> anyhow::Result<()>{
        assert_eq!(
            evaluate("\"3packs, 10mg/l\"   \t\t\t*[0] 4").unwrap(),
            evaluate("\"3packs, 10mg/l\"       \t*  \t4").unwrap()
        );

        Ok(())
    }

    #[test]
    fn simple_mult_complex_param() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l; 7,8,9,10\"\t\t\t*[-2] 6")?;
        assert_eq!(data, "3packs, 10mg/l; 7,8,54,10");
        Ok(())
    }


    #[test]
    fn simple_mult_with_not_int() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l\"\t\t\t*-4.1")?;
        assert_eq!(data, "-12.3packs, 10mg/l");
        Ok(())
    }

    #[test]
    fn simple_mult_with_param() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l; 7 packs...\"\t\t\t*[1] 6")?;
        assert_eq!(data, "3packs, 60mg/l; 7 packs...");
        Ok(())
    }

    #[test]
    fn simple_mult_with_neg_param() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l; 7,8,9\"   \t\t*[-4]\t-6.1")?;
        assert_eq!(data, "3packs, -61mg/l; 7,8,9");
        Ok(())
    }

    #[test]
    fn multiply_all_by_int() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l; 7,8,9\"  \t\t**3")?;
        assert_eq!(data, "9packs, 30mg/l; 21,24,27");
        Ok(())
    }

    #[test]
    fn multiply_all_by_float() -> anyhow::Result<()>{
        let data = evaluate("\"3packs, 10mg/l; 7,8,9\"   \t\t** 3.5")?;
        assert_eq!(data, "10.5packs, 35mg/l; 24.5,28,31.5");
        Ok(())
    }

    #[test]
    fn duplicate() -> anyhow::Result<()>{
        let data = evaluate("\"123\"   \t\t***3")?;
        assert_eq!(data, "123123123");
        Ok(())
    }

    #[test]
    fn duplicate_negative() -> anyhow::Result<()>{
        let data = evaluate("\"123\"   \t\t***-3")?;
        assert_eq!(data, "321321321");
        Ok(())
    }

    #[test]
    fn duplicate_zero() -> anyhow::Result<()>{
        let data = evaluate("\"123abcde\"   \t\t***0")?;
        assert_eq!(data, "");
        Ok(())
    }

}