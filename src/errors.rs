error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {

        MissingParameter(p: String) {
            description("missing parameter"),
            display("missing parameter '{}'", p),
        }

        ExtraParameter(p: String) {
            description("extra parameter"),
            display("extra parameter '{}'", p),
        }

        InvalidValue(m: String) {
            description("invalid value"),
            display("{}", m),
        }

    }
}
