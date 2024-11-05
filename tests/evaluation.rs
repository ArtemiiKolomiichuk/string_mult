mod evaluation {
    use string_mult::evaluate;
    mod multiplication {
        use super::*;

        #[test]
        fn simple_mult() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l\"\t\t\t*4")?;
            assert_eq!(data, "12packs, 10mg/l");
            Ok(())
        }

        #[test]
        fn simple_mult_param_zero_is_same_as_simple() -> anyhow::Result<()> {
            assert_eq!(
                evaluate("\"3packs, 10mg/l\"   \t\t\t*[0] 4").unwrap(),
                evaluate("\"3packs, 10mg/l\"       \t*  \t4").unwrap()
            );

            Ok(())
        }

        #[test]
        fn simple_mult_with_param() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l; 7 packs...\"\t\t\t*[1] 6")?;
            assert_eq!(data, "3packs, 60mg/l; 7 packs...");
            Ok(())
        }

        #[test]
        fn simple_mult_with_not_int() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l\"\t\t\t*-4.1")?;
            assert_eq!(data, "-12.3packs, 10mg/l");
            Ok(())
        }

        #[test]
        fn simple_mult_negative_param() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l; 7,8,9\"   \t\t*[-4]\t-6.1")?;
            assert_eq!(data, "3packs, -61mg/l; 7,8,9");
            Ok(())
        }

        #[test]
        fn multiply_all_by_int() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l; 7,8,9\"  \t\t**3")?;
            assert_eq!(data, "9packs, 30mg/l; 21,24,27");
            Ok(())
        }

        #[test]
        fn multiply_all_by_float() -> anyhow::Result<()> {
            let data = evaluate("\"3packs, 10mg/l; 7,8,9\"   \t\t** 3.5")?;
            assert_eq!(data, "10.5packs, 35mg/l; 24.5,28,31.5");
            Ok(())
        }
    }

    mod duplication {
        use super::*;

        #[test]
        fn duplicate() -> anyhow::Result<()> {
            let data = evaluate("\"123\"   \t\t***3")?;
            assert_eq!(data, "123123123");
            Ok(())
        }

        #[test]
        fn duplicate_negative() -> anyhow::Result<()> {
            let data = evaluate("\"123\"   \t\t***-3")?;
            assert_eq!(data, "321321321");
            Ok(())
        }

        #[test]
        fn duplicate_zero() -> anyhow::Result<()> {
            let data = evaluate("\"123abcde\"   \t\t***0")?;
            assert_eq!(data, "");
            Ok(())
        }
    }

    #[test]
    fn two_operations() -> anyhow::Result<()> {
        let data = evaluate("\"123abcdef\"   \t\t***2  *[1] 2")?;
        assert_eq!(data, "123abcdef246abcdef");
        Ok(())
    }

    #[test]
    fn three_operations() -> anyhow::Result<()> {
        let data = evaluate("\"123abcdef\"   \t\t***2  * 2 ***2")?;
        assert_eq!(data, "246abcdef123abcdef246abcdef123abcdef");
        Ok(())
    }

    #[test]
    fn many_operations() -> anyhow::Result<()> {
        let data = evaluate("  \"1a\"   \t\t***2  * 2 ***2  *[2] 3 ** 2\t\t ***2 *[-2] 10.1 ")?;
        assert_eq!(data, "4a2a12a2a4a2a121.2a2a");
        Ok(())
    }

    #[test]
    fn short_command_is_err() {
        let res = evaluate("\"5\"***");
        println!("{:?}", res);
        assert!(res.is_err());
    }

    #[test]
    fn command_list_evaluates() -> anyhow::Result<()> {
        let data = "\"12 packs\" *** 3 *2\n \"4packs\" *[2]2 \"19 bottles.\" **3";
        let results = string_mult::evaluate_list(data)?;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().unwrap(), "24 packs12 packs12 packs");
        assert!(results[1].is_err());
        assert_eq!(results[2].as_ref().unwrap(), "57 bottles.");
        Ok(())
    }

}
