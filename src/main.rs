fn print_element(elements: &[String]) {
    // traditional for loop, but we need to have &Vec<String> as a return type
    /*
    for element in elements {
        println!("element: {:?}", element);
    }
    */

    // for_each is an iterator consumer, it's automatically called next()
    elements
        .iter()
        .map(|ele| format!("{} {}", ele, ele)) // iterator adapter, it's doesn't call next()
        .for_each(|ele| println!("element: {:?}", ele));
}

fn shorten_strings(elements: &mut [String]) {
    // for element in elements {
    elements.iter_mut().for_each(|ele| ele.truncate(1)); // iterator adapter, it's doesn't call next()
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    // for element in elements {
    // elements.iter().map(|ele| ele.to_uppercase()).collect()

    // another way to do it
    elements
        .iter()
        .map(|ele| ele.to_uppercase())
        .collect::<Vec<_>>() // _ mean let the Rust compiler infer the type
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|ele| vec_b.push(ele));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|ele| ele.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search)) // find is an iterator consumer, called next() automatically
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_element(&colors); // or we can pass &colors[0..2] as an argument

    // shorten_strings(&mut colors);
    // println!("colors: {:#?}", colors);

    // let uppercased = to_uppercase(&colors);
    // println!("uppercased: {:#?}", uppercased);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);

    // println!("destination: {:#?}", destination);

    // let exploded = explode(&colors);
    // println!("exploded: {:#?}", exploded);

    let found_color = find_color_or(&colors, "red", "Orange");
    println!("found_color: {:#?}", found_color);
}
