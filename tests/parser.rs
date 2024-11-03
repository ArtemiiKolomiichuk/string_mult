use pest::Parser;
use string_mult::*;

mod parser {
    use super::*;

    #[test]
    fn num() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::num, "-44.43,-15")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no field"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "-44.43");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn num_splits_on_spacing() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::num, "-4 4.43 -15")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no field"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "-4");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn wrong_num_is_err() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::num, "--4.- 4.43 -15");
        print!("{:?}", data);
        assert!(data.is_err());
        Ok(())
    }

    #[test]
    fn int() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::int, "-44,-15")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no int"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "-44");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn mult() -> anyhow::Result<()> {
        let mut data = Grammar::parse(Rule::mult, "*")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no mult"))?;
        println!("{:?}", data);
        assert_eq!(data.as_str(), "*");
        assert_eq!(data.as_span().start(), 0);

        data = Grammar::parse(Rule::mult, "*[3]")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no parametrized mult"))?;
        println!("{:?}", data);
        assert_eq!(data.as_str(), "*[3]");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn wrong_param_mult_is_mult() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::mult, "*[-3.5]")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no mult"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "*");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn mult_all() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::multAll, "*****")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no mult_all"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "**");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn duplicate() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::duplicate, "***")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no duplicate"))?;
        print!("{:?}", data);
        assert_eq!(data.as_str(), "***");
        assert_eq!(data.as_span().start(), 0);
        Ok(())
    }

    #[test]
    fn inner_str_text() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::inner_str_text, " abc -4def,--5\"")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no inner_str"))?;
        print!("{:#?}", data);
        assert_eq!(data.as_str(), " abc ");
        Ok(())
    }

    #[test]
    fn str_param_splits_numbers() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::str_param, "\"3 abc -4def,--5\"")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no str_param"))?;
        print!("{:#?}", data);
        let mut inner_pairs = data.into_inner();
        assert_eq!(inner_pairs.next().unwrap().as_str(), "3");
        assert_eq!(inner_pairs.next().unwrap().as_str(), " abc ");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "-4");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "def,-");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "-5");
        Ok(())
    }

    #[test]
    fn str_param_allows_numbers_abscence() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::str_param, "\"abc def\"")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no str_param"))?;
        print!("{:#?}", data);
        assert_eq!(data.as_str(), "\"abc def\"");
        Ok(())
    }

    #[test]
    fn command_ignores_spacing() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::command, "\"str\" \t\t  *[3]\t  \t\t-12.2")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no command"))?;
        print!("{:#?}", data);
        let mut inner_pairs = data.into_inner();
        assert_eq!(inner_pairs.next().unwrap().as_str(), "\"str\"");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "*[3]");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "-12.2");
        Ok(())
    }

    #[test]
    fn command_allows_multiple_operations() -> anyhow::Result<()> {
        let data = Grammar::parse(Rule::command, "\"str\" \t\t  *-12.2 \t*** 3")?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no command"))?;
        print!("{:#?}", data);
        let mut inner_pairs = data.into_inner();
        assert_eq!(inner_pairs.next().unwrap().as_str(), "\"str\"");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "*");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "-12.2");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "***");
        assert_eq!(inner_pairs.next().unwrap().as_str(), "3");
        Ok(())
    }
}
