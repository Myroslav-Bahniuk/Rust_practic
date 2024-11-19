const WIDTH: usize = 30; // Ширина конверта
const HEIGHT: usize = 15; // Высота конверта

fn main() {
    // Проходим по всем строкам
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {                
                print!("*");
            } 
            else if x == 0 || x == WIDTH - 1 {        
                print!("*");
            } 
            else if x == y || x == WIDTH - y - 1 {               
                print!("*");
            } 
            else {
                print!(" ");
            }
        }
        // Переход на новую строку после каждой строки
        println!();
    }
}
