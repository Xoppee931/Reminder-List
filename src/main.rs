mod db;
mod models;

use db::connect_db;
use models::{create_task, get_tasks};
use inquire::Text;
use sqlx::PgPool;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_db().await?;
    loop {
        let mut input = String::new();
        
        println!("\nEscolha uma das opções abaixo:
1) Visualizar tarefas existentes
2) Criar nova tarefa
0) Sair do programa
");
        io::stdin()
        .read_line(&mut input)
        .expect("Escolha uma das opções");
    
        match input.trim() {
            "1" => show_tasks(&pool).await,
            "2" => createnew_task(&pool).await,
            "0" => {println!("Saindo do programa..."); break;}
            _ => println!("Opção inválida"),
        }
    }
    Ok(())
}


async fn show_tasks(pool: &PgPool) {
    match get_tasks(&pool).await {
        Ok(tasks) => {
            println!("\n--- Lista de Tarefas ---\n");
            if tasks.is_empty() {
                println!("Nenhuma tarefa encontrada");
            } else {
                for task in &tasks {
                    let desc = if task.description.trim().is_empty() {
                        "Sem descrição"
                    } else {
                        &task.description
                    };
                    println!("[{}] {} - {}", task.id, task.name, desc);
                }
            }
        }   
        Err(e) => println!("Erro ao buscar tarefas {}", e),
    }
}

async fn createnew_task(pool: &PgPool) {
    let name = Text::new("Name of the task: ")
        .with_placeholder("Ex: Study Rust")
        .prompt();

    let description = Text::new("Description of the task: ")
        .with_placeholder("Ex: Write a program to do a connection with an database")
        .prompt();

    match (name, description) {
        (Ok(name_val), Ok(desc_val)) => {
            let desc_option = if desc_val.trim().is_empty() {
                None
            } else {
                Some(desc_val)
            };

            match create_task(pool, &name_val, desc_option.as_deref()).await {
                Ok(_task) => {println!("Task criada com sucesso!"); show_tasks(pool).await;},
                Err(e) => println!("Ocorreu um erro durante a execução: {}", e),
            }
        }
        _ => println!("Operação cancelada ou erro na leitura da entrada"),
    }
}