use std::{
    fs::File,
    io::{self, Read, Write},
};

use tera::Tera;

use super::resume::{Resume, read_resume_from_input};

pub fn render_resume(
    input_resume_path: String,
    output_resume_path: String,
) -> Result<(), io::Error> {
    let mut input_fp = File::open(input_resume_path)?;
    let mut output_fp = File::create(output_resume_path)?;
    render_resume_with_io(&mut input_fp, &mut output_fp)?;
    Ok(())
}

fn render_resume_with_io<R: Read, W: Write>(
    input_fp: &mut R,
    output_fp: &mut W,
) -> Result<(), io::Error> {
    let resume = read_resume_from_input(input_fp).unwrap();
    render_resume_to_output(&resume, output_fp).unwrap();
    Ok(())
}

fn render_resume_to_output<W: Write>(resume: &Resume, output: W) -> Result<(), io::Error> {
    let tera = Tera::new("src/templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("resume", &resume);
    tera.render_to("resume.tmpl.tex", &context, output).unwrap();
    Ok(())
}
