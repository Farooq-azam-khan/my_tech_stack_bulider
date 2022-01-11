# my_tech_stack_bulider
* a cli app for ease of web development
* automatically implements boilerplate code that can easily be integrated to the project


## Features 
* [ ] `mts init` will clone the zip of this `https://github.com/Farooq-azam-khan/my_tech_stack_sample.git` down and unzip it
* [ ] `mts api create {feature_name}` will create boilerplate for a new feature. Up to you to integrate into project
    * [ ] create `api/src/{feature_name}`
    * [ ] create `api/src/{feature_name}/[models.py, schemas.py, routes.py]`
* [ ] `mts ui create {page_name}` will create an elm page with custom routes you can specify 
   * [ ] create `ui/src/{page_name}`
   * [ ] create `ui/src/{page_name}/[{page_name}_View.elm, {page_name}_Components.elm]`
* [ ] `mts ui init` create a boilerplate of the ui only
    * [ ] this command will have tailwind, elm compile to js, elm route parsing
    * [ ] it will be an opt out model (`mts ui init --remove routes`, `mts ui init --remove tailwind`)\
