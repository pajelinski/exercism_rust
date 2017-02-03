pub fn reply(message: &'static str) -> &'static str {
        if message.is_empty() { return "Fine. Be that way!"; }
        if message == message.to_uppercase() { return "Whoa, chill out!";}
        if message.ends_with("?") { return "Sure.";}

        "Whatever."
}
