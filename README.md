🎰🐸 Affirmations, Goals & Wishes

A Rustacean’s Journal Prompt Machine

A tiny interactive journal prompt generator written in Rust.

Part vending machine, part digital notebook, part creative experiment.

Choose a category, receive a prompt, and let the machine make a little deposit in your personal text-file vault. Every interaction adds to a growing record of ideas, questions and things worth thinking about.

Built with Rust’s standard library. No external crates. No productivity guru required.

⸻

🌱 The three machines

The vending machine has three buttons:

Button	Machine	What it dispenses
🌱 1	Botanical Goals	Prompts about growth, planting ideas, ambitions and things you’d like to cultivate
🌙 2	Celestial Lessons	Reflections inspired by moons, stars, eclipses, distance and perspective
❤️ 3	Affirmations	Gentle reminders about curiosity, self-worth, beginnings and being human

Press Enter instead of selecting a button and the machine chooses a category for you.

🦋 The wildcard mechanism

The affirmation machine has a small surprise built into its state.

Every fifth affirmation deposit, it interrupts the usual routine and dispenses a wildcard page title instead.

Examples:

* Objects that deserve a biography
* Things I find fascinating for no good reason
* Unpopular opinions I will defend
* My personal museum of insignificant things
* Conversations I’d love to have with someone dead

There are ten wildcard titles. They cycle in sequence, creating an occasional invitation to abandon the original question and follow an entirely different train of thought.

💾 Persistent memory: the vaults

The machine doesn’t forget what it dispenses.

Each category maintains its own plain-text vault:

botanical_goals.txt
celestial_lessons.txt
affirmation_deposits.txt

Every prompt is appended as a numbered entry.

For example:

1. A wild thing I'd let take over the whole garden
2. What would I love to see in full bloom by next spring?
3. Something I keep watering that never grows

The numbering continues across sessions because the machine reads the existing file before making its next deposit.

The affirmation vault also remembers its previous non-wildcard prompt, so it can avoid serving the same affirmation twice consecutively.

Your journal history stays on your own machine.

⚙️ Under the bonnet

This project explores several fundamental Rust concepts:

Concept	Implementation
Constants	Fixed-size arrays of static prompts
Functions	Small, reusable operations for each machine
Slices	Passing prompt collections into shared functions
File I/O	Reading and appending persistent vault files
Error handling	io::Result and the ? operator
Iterators	Filtering and searching previous deposits
Pattern matching	Selecting the machine from terminal input
State	Deposit numbering and wildcard scheduling
Time	Lightweight clock-derived prompt selection

Random selection

Prompt selection uses the current system time in nanoseconds, with simple bitwise operations to scramble the value.

This avoids an external random-number dependency.

It is intended for a small creative application, not security-sensitive randomness.

State and persistence

The machine reconstructs its state from the vault files each time it runs.

There is no database, account system or external service.

Just Rust and ordinary text files.

🚀 Running the project

You’ll need an installed Rust toolchain.

Clone the repository:

git clone https://github.com/emmatoad-droid/affirmations_goals_wishes_rustacean_journal_prompts.git

Enter the project directory:

cd affirmations_goals_wishes_rustacean_journal_prompts

Run:

cargo run

Or check compilation:

cargo check

No external dependencies are required.

📁 Project structure

affirmations_goals_wishes_rustacean_journal_prompts/
│
├── Cargo.toml
├── README.md
├── .gitignore
│
└── src/
    └── main.rs

The three vault files are generated automatically in the working directory when the corresponding machine is used.

🛠️ Possible future experiments

* An interactive loop for repeated vending.
* A command-line option to inspect previous deposits.
* Additional prompt collections.
* A moth-themed machine.
* A way to export or browse the journal history.

These are possibilities, not prerequisites. The little machine already has a job.

🐸 About

Made by Emma — self-taught Rustacean, zine maker and founder of Chaos Gremlin Press.

This project grew out of three physical embossed notebooks: Goals, Lessons and Affirmations.

The idea was to translate that creative practice into a small piece of software: something that could generate a question, preserve the answer’s starting point and make room for unexpected ideas.

Chaos Gremlin Press brings together zines, poetry, code and collaborative fiction.

Sometimes a garden is a data structure.

Sometimes a journal is a vending machine.

Sometimes you just need to press a button and see what comes out.

Built with Rust. Powered by curiosity. Operated by frogs. 🐸

⸻
