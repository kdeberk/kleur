error_chain!{
    foreign_links {
        Io(::std::io::Error);
        DeserializeYaml(::serde_yaml::Error);
        MissingHomeEnv(::std::env::VarError);
        RegexError(::regex::Error);
    }
}
