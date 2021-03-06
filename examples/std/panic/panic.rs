// Реализуем свою версию целочисленного деления (/)
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Деление на ноль вызывает панику
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// Основной поток `main`
fn main() {
    // Целочисленное значение, выделенное в куче
    let _x = Box::new(0i32);

    // Это операция вызовет панику в основном потоке
    division(3, 0);

    println!("Эта часть кода не будет достигнута");

    // `_x` должен быть уничтожен в этой точке
}
