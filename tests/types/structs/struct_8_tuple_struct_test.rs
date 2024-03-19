struct RGB(f64, f64, f64);
struct Vec3(f64, f64, f64);

// Print the RGB values out
fn print_color(color: &RGB) {
    println!("R {}, G {}, B {}", color.0, color.1, color.2);
}

// Print the vector3's XYZ values out
fn print_vector_three(vector3: &Vec3) {
    println!("X {}, Y {}, Z {}", vector3.0, vector3.1, vector3.2);
}

#[test]
fn tuple_struct_test() {
    let tomato = RGB(1.0, 0.38824, 0.27843);
    let start_position = Vec3(1.74531, 6.221, 3.132234);

    print_color(&tomato);
    print_vector_three(&start_position);

    assert_eq!(tomato.0, 1.0, "instance returns");
    assert_eq!(tomato.1, 0.38824, "instance returns");
    assert_eq!(tomato.2, 0.27843, "instance returns");

    assert_eq!(start_position.0, 1.74531, "instance returns");
    assert_eq!(start_position.1, 6.221, "instance returns");
    assert_eq!(start_position.2, 3.132234, "instance returns");
}
