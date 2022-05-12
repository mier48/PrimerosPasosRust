fn main() {
    let algo = Option::Some(3);

    let pointA = Point { x: 0.5, y: 12.41 };
    let pointB = Point { x: 12.52, y: 12.41 };

    calcular_area(pointA, pointB);
}

struct Point<T> {
    x: T,
    y: T,
}

fn calcular_area<T>(pointa: Point<T>, pointb: Point<T>) {
    //
}
