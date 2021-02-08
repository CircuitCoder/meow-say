use image::Rgb;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    text: String,

    #[structopt(short, long)]
    img: String,

    #[structopt(short, long, default_value="#FFFFFF", parse(try_from_str=meow_say::try_parse_color))]
    color: Rgb<u8>,
}


#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let img = meow_say::img::dynamic(&args.img)?;
    let rendered = meow_say::draw::draw(&args.text, &img, args.color)?;

    rendered.save("./output.png")?;

    Ok(())
}