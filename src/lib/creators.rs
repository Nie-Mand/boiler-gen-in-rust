use std::env::set_current_dir;
use std::fs::{create_dir_all, read_to_string, write};
use std::process::Command;

pub fn create_express_boilerplate(project_name: &str) {
    init_git();
    init_node_project();
    make_folder_structure(&[
        "app",
        "app/models",
        "app/services",
        "app/routes",
        "app/controllers",
        "app/utils",
    ]);
    add_npm_scripts(&[
        "\t\t\"dev\": \"nodemon --exec babel-node index.js\",",
        "\t\t\"run\": \"babel-node index.js\"",
    ]);
    add_express_index_file();
    add_prettier_config();
    add_env_file();
    add_babel_config();
    add_basic_readme(extract_project_name(project_name));
    install_global_dependencies_npm(&["nodemon"]);
    install_dev_dependencies_npm(&["@babel/core", "@babel/node", "@babel/preset-env"]);
    install_dependencies_npm(&[
        "body-parser",
        "cors",
        "dotenv",
        "express",
        "helmet",
        "lodash",
        "xss-clean",
    ]);
    add_routes_index_file();
    keep_folders(&[
        "app/controllers",
        "app/routes",
        "app/services",
        "app/models",
        "app/utils",
    ]);
    add_global_utils_express();
}

pub fn create_react_boilerplate() {
    println!("Create a react boilerplate");
}

pub fn create_react_ts_boilerplate() {
    println!("Create a react as ts boilerplate");
}

pub fn place_me_at(location: &str) {
    if location != "." {
        create_dir_all(location).ok();
        set_current_dir(location).ok();
    }
}

fn init_node_project() {
    Command::new("npm").arg("init").arg("-y").output().ok();
    println!("+ Initiated a Node Project");
}

fn add_npm_scripts(scripts: &[&str]) {
    // Read from the file
    let mut content = read_to_string("package.json").unwrap();

    // Prepare the Content
    let mut lines: Vec<&str> = content.split('\n').collect();
    lines.remove(6);
    let mut idx = 6;
    for script in scripts {
        lines.insert(idx, script);
        idx += 1;
    }
    content = lines.join("\n");

    // Write the content back
    write("package.json", content.as_bytes()).unwrap();
    println!("+ Added The Necessary Scripts to package.json");
}

fn add_prettier_config() {
    let content =
        "{\n\t\"semi\": false,\n\t\"arrowParens\": \"avoid\",\n\t\"singleQuote\": true\n}";
    write(".prettierrc", content.as_bytes()).unwrap();
    println!("+ Added Prettier Config");
}

fn init_git() {
    Command::new("git").arg("init").output().ok();
    write(".gitignore", "node_modules".as_bytes()).unwrap();
    println!("+ Initiated a git repository");
}

fn add_env_file() {
    let content = "PORT=8000";
    write(".env", content.as_bytes()).unwrap();
    println!("+ Added .ENV File");
}

fn add_babel_config() {
    let content = "{ \"presets\": [\"@babel/preset-env\"] }";
    write(".babelrc", content.as_bytes()).unwrap();
    println!("+ Added Babel Config");
}

fn add_basic_readme(project_name: &str) {
    let content = format!("# {}\nHello World", project_name.to_uppercase());
    write("README.md", content.as_bytes()).unwrap();
    println!("+ Added Basic README");
}

fn add_express_index_file() {
    let content = "import dotenv from 'dotenv'\ndotenv.config()\n\nimport Express from 'express'\nimport routes from './routes'\nimport cors from 'cors'\nimport { json } from 'body-parser'\nimport xss from 'xss-clean'\nimport helmet from 'helmet'\n\nconst app = Express()\n\n// Middlewares\napp.use(cors())\napp.use(json())\napp.use(xss())\napp.use(helmet())\n\n// Routes\napp.use(routes)\napp.get('/', (req, res) => res.send('Hello, World!'))\n\napp.listen(process.env.PORT || 8000, () =>\n  console.log('Server is running at http://localhost:8000')\n)";
    write("app/index.js", content.as_bytes()).unwrap();
    println!("+ Generated index.js");
}

fn add_routes_index_file() {
    let content =
        "import { Router } from 'express'\nconst router = Router()\nexport default router";
    write("app/routes/index.js", content.as_bytes()).unwrap();
}

fn install_dev_dependencies_npm(dependencies: &[&str]) {
    Command::new("npm")
        .arg("i")
        .arg("-D")
        .args(dependencies)
        .spawn()
        .ok();
    println!("+ Installed the DEV Dependencies");
}

fn install_dependencies_npm(dependencies: &[&str]) {
    Command::new("npm").arg("i").args(dependencies).spawn().ok();
    println!("+ Installed the Dependencies");
}

fn install_global_dependencies_npm(dependencies: &[&str]) {
    Command::new("npm")
        .arg("i")
        .arg("-g")
        .args(dependencies)
        .spawn()
        .ok();
    println!("+ Installed the Global Dependencies");
}

fn keep_folders(folders: &[&str]) {
    for folder in folders {
        write(format!("{}/.gitkeep", folder), "".as_bytes()).unwrap();
    }
}

fn add_global_utils_express() {
    let content =
    "import { intersection } from 'lodash'\n\n\nexport const safeResponse = controller => (req, res) =>\n  controller(req, res).catch(e => res.status(500).send(e.message))\n\n\nexport const removeFields = (obj, fields) => {\n  fields.split(' ').forEach(field => {\n    delete obj[field]\n  })\n}\n\n\nexport const doesInclude = (obj, keys) =>\n  intersection(Object.keys(obj), keys.split(' ')).length != 0\n\n\nexport const selectFields = (obj, fields) => {\n  Object.keys(obj).forEach(key => {\n    if (fields.split(' ').indexOf(key) == -1) delete obj[key]\n  })\n}";
    write("app/utils/global.utils.js", content.as_bytes()).unwrap();
}
// Helper Functions

fn make_folder_structure(folders: &[&str]) {
    for folder in folders {
        create_dir_all(folder).ok();
    }
    println!("+ Created the Folders Structure");
}

fn extract_project_name(project_name: &str) -> &str {
    let lines: Vec<&str> = project_name.split('/').collect();
    return lines.last().unwrap();
}
