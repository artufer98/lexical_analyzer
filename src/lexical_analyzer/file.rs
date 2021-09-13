use std::fs;
use std::process;


fn get_file_name(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        return Err("La ruta del archivo no fue proporcionada");
    }
    Ok(&args[1])
}


fn get_file_content(filename: String) -> Result<String, String> {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|err| {
            return err.to_string();
        });

    Ok(contents)
}


pub struct File {
    pub name: String,
    pub content: String,
}


impl File {
    pub fn new(args: Vec<String>) -> File {
        let name: &str = get_file_name(&args)
            .unwrap_or_else(|e| {
                eprintln!("Problemas al obtener la ruta del archivo\n{}", e);
                process::exit(1);
            });

        let content: String = get_file_content(name.to_owned())
            .unwrap_or_else(|e| {
                eprintln!("Problemas al obtener el contenido del archivo\n{}", e);
                process::exit(1);
            });

        File {
            name: name.to_owned(),
            content: content.to_owned(),
        }
    }
}
