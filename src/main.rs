use std::path::PathBuf; 
use structopt::StructOpt;
use std::fs; 


#[derive(StructOpt, Debug)]
#[structopt(name="my_tech_stack")]
struct MyTechStackArguments {
  #[structopt(subcommand)]
  pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
  #[structopt(name = "init")]
  Init(InitNewApplication), 
  #[structopt(name = "generate_feature")]
  NewFeature(NewFeatureArguments) 
}

#[derive(StructOpt, Debug)]
pub struct NewFeatureArguments {
  #[structopt(short, long)]
  name: String 
}

#[derive(StructOpt, Debug)]
pub struct InitNewApplication {
  #[structopt(short, long, parse(from_os_str))]
  app_location: PathBuf, 
  #[structopt(long)]
  use_graphql: bool, 
  #[structopt(long)]
  generate_elm_routes: bool 
}

fn initialize_new_application(new_app: &InitNewApplication) -> () {
  println!("Will generate a new FastAPI project with Pipenv at {:?}", &new_app.app_location);
      let result = fs::create_dir(&new_app.app_location); 

      match result {
        Ok(r) => {
          println!("{:?}", r);
          fs::create_file(&new_app.app_location + "main.py")
        },
        Err(_) => {
          println!("Could not create dir {:?}", &new_app.app_location);} 
      }
}

fn main() {
  let args = MyTechStackArguments::from_args(); 
  print!("{:#?}", args); 
  match args.command {
    Command::Init(new_app) => {
      initialize_new_application(&new_app); 
    
    }, 
    Command::NewFeature(_new_feature) => {
      println!("Will create new fastapi feature"); 
    }
  }
  // std::
  // if args.use_graphql {
  //   println!("Will generate a graphql setup..."); 
  // }
  // println!("Will generate Postgresql [test, dev] database"); 
  // println!("Will generate an Elm project with tailwindcss");
  // if opt.generate_elm_routes {
  //   println!("Will generate elm route")
  // }
}
