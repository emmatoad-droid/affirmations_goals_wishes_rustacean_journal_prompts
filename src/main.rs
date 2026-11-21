use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const GOALS: [&str; 10] = [
    "Seed: something I want to plant this month and not tell anyone about",
    "Something I keep watering that never grows",
    "A wild thing I'd let take over the whole garden",
    "What would I love to see in full bloom by next spring?",
    "A weed I'm ready to pull out of my life",
    "Something that grows best in the dark",
    "What has outgrown its pot and needs repotting?",
    "A seed I've been saving for the right time",
    "Something I'd grow purely for the smell",
    "If my year was a garden, what's the daft bit in the corner?",
];

const LESSONS: [&str; 10] = [
    "Waning moon: what am I letting get smaller?",
    "Something that only made sense once I saw it from far away",
    "A lesson that arrived like a comet and left just as fast",
    "Who or what am I orbiting lately?",
    "Something I learned in the dark",
    "Old light: a lesson that took years to reach me",
    "New moon: what don't I understand yet?",
    "Which star would I steer by, and who gave it to me?",
    "An eclipse: when something obvious got hidden, what showed up instead?",
    "What would the last full moon say it caught me doing?",
];

const AFFIRMATIONS: [&str; 10] = [
    "I do not have to be useful to be valuable.",
    "My curiosity is not frivolous.",
    "I can be both knowledgeable and inexperienced.",
    "I can change without disowning the person I used to be.",
    "There is no shame in beginning something late.",
    "I am allowed to have interests that serve absolutely no purpose.",
    "My life is not a competition with somebody else's timeline.",
    "I can take something seriously without taking myself seriously.",
    "I don't have to turn every difficult experience into something productive.",
    "There is room in my life for both ambition and silliness.",
];

const WILDCARDS: [&str; 10] = [
    "Objects that deserve a biography",
    "Things I find fascinating for no good reason",
    "Unpopular opinions I will defend",
    "Things that should exist but don't",
    "My personal museum of insignificant things",
    "Conversations I'd love to have with someone dead",
    "Things I refuse to become too sensible for",
    "An entirely unnecessary theory about something",
    "What I would put in a time capsule",
    "Things that make me feel very much alive",
];

const WILDCARD_EVERY: usize = 5;

fn nanos() -> usize {
    let mut n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;
    n ^= n >> 12;
    n ^= n << 25;
    n ^= n >> 27;
    n
}

fn read_vault(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect()
}

fn append_line(path: &str, line: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", line)
}

fn pick<'a>(prompts: &[&'a str], last: &str) -> &'a str {
    let mut idx = nanos() % prompts.len();
    if last.ends_with(prompts[idx]) {
        idx = (idx + 1) % prompts.len();
    }
    prompts[idx]
}

fn simple_machine(icon: &str, title: &str, path: &str, prompts: &[&str]) -> io::Result<()> {
    let vault = read_vault(path);
    let number = vault.len() + 1;
    let last = vault.last().map(|s| s.as_str()).unwrap_or("");

    let prompt = pick(prompts, last);
    append_line(path, &format!("{}. {}", number, prompt))?;

    println!("\n{}✨ {} #{} ✨{}", icon, title, number, icon);
    println!("{}", prompt);
    Ok(())
}

fn affirmation_machine() -> io::Result<()> {
    let path = "affirmation_deposits.txt";
    let vault = read_vault(path);
    let number = vault.len() + 1;

    if number % WILDCARD_EVERY == 0 {
        let idx = (number / WILDCARD_EVERY - 1) % WILDCARDS.len();
        let text = WILDCARDS[idx];
        append_line(path, &format!("{}. [WILDCARD] {}", number, text))?;

        println!("\n🦋✨ WILDCARD PAGE! Deposit #{} ✨🦋", number);
        println!("Title for a page, then just let go: \"{}\"", text);
    } else {
        let last = vault
            .iter()
            .rev()
            .find(|l| !l.contains("[WILDCARD]"))
            .map(|s| s.as_str())
            .unwrap_or("");

        let text = pick(&AFFIRMATIONS, last);
        append_line(path, &format!("{}. {}", number, text))?;

        println!("\n🐸✨ Deposit #{} ✨🐸", number);
        println!("{}", text);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("🎰🐸 CHAOS GREMLIN VENDING MACHINE 🐸🎰");
    println!("  1 = 🌱 Botanical Goals");
    println!("  2 = 🌙 Celestial Lessons");
    println!("  3 = ❤️  Affirmations");
    println!("  (or just press Enter for a surprise)");
    print!("\nPick a button: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let button = match choice.trim() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => nanos() % 3 + 1,
    };

    match button {
        1 => simple_machine("🌱", "BOTANICAL GOALS", "botanical_goals.txt", &GOALS),
        2 => simple_machine("🌙", "CELESTIAL LESSONS", "celestial_lessons.txt", &LESSONS),
        _ => affirmation_machine(),
    }
}

