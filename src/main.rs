use std::io;
use tui::Terminal;
use tui::backend::{Backend};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};


fn main() -> Result<(), io::Error> {

    #[cfg(target_os = "windows")] let mut term = {
        use tui::backend::CrosstermBackend;

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Terminal::new(backend)?
    };

    #[cfg(any(target_os = "macos", target_os = "linux"))] let mut term = {
        use termion::raw::IntoRawMode;
        use tui::backend::TermionBackend;

        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
    };

    term.clear();

    let mut tick : u64 = 0;
    let chars = ['/', '|', '\\'];
    const speed : u32 = 50;

    loop {
        let index = (tick / speed as u64) % 3;

        term.draw(|f| {
            let size = f.size();
            let mut block = Block::default()
                .title(format!("Loading {}", chars[index as usize]))
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        tick += 1;
    }


    Ok(())
}
