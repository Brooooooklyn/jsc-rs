use clap::{self, Parser};
use jsc::{ClassAttribute, ClassDefinition, Context, PropertyAttributes};
use tokio::fs;

mod console;

#[derive(Parser)]
#[clap(author, version, about = "tyr", long_about = "Tyr JavaScript runtime")]
#[clap(propagate_version = true)]
struct Tyr {
  /// JavaScript file to run
  entry: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
  let global_class = ClassDefinition::default()
    .with_name("globalThis")?
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()?;
  let ctx = Context::with_global_class(global_class)?;
  let mut global = ctx.global();
  let mut console = ClassDefinition::default()
    .with_name("Console")?
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()?
    .make_object(&ctx);
  let log = ctx.create_function("log", Some(console::console_log))?;
  console
    .set_property("log", &log, PropertyAttributes::None)
    .unwrap();
  global
    .set_property("console", &console, PropertyAttributes::DontDelete)
    .unwrap();
  let tyr = Tyr::parse();
  let script = fs::read_to_string(tyr.entry).await?;
  ctx.eval(script)?;
  Ok(())
}
