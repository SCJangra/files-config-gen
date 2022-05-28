#[macro_export]
macro_rules! scan {
    ($prompt:literal, $type:ty) => {{
        let fun = || -> anyhow::Result<$type> {
            use anyhow::Context;
            use std::io::Write;

            print!($prompt);
            std::io::stdout()
                .flush()
                .with_context(|| "Could not write to stdout")?;

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .with_context(|| "Could not read from stdin")?;

            input
                .trim()
                .parse::<$type>()
                .with_context(|| format!("Invalid input type, expected {}", stringify!($type)))
        };

        fun()
    }};
}
