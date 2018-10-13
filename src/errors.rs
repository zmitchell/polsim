error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {

        MissingParameter(p: String) {
            description("missing parameter"),
            display("missing parameter in definition: '{}'", p),
        }

        ExtraParameter(m: String) {
            description("extra parameter"),
            display("extra parameter in definition: '{}'", m),
        }

        InvalidValue(m: String) {
            description("invalid value"),
            display("{}", m),
        }

        WrongElementType(m: String) {
            description("attempted to validate wrong element type"),
            display("{}", m),
        }

        WrongPolarizationType(m: String) {
            description("attempted to validate a beam with the wrong polarization type"),
            display("{}", m)
        }
    }
}
