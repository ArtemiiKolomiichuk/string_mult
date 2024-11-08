use string_mult::parsing::*;
use string_mult::*;

mod parsing {
    use super::*;

    #[test]
    fn parsing_list() -> anyhow::Result<()> {
        let res = parse_list("\"15 packs, 10mg/l\" *[0] 100\n \"15 packs, 10mg/l\" *[1] 101");
        println!("{:#?}", res);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 2);
        assert!(res[0].is_ok());
        assert!(res[1].is_ok());
        Ok(())
    }

    #[test]
    fn parsing_list_with_errors() -> anyhow::Result<()> {
        let res = parse_list(
            "\"15 packs, 10mg/l\" *[0] 100\n \"15 packs, 10mg/l\" ***** 101\n\"3434\" **2 ",
        );
        println!("{:#?}", res);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 3);
        assert!(res[0].is_ok());
        assert!(res[1].is_err());
        assert!(res[2].is_ok());
        Ok(())
    }

    #[test]
    fn parsing_command() -> anyhow::Result<()> {
        let res = parse_command("\"15 packs, 10mg/l\" *[1] 100");
        assert!(res.is_ok());

        let res = res.unwrap();
        assert_eq!(res.params.len(), 4);
        assert_eq!(res.params[0], ParamsPiece::Num(15_f64));
        assert_eq!(res.params[1], ParamsPiece::Str(" packs, ".to_string()));
        assert_eq!(res.params[2], ParamsPiece::Num(10_f64));
        assert_eq!(res.params[3], ParamsPiece::Str("mg/l".to_string()));

        assert_eq!(res.operations.len(), 1);
        assert_eq!(
            res.operations[0].operation_type,
            OperationType::Mult(Some(1))
        );
        assert_eq!(res.operations[0].argument.right(), Some(100_f64));
        Ok(())
    }

    #[test]
    fn no_valid_command() -> anyhow::Result<()> {
        let res = parse_command("\"15 packs, 10mg/l *[1]\" ***** 10");
        assert!(res.is_err());
        return if let Err(ParseError::WrongCommand(_)) = res {
            Ok(())
        } else {
            Err(anyhow::anyhow!("wrong error"))
        };
    }

    #[test]
    fn wrong_command_in_list() -> anyhow::Result<()> {
        let res = parse_list("\"15 packs, 10mg/l *[1]\" ***** 10");
        assert!(res.is_ok());
        return if let Err(ParseError::WrongCommand(_)) = res.unwrap()[0] {
            Ok(())
        } else {
            Err(anyhow::anyhow!("wrong error"))
        };
    }

    #[test]
    fn argument_without_operation() -> anyhow::Result<()> {
        let res = parse_command("\"15 packs, 10mg/l\" 10 **");
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn int_too_long() -> anyhow::Result<()> {
        let res = parse_command("\"15 packs, 10mg/l\" *** 10000000000000000000000000000000000000000000000000000000000000");
        assert!(res.is_err());
        return if let Err(ParseError::ParseIntError(_)) = res {
            Ok(())
        } else {
            Err(anyhow::anyhow!("wrong error"))
        };
    }
}
