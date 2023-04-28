pub fn reply(message: &str) -> &str {
    if message.trim().chars().all(|x| x.is_whitespace()) {
        "Fine. Be that way!"
    } else if message.trim().chars().last().unwrap() == '?' {
        if !message.trim().chars().all(|x| !x.is_alphabetic())
            && message
                .trim()
                .chars()
                .all(|x| x.is_uppercase() || !x.is_alphabetic())
        {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if !message.trim().chars().all(|x| !x.is_alphabetic())
        && message
            .trim()
            .chars()
            .all(|x| x.is_uppercase() || !x.is_alphabetic())
    {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
