use tokio::time::{Duration, sleep} ;

#[tokio::main]
async fn main() {
    // первая асинхронная задача
    let op1 = async {
        sleep(Duration::from_secs(1)).await ;
        "Результат 1"
    } ;

    // вторая асинхронная задача
    let op2 = async {
        sleep(Duration::from_secs(2)).await ;
        42
    } ;

    // Ожидает завершения нескольких одновременных ветвей и возвращает управление, 
    // когда все они завершатся.
    let (res1, res2) = tokio::join!(
        op1,
        op2
    ) ;

    println!("res1 = {}, res2 = {}", res1, res2) ;
}
