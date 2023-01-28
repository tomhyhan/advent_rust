fn look_and_say(text: &mut String) -> String {
    let mut current_text = text.as_bytes();
    let mut new_string = String::from("");
    // println!("{current_text:?}");

    let mut idx = 0;
    while idx < current_text.len() {
        let mut count = 1;
        while idx + count < current_text.len() && current_text[idx] == current_text[idx + count] {
            count += 1
        }

        new_string.push(char::from_digit(count as u32, 10).unwrap());
        new_string.push(current_text[idx] as char);
        idx += count;
    }
    new_string
}

fn main() {
    let mut text = "1321131112".to_string();

    for _ in 0..50 {
        let new_text = look_and_say(&mut text);
        println!("{:?}", new_text.len());
        text = new_text
    }
}
