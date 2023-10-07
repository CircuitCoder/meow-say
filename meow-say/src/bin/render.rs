use clap::Parser;
use image::Rgb;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    text: String,

    #[arg(short, long)]
    img: String,

    #[arg(short, long, default_value="#FFFFFF", value_parser=meow_say::try_parse_color)]
    color: Rgb<u8>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let img = meow_say::img::dynamic(&args.img)?;
    let rendered = meow_say::draw::draw(&args.text, &img, args.color)?;

    rendered.save("./output.png")?;

    Ok(())
}
