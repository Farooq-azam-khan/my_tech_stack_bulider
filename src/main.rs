use std::path::PathBuf; 
use structopt::StructOpt;
use std::fs; 
use std::path::Path; 
use std::io::Write;

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

fn create_mainpy_file(src_path: &Path) -> () {
  let main_path = src_path.join(Path::new("main.py")); 
  let mut main_file = fs::File::create(main_path).unwrap(); 
  main_file.write_all(b"from fastapi import FastAPI\nfrom pydantic import BaseModel\nfrom fastapi.middleware.cors import CORSMiddleware\n\napp = FastAPI()\nclass User(BaseModel):\n\tusername: str\n\temail: str\n\tpassword: str\norigins = ['http://localhost',\n\t'http://localhost:8080'\n]\n\napp.add_middleware(CORSMiddleware,\n\tallow_origins=origins,\n\tallow_credentials=True,allow_methods=['*'],allow_headers=['*']\n)\n
\n@app.get('/')\ndef main():\n\treturn 'hello world'").unwrap();
}

fn create_mainelm_file(ui_path: &Path) -> () {

  let main_elm_file_path = ui_path.join(Path::new("main.elm")); 
  let mut elm_file = fs::File::create(&main_elm_file_path).unwrap(); 
  elm_file.write_all(b"module Main where").unwrap();
}

fn initialize_new_application(new_app: &InitNewApplication) -> () {
  println!("Will generate a new FastAPI project at {:?}", &new_app.app_location);
      let result = fs::create_dir(&new_app.app_location);

      match result {
        Ok(r) => {
          println!("{:?}", r);
          let src_path = new_app.app_location.as_path().join(Path::new("src")); 
          let _src_foler = fs::create_dir(&src_path);
          create_mainpy_file(&src_path); 
          let ui_path = src_path.join(Path::new("UI")); 
          let _ui_dir = fs::create_dir(&ui_path).unwrap(); 
          create_mainelm_file(&ui_path); 
          // main_file.write_all().unwrap();

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

}
