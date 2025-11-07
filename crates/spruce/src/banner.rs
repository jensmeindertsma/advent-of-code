use core::fmt::{self, Formatter};
use owo_colors::OwoColorize;
use terminal_size::{Width, terminal_size};

pub enum Banner {
    Snowman,
    Village,
}

impl fmt::Display for Banner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (Width(width), _) = terminal_size().expect("terminal size should be available");

        match self {
            Self::Snowman => {
                // If the terminal is too narrow the message will
                // render wholly screwed up. So we skip the message
                // and render only the snowman
                if width > 100 {
                    let line_one = format!(
                    "     {}          {}",
                    "__".bold().red(),
                    " _   _                              _   _       _ _     _                      _"
                        .bold()
                );
                    let line_two = format!(
                    "   {}        {}",
                    "_|==|_".bold().red(),
                    "| | | | __ _ _ __  _ __  _   _     | | | | ___ | (_) __| | __ _ _   _ ___     | |"
                        .bold()
                );
                    let line_three = format!(
                    "    {}{}     {}",
                    "('')".bold().white(),
                    "___/".bold().yellow(),
                    "| |_| |/ _` | '_ \\| '_ \\| | | |    | |_| |/ _ \\| | |/ _` |/ _` | | | / __|    | |"
                        .bold()
                );
                    let line_four = format!(
                    "{}{}        {}",
                    ">--".bold().yellow(),
                    "(`^^')".bold().white(),
                    "|  _  | (_| | |_) | |_) | |_| |    |  _  | (_) | | | (_| | (_| | |_| \\__ \\    | |"
                        .bold()
                );
                    let line_five = format!(
                    "  {}       {}",
                    "(`^'^'`)".bold().white(),
                    "|_| |_|\\__,_| .__/| .__/ \\__, |    |_| |_|\\___/|_|_|\\__,_|\\__,_|\\__, |___/    |_|"
                        .bold()
                );
                    let line_six = format!(
                    "  {}      {}",
                    "`======' ".bold().white(),
                    "            |_|   |_|    |___/                                  |___/         (_)"
                        .bold()
                );

                    write!(
                        f,
                        "{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}"
                    )
                } else {
                    let line_one = format!("     {}", "__".bold().red(),);
                    let line_two = format!("   {}", "_|==|_".bold().red(),);
                    let line_three =
                        format!("    {}{}", "('')".bold().white(), "___/".bold().yellow(),);
                    let line_four =
                        format!("{}{}", ">--".bold().yellow(), "(`^^')".bold().white(),);
                    let line_five = format!("  {}", "(`^'^'`)".bold().white(),);
                    let line_six = format!("  {}", "`======' ".bold().white(),);

                    write!(
                        f,
                        "{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}"
                    )
                }
            }

            Self::Village => {
                let line_one = format!(
                    "   {}                                                   {}",
                    ".-.".bold().yellow(),
                    "\\ /".bold().yellow()
                );
                let line_two = format!(
                    "  {}                                {}                  {} {} {}",
                    "( (".bold().yellow(),
                    "|".bold().black(),
                    "-".bold().yellow(),
                    "*".bold().red(),
                    "-".bold().yellow()
                );
                let line_three = format!(
                    "   {}                              {}                  {}",
                    "'-`".bold().yellow(),
                    "-+-".bold().black(),
                    "/ \\".bold().yellow()
                );
                let line_four = format!(
                    "            {}            {}          {}{}{}          {}",
                    "\\".bold().black(),
                    "o".bold().yellow(),
                    "_".bold().red(),
                    "|".bold().black(),
                    "_".bold().red(),
                    "\\".bold().black()
                );
                let line_five = format!(
                    "            {}          {}        {}         {}",
                    "))".bold().black(),
                    "{^}".bold().green(),
                    "/___\\".bold().red(),
                    "))".bold().black()
                );
                let line_six = format!(
                    "          {}{}{}     {}     {}    {}{}{}",
                    ".-".bold().red(),
                    "#".bold().black(),
                    "-----.".bold().red(),
                    "/|\\".bold().green(),
                    ".---'-'---.".bold().red(),
                    ".-".bold().red(),
                    "#".bold().black(),
                    "-----.".bold().red()
                );
                let line_seven = format!(
                    "     {}   {}   {}  {}",
                    "___ /_________\\".bold().red(),
                    "//|\\\\".bold().green(),
                    "/___________\\".bold().red(),
                    "/_________\\".bold().red()
                );
                let line_eight = format!(
                    "    {} {}{} {} {}{}    {}    {} {} {} {} {}    {} {}{}{} {} {}",
                    "/___\\".bold().red(),
                    "|".bold().yellow(),
                    "[]".bold().blue(),
                    "_".bold().yellow(),
                    "[]".bold().blue(),
                    "|".bold().yellow(),
                    "//|\\\\".bold().green(),
                    "|".bold().yellow(),
                    "A".bold().blue(),
                    "/^\\".bold().yellow(),
                    "A".bold().blue(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "[]".bold().blue(),
                    "_".bold().yellow(),
                    "[]".bold().blue(),
                    "|".bold().yellow(),
                    "_.O,_".bold().white()
                );
                let line_nine = format!(
                    "{}{}{}{}{}{}{}{}  {}{}{}  {}{}{}{}{}   {}{}{}   {}{}{}  {}{}{}  {}{}",
                    "....".bold().white(),
                    "|".bold().yellow(),
                    "\"".bold().blue(),
                    "#".bold().black(),
                    "\"".bold().blue(),
                    "|".bold().yellow(),
                    ".".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "*".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "...".bold().white(),
                    "///|\\\\\\".bold().green(),
                    "...".bold().white(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "\"".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "....".bold().white(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "*".bold().black(),
                    "|".bold().yellow(),
                    "|".bold().yellow(),
                    "..(^)....".bold().white()
                );

                write!(
                    f,
                    "{line_one}\n{line_two}\n{line_three}\n{line_four}\n{line_five}\n{line_six}\n{line_seven}\n{line_eight}\n{line_nine}"
                )
            }
        }
    }
}
