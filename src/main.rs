use structopt::StructOpt;
use colored::Colorize;
use std::fmt;

use inquire::{
    formatter::MultiOptionFormatter,
    MultiSelect,
    Select
};


#[derive(StructOpt, Debug)]
#[structopt(name="my_tech_stack")]
struct MyTechStackArguments {
  #[structopt(subcommand)]
  pub command: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
  #[structopt(name = "init")]
  Init(InitNewApplication)
}

#[derive(StructOpt, Debug)]
pub struct InitNewApplication {
}

#[derive(PartialEq)]
enum AppType {
    FullStackApp, 
    StaticWebApp, 
    ApiOnlyApp
}

impl fmt::Display for AppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppType::FullStackApp => write!(f, "Full Stack App"), 
            AppType::StaticWebApp => write!(f, "Static Web App"), 
            AppType::ApiOnlyApp   => write!(f, "Api Only App"), 
        }
    }
}

#[derive(Debug)]
enum BackendTech {
    Hasura, 
    FastApi
}

impl fmt::Display for BackendTech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendTech::Hasura => write!(f, "Hasura"), 
            BackendTech::FastApi => write!(f, "FastApi")
        }
    }
}

#[derive(Debug)]
enum FrontendTech {
    Elm
}
impl fmt::Display for FrontendTech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrontendTech::Elm => write!(f, "Elm")
        }
    }
}

fn parse_fullstack_choice(fs_choice: AppType) -> () {

    let backend_types: Vec<BackendTech> = vec![BackendTech::Hasura, BackendTech::FastApi ]; 
    let backend_type_ = Select::new("What type of bakcend framework would you like to use?", backend_types);
    
    let frontend_types: Vec<FrontendTech> = vec![FrontendTech::Elm]; 
    let frontend_type_ = Select::new("What type of frontend framework would you like to use?", frontend_types);

    let elm_options = vec!["elm/graphql", "elm/ts-interop", "page routing", "tailwindcss"];
    let formatter: MultiOptionFormatter<&str> = &|a| format!("Selected {} different options", a.len());

    let backend_feature_options = vec!["stripe", "auth"]; 
    let backend_features_ = MultiSelect::new("What would you like in you backend setup?", backend_feature_options)
                                        .with_formatter(formatter); 


    match fs_choice  {
        AppType::FullStackApp => {

            let backend_features_list = backend_features_.prompt().unwrap(); 
            let backend_type_ans = backend_type_.prompt().unwrap(); 
            match backend_type_ans {
                BackendTech::Hasura => {
                    let frontend_type_ans = frontend_type_.prompt().unwrap(); 
                    match frontend_type_ans {
                        FrontendTech::Elm => {
                            let ans = MultiSelect::new("What would you like in you Elm setup?", elm_options)
                                        .with_formatter(formatter)
                                        .prompt()
                                        .unwrap();
                            println!("{:?}", ans); 
                        }
                    }
                println!("{}", format!("Hasura is supported").green()); 

                },  
                BackendTech::FastApi => {
                    println!("{} {}", format!("FastApi").red(), format!("is not supported yet").red()); 
                }
            };

                    }, 
        AppType::StaticWebApp => {
            println!("{}", format!("Static Web App generation not supported yet!").red()); 
        }, 
        AppType::ApiOnlyApp => {
            let backend_features_list = backend_features_.prompt(); 
            println!("{}", format!("Static Web App generation not supported yet!").red()); 
        }
    };
}

fn initialize_new_application(_new_app: &InitNewApplication) -> () {
    let app_types: Vec<AppType> = vec![AppType::FullStackApp, AppType::StaticWebApp, AppType::ApiOnlyApp]; 
    let app_type_ans = Select::new("What type of application are you tryping to build?", app_types).prompt(); 
    parse_fullstack_choice(app_type_ans.unwrap()); 
    
}

fn main() {
  let args = MyTechStackArguments::from_args(); 
  //print!("{:#?}", args); 
  match args.command {
    Command::Init(new_app) => {
      initialize_new_application(&new_app); 
    
    }
  }

}
