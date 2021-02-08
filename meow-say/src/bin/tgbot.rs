#![feature(async_closure)]

use std::{cmp::max, env};
use futures::StreamExt;

use image::{ColorType, EncodableLayout, Rgb, Rgba, png::PngEncoder};
use imageproc::{definitions::Image, geometric_transformations::{Interpolation, Projection, warp_into}};
use meow_say::img::{Img, PARSED_CATS};
use structopt::StructOpt;
use telegram_bot::*;
use tokio;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    null: i64,

    #[structopt(short, long, default_value="#FFFFFF", parse(try_from_str=meow_say::try_parse_color))]
    color: Rgb<u8>,
}

async fn process(api: Api, query: InlineQuery, cats: &Vec<Img>, color: Rgb<u8>, null: i64) -> anyhow::Result<()> {
    log::debug!("Query {:#?}", query);

    if query.query == "" {
        return Ok(())
    }

    let rendered: anyhow::Result<Vec<Vec<u8>>> = cats.iter().map(|c| -> Result<_, _> {
        let drawn = meow_say::draw::draw(
            &query.query,
            c,
            color,
        )?;
        let dim = drawn.dimensions();
        let max_dim = max(dim.0, dim.1);
        let scale = 512.0 / max_dim as f32;
        let mut alloc = Image::new((dim.0 as f32 *scale).round() as u32, (dim.1 as f32*scale).round() as u32);
        warp_into(&drawn, &Projection::scale(scale, scale), Interpolation::Bilinear, Rgba([0,0,0,0]), &mut alloc);

        let mut buffer = Vec::new();
        let encoder = PngEncoder::new(&mut buffer);
        let (w, h) = alloc.dimensions();
        encoder.encode(alloc.as_bytes(), w, h, ColorType::Rgba8)?;
        Ok(buffer)
    }).collect();

    let rendered = rendered?;

    // let user = &query.from;
    let api_move = &api;

    let ids = futures::future::join_all(rendered.into_iter().map(async move |r| -> anyhow::Result<Option<String>> {
        log::debug!("Uploading...");
        let upload = InputFileUpload::with_data(r, "Meow.png");

        let req = SendSticker::new(
            ChatId::new(null),
            upload,
        );
        let resp = api_move.send(req).await?;
        log::debug!("Send complete: {:#?}", resp);

        use MessageOrChannelPost::*;
        let kind = match resp {
            ChannelPost(p) => p.kind,
            Message(m) => m.kind,
        };

        if let MessageKind::Sticker { data } = kind {
            Ok(Some(data.file_id))
        } else {
            Ok(None)
        }
    })).await;

    let ids: Result<Vec<_>, _> = ids.into_iter().collect();
    let ids = ids?;
    let ids = ids.into_iter().filter_map(|e| e);

    let reply = query.answer(ids.enumerate().map(|(i, id)| InlineQueryResult::InlineQueryResultCachedSticker(InlineQueryResultCachedSticker {
        sticker_file_id: id,
        id: format!("{}", i),
        reply_markup: None,
        input_message_content: None,
    })).collect());
    api.send(reply).await?;

    Ok(())
}

#[paw::main]
#[tokio::main]
async fn main(args: Args) -> anyhow::Result<()> {
    env_logger::init();
    let token = env::var("TG_BOT_TOKEN")?;

    let cats = (&*PARSED_CATS).as_ref().unwrap();
    let api = Api::new(token);
    let mut stream = api.stream();

    log::info!("Accepting message...");
    while let Some(msg) = stream.next().await {
        let msg = msg?;
        log::debug!("{:#?}", msg);
        match msg.kind {
            UpdateKind::InlineQuery(query) => {
                let pfut = process(api.clone(), query, cats, args.color, args.null);
                let fut = async {
                    if let Err(e) = pfut.await {
                        log::error!("Failure: {}", e);
                    }
                };
                tokio::spawn(fut);
            },
            _ => { }
        }
    }

    Ok(())
}