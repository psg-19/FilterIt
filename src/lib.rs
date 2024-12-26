use clap::{Parser, ValueEnum};
use colored::Colorize;
use image::{io::Reader as ImageReader, DynamicImage};
use std::{path::PathBuf, process};
mod photo_filters;
mod utils;
use crate::photo_filters::img_blur::apply_blur;
use crate::photo_filters::img_grayscale::apply_grayscale;
use crate::photo_filters::img_negative::apply_negative;
use crate::photo_filters::img_reflection::apply_reflection;
use crate::photo_filters::img_sepia::apply_sepia_filter;
use crate::photo_filters::img_edge::apply_edge;
use crate::utils::image_util::save_image;
// use FilterIt::utils;

const DEFAULT_IMAGE_OUTPUT_FILENAME: &str = "output.png";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'i', long, required = true)]
    src: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long, value_enum, required = true)]
    filter: Filters,
}
