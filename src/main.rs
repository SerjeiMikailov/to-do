use std::process::Command;
use std::io::{self, Write};
use std::fs::File;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        let mut args = command.splitn(2, ' ');
        let command_name = args.next().unwrap_or("");
        let command_arg = args.next().unwrap_or("");

        match command_name {
            "create" => {
                if command_arg.starts_with("list items") {
                    let list_name = command_arg.trim_start_matches("list items").trim();
                    create_list(list_name, "items");
                } else if command_arg.starts_with("list remember") {
                    let list_name = command_arg.trim_start_matches("list remember").trim();
                    create_list(list_name, "remember");
                } else {
                    println!("Comando inválido. Use 'create list items \"nome da lista\"' ou 'create remember \"nome da lista\"' para criar uma lista.");
                }
            }
            _ => {
                let output = Command::new("wt.exe")
                    .arg("-p")
                    .arg("Windows PowerShell")
                    .arg("-d")
                    .arg("C:/Caminho/Pasta")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("Falha ao executar o comando");

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("Saída do comando: {}", stdout);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Erro ao executar o comando: {}", stderr);
                }
            }
        }
    }
}

fn create_list(list_name: &str, list_type: &str) {
    let file_name = format!("{}.txt", list_name);
    let content = match list_type {
        "items" => {
            format!("
            Lista de Itens:

            - Item 1
            - Item 2
            - Item 3
            ")
        }
        "remember" => {
            format!("
            Lista de Lembretes:

            - Digite aqui: -
            ")
        }
        _ => {
            println!("Tipo de lista inválido.");
            return;
        }
    };

    let mut file = match File::create(&file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Erro ao criar o arquivo: {}", err);
            return;
        }
    };

    if let Err(err) = file.write_all(content.as_bytes()) {
        eprintln!("Erro ao escrever no arquivo: {}", err);
        return;
    }

    println!("Lista criada com sucesso: {}", file_name);
}
